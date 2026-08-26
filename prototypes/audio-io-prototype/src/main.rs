// Audio I/O device enumeration + live stream callback timing prototype
// AcapellaStudio Phase 3 Feasibility Study
// Run on Michael's actual reference hardware (AMD Ryzen 7 8840HS, Kubuntu, PipeWire 1.6.2).

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

fn main() {
    let host = cpal::default_host();
    let device = host.default_input_device().expect("no input device");
    let config = device.default_input_config().expect("no default input config");
    println!("Using config: {:?}", config);

    let callback_count = Arc::new(AtomicU64::new(0));
    let sample_count = Arc::new(AtomicU64::new(0));
    let cc = callback_count.clone();
    let sc = sample_count.clone();

    let stream_config: cpal::StreamConfig = config.clone().into();
    let channels = stream_config.channels as u64;

    let stream = device.build_input_stream(
        stream_config.clone(),
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            cc.fetch_add(1, Ordering::Relaxed);
            sc.fetch_add((data.len() as u64) / channels, Ordering::Relaxed);
        },
        move |err| eprintln!("Stream error: {}", err),
        None,
    ).expect("failed to build input stream");

    stream.play().expect("failed to start stream");
    println!("Recording for 5 seconds... (say something into the mic if you like)");

    let start = Instant::now();
    std::thread::sleep(Duration::from_secs(5));
    let elapsed = start.elapsed();

    let calls = callback_count.load(Ordering::Relaxed);
    let samples = sample_count.load(Ordering::Relaxed);
    println!("\nElapsed: {:.6}s", elapsed.as_secs_f64());
    println!("Total callbacks: {}", calls);
    println!("Total sample-frames captured: {}", samples);
    println!("Average samples per callback: {:.1}", samples as f64 / calls as f64);
    println!("Implied sample rate: {:.1} Hz (config says {})", samples as f64 / elapsed.as_secs_f64(), stream_config.sample_rate);
}
