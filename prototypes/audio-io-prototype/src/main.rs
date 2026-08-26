// Round-trip acoustic loopback latency test — AcapellaStudio Phase 3 Feasibility Study
// Run on Michael's actual reference hardware (AMD Ryzen 7 8840HS, Kubuntu, PipeWire 1.6.2).
//
// HONEST METHOD NOTE: this measures ACOUSTIC round-trip latency (output ->
// headphone driver -> air -> microphone -> input), not pure software/OS
// audio-path latency. It includes real physical propagation delay and
// transducer response time. It uses simple amplitude-threshold click
// detection, not cross-correlation - a rough but real and honest estimate,
// not a lab-grade measurement.

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

const CLICK_AMPLITUDE: f32 = 0.9;
const CLICK_DURATION_SAMPLES: usize = 4800; // 100ms burst at 48kHz, loud and long enough to detect reliably
const DETECT_THRESHOLD: f32 = 0.02; // conservative, since acoustic path attenuates a lot

fn main() {
    let host = cpal::default_host();

    let input_device = host.default_input_device().expect("no input device");
    let output_device = host.default_output_device().expect("no output device");

    let mut input_config: cpal::StreamConfig = input_device.default_input_config().unwrap().into();
    let mut output_config: cpal::StreamConfig = output_device.default_output_config().unwrap().into();
    input_config.buffer_size = cpal::BufferSize::Fixed(128);
    output_config.buffer_size = cpal::BufferSize::Fixed(128);

    println!("Input config: {:?}", input_config);
    println!("Output config: {:?}", output_config);

    let start = Instant::now();

    // 0 = not yet happened. Stored as nanoseconds since `start`.
    let play_time_ns = Arc::new(AtomicU64::new(0));
    let detect_time_ns = Arc::new(AtomicU64::new(0));

    let play_time_ns_out = play_time_ns.clone();
    let mut click_samples_remaining = 0usize;
    let mut click_started = false;
    let warmup_callbacks = Arc::new(AtomicU64::new(0));
    let warmup_callbacks_out = warmup_callbacks.clone();

    let out_channels = output_config.channels as usize;

    let output_stream = output_device.build_output_stream(
        output_config.clone(),
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let n_callbacks = warmup_callbacks_out.fetch_add(1, Ordering::Relaxed);
            // Wait ~1 second of callbacks before firing the click, so streams
            // are stable first. At 128 samples/48kHz that's roughly 375 callbacks.
            if n_callbacks < 375 {
                for s in data.iter_mut() { *s = 0.0; }
                return;
            }
            if !click_started {
                click_started = true;
                click_samples_remaining = CLICK_DURATION_SAMPLES;
                play_time_ns_out.store(start.elapsed().as_nanos() as u64, Ordering::Relaxed);
            }
            for frame in data.chunks_mut(out_channels) {
                let sample = if click_samples_remaining > 0 {
                    click_samples_remaining -= 1;
                    CLICK_AMPLITUDE
                } else {
                    0.0
                };
                for s in frame.iter_mut() { *s = sample; }
            }
        },
        move |err| eprintln!("Output stream error: {}", err),
        None,
    ).expect("failed to build output stream");

    let play_time_ns_in = play_time_ns.clone();
    let detect_time_ns_in = detect_time_ns.clone();
    let in_channels = input_config.channels as usize;

    let input_stream = input_device.build_input_stream(
        input_config.clone(),
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            if detect_time_ns_in.load(Ordering::Relaxed) != 0 {
                return; // already detected, ignore further callbacks
            }
            let played_at = play_time_ns_in.load(Ordering::Relaxed);
            if played_at == 0 {
                return; // click hasn't been played yet
            }
            for frame in data.chunks(in_channels) {
                if frame.iter().any(|s| s.abs() > DETECT_THRESHOLD) {
                    let now_ns = start.elapsed().as_nanos() as u64;
                    detect_time_ns_in.store(now_ns, Ordering::Relaxed);
                    break;
                }
            }
        },
        move |err| eprintln!("Input stream error: {}", err),
        None,
    ).expect("failed to build input stream");

    output_stream.play().expect("failed to start output stream");
    input_stream.play().expect("failed to start input stream");

    println!("Warming up (~1s), then playing a 100ms click through your speakers.");

    // Wait up to 5 seconds total for detection.
    std::thread::sleep(Duration::from_secs(6));

    let played = play_time_ns.load(Ordering::Relaxed);
    let detected = detect_time_ns.load(Ordering::Relaxed);

    if played == 0 {
        println!("\nClick was never played - something went wrong with the output stream timing.");
    } else if detected == 0 {
        println!("\nClick was played at {:.3}ms but was NOT detected on the input within the test window.", played as f64 / 1_000_000.0);
        println!("Possible causes: speaker volume too low, detection threshold too high, or genuinely no signal reaching the mic.");
    } else {
        let latency_ns = detected.saturating_sub(played);
        let latency_ms = latency_ns as f64 / 1_000_000.0;
        println!("\n=== RESULT ===");
        println!("Click played at:   {:.3}ms (since stream start)", played as f64 / 1_000_000.0);
        println!("Click detected at: {:.3}ms (since stream start)", detected as f64 / 1_000_000.0);
        println!("Estimated round-trip acoustic latency: {:.2}ms", latency_ms);
        println!("\nNOTE: this includes real acoustic propagation + transducer delay,");
        println!("not just software/OS audio-path latency. Treat as a rough real-world");
        println!("estimate, not a precise software-only measurement.");
    }
}
