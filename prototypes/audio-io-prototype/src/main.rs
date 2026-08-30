// Real-time FFT-based YIN pitch detection on live microphone input
// AcapellaStudio Phase 3 Feasibility Study
// Run on Michael's actual reference hardware (AMD Ryzen 7 8840HS, Kubuntu, PipeWire 1.6.2).
//
// KNOWN INEFFICIENCY (flagged, not fixed yet): creates a fresh FftPlanner
// every callback instead of caching one - identified as the likely cause of
// the real speedup (1.79x) falling short of the sandbox-measured speedup
// (3.79x). Next step is to fix this and re-measure.

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rustfft::{FftPlanner, num_complex::Complex};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

const RING_SIZE: usize = 2048;
const YIN_THRESHOLD: f64 = 0.10;
const TEST_DURATION_SECS: u64 = 20;
const EXPECTED_BUFFER_MS: f64 = 128.0 / 48000.0 * 1000.0;
const GAP_WARNING_MULTIPLIER: f64 = 2.0;

fn fft_difference_function(buffer: &[f64], max_tau: usize, planner: &mut FftPlanner<f64>) -> Vec<f64> {
    let n = buffer.len();
    let w = n - max_tau;

    let a: f64 = buffer[0..w].iter().map(|x| x * x).sum();

    let mut prefix_sq = vec![0.0; n + 1];
    for i in 0..n {
        prefix_sq[i + 1] = prefix_sq[i] + buffer[i] * buffer[i];
    }

    let fft_len = (n + w).next_power_of_two();
    let fft = planner.plan_fft_forward(fft_len);
    let ifft = planner.plan_fft_inverse(fft_len);

    let mut sig_full: Vec<Complex<f64>> = buffer.iter().map(|&x| Complex::new(x, 0.0)).collect();
    sig_full.resize(fft_len, Complex::new(0.0, 0.0));

    let mut kernel: Vec<Complex<f64>> = buffer[0..w].iter().rev().map(|&x| Complex::new(x, 0.0)).collect();
    kernel.resize(fft_len, Complex::new(0.0, 0.0));

    fft.process(&mut sig_full);
    fft.process(&mut kernel);

    let mut product: Vec<Complex<f64>> = sig_full.iter().zip(kernel.iter())
        .map(|(a, b)| a * b)
        .collect();

    ifft.process(&mut product);
    let norm = fft_len as f64;

    let mut c = vec![0.0; max_tau];
    for tau in 0..max_tau {
        let idx = w - 1 + tau;
        c[tau] = product[idx].re / norm;
    }

    let mut d = vec![0.0; max_tau];
    for tau in 1..max_tau {
        let b_tau = prefix_sq[tau + w] - prefix_sq[tau];
        d[tau] = a + b_tau - 2.0 * c[tau];
    }
    d
}

fn cmndf(d: &[f64]) -> Vec<f64> {
    let mut d_prime = vec![1.0; d.len()];
    let mut running_sum = 0.0;
    for tau in 1..d.len() {
        running_sum += d[tau];
        d_prime[tau] = d[tau] / (running_sum / tau as f64);
    }
    d_prime
}

fn absolute_threshold(d_prime: &[f64], threshold: f64) -> Option<usize> {
    let mut tau = 2;
    while tau < d_prime.len() {
        if d_prime[tau] < threshold {
            while tau + 1 < d_prime.len() && d_prime[tau + 1] < d_prime[tau] {
                tau += 1;
            }
            return Some(tau);
        }
        tau += 1;
    }
    None
}

fn parabolic_interpolation(d_prime: &[f64], tau_estimate: usize) -> f64 {
    if tau_estimate == 0 || tau_estimate >= d_prime.len() - 1 {
        return tau_estimate as f64;
    }
    let x0 = tau_estimate - 1;
    let x2 = tau_estimate + 1;
    let s0 = d_prime[x0];
    let s1 = d_prime[tau_estimate];
    let s2 = d_prime[x2];
    let denom = 2.0 * (2.0 * s1 - s2 - s0);
    if denom.abs() < 1e-12 {
        return tau_estimate as f64;
    }
    tau_estimate as f64 + (s2 - s0) / (2.0 * denom)
}

fn yin_detect_fft(buffer: &[f64], max_tau: usize, sample_rate: f64, threshold: f64, planner: &mut FftPlanner<f64>) -> Option<f64> {
    let d = fft_difference_function(buffer, max_tau, planner);
    let d_prime = cmndf(&d);
    let tau = absolute_threshold(&d_prime, threshold)?;
    let refined_tau = parabolic_interpolation(&d_prime, tau);
    if refined_tau <= 0.0 {
        return None;
    }
    Some(sample_rate / refined_tau)
}

