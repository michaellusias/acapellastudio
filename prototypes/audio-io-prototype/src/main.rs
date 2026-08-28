// Controlled reference-pitch accuracy test — AcapellaStudio Phase 3 Feasibility Study
// Run on Michael's actual reference hardware (AMD Ryzen 7 8840HS, Kubuntu, PipeWire 1.6.2).
//
// METHOD: play a KNOWN reference tone (440.0 Hz, concert A4) through the
// speaker, run the real YIN detector (same algorithm as before, sliding
// 2048-sample ring buffer) on the real microphone picking it up, and
// compare detected frequency against the known 440.0 Hz ground truth.
// This tests the full real chain (DAC -> speaker -> air -> mic -> ADC ->
// real-time YIN) without relying on human vocal pitch accuracy.
//
// Every callback's raw result (including "no detection" and out-of-range
// values) is logged, fixing the "sticky printout" issue from the previous
// test - this gives a true, honest detection rate, not a misleading one.

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

const RING_SIZE: usize = 2048;
const YIN_THRESHOLD: f64 = 0.10;
const TEST_DURATION_SECS: u64 = 10;
const REFERENCE_FREQ: f64 = 440.0; // concert A4, known ground truth

fn difference_function(buffer: &[f64], max_tau: usize) -> Vec<f64> {
    let mut d = vec![0.0; max_tau];
    for tau in 1..max_tau {
        let mut sum = 0.0;
        for j in 0..(buffer.len() - max_tau) {
            let diff = buffer[j] - buffer[j + tau];
            sum += diff * diff;
        }
        d[tau] = sum;
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

fn yin_detect(buffer: &[f64], sample_rate: f64, threshold: f64) -> Option<f64> {
    let max_tau = buffer.len() / 2;
    let d = difference_function(buffer, max_tau);
    let d_prime = cmndf(&d);
    let tau = absolute_threshold(&d_prime, threshold)?;
    let refined_tau = parabolic_interpolation(&d_prime, tau);
    if refined_tau <= 0.0 {
        return None;
    }
    Some(sample_rate / refined_tau)
}

fn cents_error(true_freq: f64, detected_freq: f64) -> f64 {
    1200.0 * (detected_freq / true_freq).log2()
}

fn main() {
    let host = cpal::default_host();
    let input_device = host.default_input_device().expect("no input device");
    let output_device = host.default_output_device().expect("no output device");

    let mut input_config: cpal::StreamConfig = input_device.default_input_config().unwrap().into();
    let mut output_config: cpal::StreamConfig = output_device.default_output_config().unwrap().into();
    input_config.buffer_size = cpal::BufferSize::Fixed(128);
    output_config.buffer_size = cpal::BufferSize::Fixed(128);
    let sample_rate = input_config.sample_rate as f64;
    let in_channels = input_config.channels as usize;
    let out_channels = output_config.channels as usize;

    println!("Playing a KNOWN {:.1} Hz reference tone through your speakers for {}s.", REFERENCE_FREQ, TEST_DURATION_SECS);
    println!("Real-time YIN will detect it via the real microphone. No singing needed.\n");

    // Output: continuous sine wave at REFERENCE_FREQ
    let mut phase: f64 = 0.0;
    let out_sample_rate = output_config.sample_rate as f64;
    let output_stream = output_device.build_output_stream(
        output_config.clone(),
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for frame in data.chunks_mut(out_channels) {
                let sample = (0.5 * (2.0 * std::f64::consts::PI * phase).sin()) as f32;
                for s in frame.iter_mut() { *s = sample; }
                phase += REFERENCE_FREQ / out_sample_rate;
                if phase >= 1.0 { phase -= 1.0; }
            }
        },
        move |err| eprintln!("Output stream error: {}", err),
        None,
    ).expect("failed to build output stream");

    // Input: real YIN detection, logging EVERY callback's raw result
    let ring: Arc<Mutex<Vec<f64>>> = Arc::new(Mutex::new(vec![0.0; RING_SIZE]));
    let results: Arc<Mutex<Vec<Option<f64>>>> = Arc::new(Mutex::new(Vec::with_capacity(5000)));
    let ring_cb = ring.clone();
    let results_cb = results.clone();

    let input_stream = input_device.build_input_stream(
        input_config.clone(),
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let mut buf = ring_cb.lock().unwrap();
            let n_new = data.len() / in_channels;
            let shift = n_new.min(RING_SIZE);
            buf.copy_within(shift.., 0);
            let start_idx = RING_SIZE - shift;
            for (i, frame) in data.chunks(in_channels).enumerate().take(shift) {
                let mono: f32 = frame.iter().sum::<f32>() / in_channels as f32;
                buf[start_idx + i] = mono as f64;
            }
            let result = yin_detect(&buf, sample_rate, YIN_THRESHOLD);
            results_cb.lock().unwrap().push(result);
        },
        move |err| eprintln!("Input stream error: {}", err),
        None,
    ).expect("failed to build input stream");

    output_stream.play().expect("failed to start output stream");
    input_stream.play().expect("failed to start input stream");

    std::thread::sleep(Duration::from_secs(TEST_DURATION_SECS));

    drop(output_stream);
    drop(input_stream);

    let all_results = results.lock().unwrap();
    let total = all_results.len();
    let no_detection = all_results.iter().filter(|r| r.is_none()).count();
    let detected: Vec<f64> = all_results.iter().filter_map(|r| *r).collect();

    println!("=== RESULTS ===");
    println!("Total callbacks: {}", total);
    println!("No detection (YIN returned None): {} ({:.1}%)", no_detection, no_detection as f64 / total as f64 * 100.0);
    println!("Detected something: {} ({:.1}%)", detected.len(), detected.len() as f64 / total as f64 * 100.0);

    if !detected.is_empty() {
        let mean: f64 = detected.iter().sum::<f64>() / detected.len() as f64;
        let min = detected.iter().cloned().fold(f64::MAX, f64::min);
        let max = detected.iter().cloned().fold(f64::MIN, f64::max);
        let cents_errors: Vec<f64> = detected.iter().map(|f| cents_error(REFERENCE_FREQ, *f)).collect();
        let mean_cents_error: f64 = cents_errors.iter().sum::<f64>() / cents_errors.len() as f64;
        let mean_abs_cents_error: f64 = cents_errors.iter().map(|e| e.abs()).sum::<f64>() / cents_errors.len() as f64;

        // Count how many detections are "close" (within 50 cents = half a semitone) to the reference
        let close_count = detected.iter().filter(|f| cents_error(REFERENCE_FREQ, **f).abs() < 50.0).count();

        println!("\nReference frequency: {:.2} Hz", REFERENCE_FREQ);
        println!("Detected mean:       {:.2} Hz", mean);
        println!("Detected min/max:    {:.2} Hz / {:.2} Hz", min, max);
        println!("Mean cents error (signed): {:.2} cents", mean_cents_error);
        println!("Mean absolute cents error: {:.2} cents", mean_abs_cents_error);
        println!("Detections within 50 cents of reference: {} / {} ({:.1}%)", close_count, detected.len(), close_count as f64 / detected.len() as f64 * 100.0);
    } else {
        println!("\nNo detections at all - something is wrong with the setup (volume, mic gain, etc).");
    }
}
