// CPU/dropout endurance test — AcapellaStudio Phase 3 Feasibility Study
// Run on Michael's actual reference hardware (AMD Ryzen 7 8840HS, Kubuntu, PipeWire 1.6.2).
//
// HONEST METHOD NOTE: this does NOT read PipeWire's own xrun/underrun counter
// (cpal doesn't expose that portably). Instead it measures the wall-clock
// time between consecutive input callbacks. At a 128-sample/48kHz buffer,
// callbacks should arrive roughly every 2.667ms. A callback that arrives
// much later than expected suggests something delayed the audio thread -
// a reasonable proxy for real-time deadline problems, not a direct
// measurement of an actual buffer underrun/overrun. CPU and memory usage
// should be measured by wrapping this binary in `/usr/bin/time -v`, not
// self-reported from inside the program.

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

const TEST_DURATION_SECS: u64 = 30;
const EXPECTED_BUFFER_MS: f64 = 128.0 / 48000.0 * 1000.0;
const GAP_WARNING_MULTIPLIER: f64 = 2.0;

fn main() {
    let host = cpal::default_host();
    let input_device = host.default_input_device().expect("no input device");
    let output_device = host.default_output_device().expect("no output device");

    let mut input_config: cpal::StreamConfig = input_device.default_input_config().unwrap().into();
    let mut output_config: cpal::StreamConfig = output_device.default_output_config().unwrap().into();
    input_config.buffer_size = cpal::BufferSize::Fixed(128);
    output_config.buffer_size = cpal::BufferSize::Fixed(128);

    println!("Running {}s endurance test at 128-sample buffer (expected ~{:.3}ms/callback)", TEST_DURATION_SECS, EXPECTED_BUFFER_MS);
    println!("(This process should be wrapped in `/usr/bin/time -v` for real CPU/memory numbers)\n");

    let callback_count = Arc::new(AtomicU64::new(0));
    let large_gap_count = Arc::new(AtomicU64::new(0));
    let last_callback_time: Arc<Mutex<Option<Instant>>> = Arc::new(Mutex::new(None));
    let gap_samples: Arc<Mutex<Vec<f64>>> = Arc::new(Mutex::new(Vec::with_capacity(20000)));

    let cc = callback_count.clone();
    let lgc = large_gap_count.clone();
    let lct_in = last_callback_time.clone();
    let gs_in = gap_samples.clone();

    let output_stream = output_device.build_output_stream(
        output_config.clone(),
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for s in data.iter_mut() { *s = 0.0; }
        },
        move |err| eprintln!("Output stream error: {}", err),
        None,
    ).expect("failed to build output stream");

    let input_stream = input_device.build_input_stream(
        input_config.clone(),
        move |_data: &[f32], _: &cpal::InputCallbackInfo| {
            cc.fetch_add(1, Ordering::Relaxed);
            let now = Instant::now();
            let mut last = lct_in.lock().unwrap();
            if let Some(prev) = *last {
                let gap_ms = now.duration_since(prev).as_secs_f64() * 1000.0;
                gs_in.lock().unwrap().push(gap_ms);
                if gap_ms > EXPECTED_BUFFER_MS * GAP_WARNING_MULTIPLIER {
                    lgc.fetch_add(1, Ordering::Relaxed);
                }
            }
            *last = Some(now);
        },
        move |err| eprintln!("Input stream error: {}", err),
        None,
    ).expect("failed to build input stream");

    output_stream.play().expect("failed to start output stream");
    input_stream.play().expect("failed to start input stream");

    std::thread::sleep(Duration::from_secs(TEST_DURATION_SECS));

    drop(output_stream);
    drop(input_stream);

    let total_callbacks = callback_count.load(Ordering::Relaxed);
    let large_gaps = large_gap_count.load(Ordering::Relaxed);
    let gaps = gap_samples.lock().unwrap();

    if gaps.is_empty() {
        println!("No gap data collected - something went wrong.");
        return;
    }

    let mean: f64 = gaps.iter().sum::<f64>() / gaps.len() as f64;
    let max: f64 = gaps.iter().cloned().fold(f64::MIN, f64::max);
    let min: f64 = gaps.iter().cloned().fold(f64::MAX, f64::min);
    let variance: f64 = gaps.iter().map(|g| (g - mean).powi(2)).sum::<f64>() / gaps.len() as f64;
    let stddev = variance.sqrt();

    println!("=== RESULTS ===");
    println!("Total callbacks: {}", total_callbacks);
    println!("Expected callback interval: {:.3}ms", EXPECTED_BUFFER_MS);
    println!("Measured mean interval:     {:.3}ms", mean);
    println!("Measured min interval:      {:.3}ms", min);
    println!("Measured max interval:      {:.3}ms", max);
    println!("Measured std deviation:     {:.3}ms", stddev);
    println!("Callbacks with gap > {}x expected ({:.3}ms): {} out of {} ({:.3}%)",
        GAP_WARNING_MULTIPLIER, EXPECTED_BUFFER_MS * GAP_WARNING_MULTIPLIER,
        large_gaps, total_callbacks, (large_gaps as f64 / total_callbacks as f64) * 100.0);

    println!("\nNOTE: 'large gap' events are a proxy for real-time deadline problems,");
    println!("based on callback timing alone - not a direct read of PipeWire's own");
    println!("xrun/underrun counter, which cpal does not expose portably.");
}