fn main() {
    let host = cpal::default_host();
    let input_device = host.default_input_device().expect("no input device");
    let mut input_config: cpal::StreamConfig = input_device.default_input_config().unwrap().into();
    input_config.buffer_size = cpal::BufferSize::Fixed(128);
    let sample_rate = input_config.sample_rate as f64;
    let channels = input_config.channels as usize;
    let max_tau = RING_SIZE / 2;

    println!("Running {}s real-time FFT-based YIN test on LIVE MICROPHONE INPUT.", TEST_DURATION_SECS);
    println!("Sing or hum something! Detected pitch will print periodically.\n");
    println!("(Wrap this binary in `/usr/bin/time -v` for real CPU/memory numbers)\n");

    let ring: Arc<Mutex<Vec<f64>>> = Arc::new(Mutex::new(vec![0.0; RING_SIZE]));
    let callback_count = Arc::new(AtomicU64::new(0));
    let large_gap_count = Arc::new(AtomicU64::new(0));
    let last_callback_time: Arc<Mutex<Option<Instant>>> = Arc::new(Mutex::new(None));
    let gap_samples: Arc<Mutex<Vec<f64>>> = Arc::new(Mutex::new(Vec::with_capacity(10000)));
    let last_detected: Arc<Mutex<Option<f64>>> = Arc::new(Mutex::new(None));
    let detect_count = Arc::new(AtomicU64::new(0));

    let ring_cb = ring.clone();
    let cc = callback_count.clone();
    let lgc = large_gap_count.clone();
    let lct = last_callback_time.clone();
    let gs = gap_samples.clone();
    let ld = last_detected.clone();
    let dc = detect_count.clone();

    let stream = input_device.build_input_stream(
        input_config.clone(),
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let mut planner = FftPlanner::<f64>::new();

            let callback_start = Instant::now();

            cc.fetch_add(1, Ordering::Relaxed);
            {
                let mut last = lct.lock().unwrap();
                if let Some(prev) = *last {
                    let gap_ms = callback_start.duration_since(prev).as_secs_f64() * 1000.0;
                    gs.lock().unwrap().push(gap_ms);
                    if gap_ms > EXPECTED_BUFFER_MS * GAP_WARNING_MULTIPLIER {
                        lgc.fetch_add(1, Ordering::Relaxed);
                    }
                }
                *last = Some(callback_start);
            }

            let mut buf = ring_cb.lock().unwrap();
            let n_new = data.len() / channels;
            let shift = n_new.min(RING_SIZE);
            buf.copy_within(shift.., 0);
            let start_idx = RING_SIZE - shift;
            for (i, frame) in data.chunks(channels).enumerate().take(shift) {
                let mono: f32 = frame.iter().sum::<f32>() / channels as f32;
                buf[start_idx + i] = mono as f64;
            }

            if let Some(freq) = yin_detect_fft(&buf, max_tau, sample_rate, YIN_THRESHOLD, &mut planner) {
                if freq > 60.0 && freq < 1200.0 {
                    *ld.lock().unwrap() = Some(freq);
                    dc.fetch_add(1, Ordering::Relaxed);
                }
            }
        },
        move |err| eprintln!("Input stream error: {}", err),
        None,
    ).expect("failed to build input stream");

    stream.play().expect("failed to start input stream");

    let start = Instant::now();
    while start.elapsed().as_secs() < TEST_DURATION_SECS {
        std::thread::sleep(Duration::from_millis(500));
        if let Some(freq) = *last_detected.lock().unwrap() {
            println!("  detected: {:.1} Hz", freq);
        } else {
            println!("  (no pitch detected yet)");
        }
    }

    drop(stream);

    let total_callbacks = callback_count.load(Ordering::Relaxed);
    let large_gaps = large_gap_count.load(Ordering::Relaxed);
    let detections = detect_count.load(Ordering::Relaxed);
    let gaps = gap_samples.lock().unwrap();

    let mean: f64 = gaps.iter().sum::<f64>() / gaps.len() as f64;
    let max: f64 = gaps.iter().cloned().fold(f64::MIN, f64::max);
    let stddev = (gaps.iter().map(|g| (g - mean).powi(2)).sum::<f64>() / gaps.len() as f64).sqrt();

    println!("\n=== RESULTS (FFT-based YIN) ===");
    println!("Total callbacks: {}", total_callbacks);
    println!("Callbacks with a plausible pitch detected: {} ({:.1}%)", detections, detections as f64 / total_callbacks as f64 * 100.0);
    println!("Measured mean interval: {:.3}ms (expected {:.3}ms)", mean, EXPECTED_BUFFER_MS);
    println!("Measured max interval:  {:.3}ms", max);
    println!("Measured std deviation: {:.3}ms", stddev);
    println!("Large gaps (>{}x expected): {} out of {} ({:.3}%)",
        GAP_WARNING_MULTIPLIER, large_gaps, total_callbacks, large_gaps as f64 / total_callbacks as f64 * 100.0);
}
