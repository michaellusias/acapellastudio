//! Pitch detection benchmark harness — roadmap Phase 12 deliverable support.
//!
//! Records a labeled test condition (e.g. "vibrato", "quiet_singing",
//! "background_noise") for a fixed duration, runs the real YinFftDetector
//! on a sliding window over the captured audio, and logs measurable
//! results (frequency, confidence, MIDI note per detection) to a CSV file,
//! plus prints summary statistics. This is real infrastructure for
//! gathering the roadmap's required test data:
//!
//!   "Test: Different singers, Different registers, Vibrato, Quiet singing,
//!    Loud singing, Breathiness, Background noise. Record measurable
//!    results."
//!
//! This tool does NOT run the tests itself — it cannot sing or generate
//! real vocal conditions. It exists so that Michael can run it once per
//! real condition, on real hardware, with real singing, and get real,
//! logged, comparable results instead of informal by-ear impressions
//! (which the Feasibility Study already flagged as a real limitation,
//! §2.12-2.13).
//!
//! REAL BUG FOUND AND FIXED (first real run of this tool, on actual stereo
//! input hardware): the original version used drain_available() directly,
//! which returns raw interleaved multi-channel samples. On stereo input,
//! this corrupted the pitch analysis (feeding alternating L/R samples to
//! a mono pitch detector) and doubled the apparent recording duration in
//! the printed summary (1453824 samples / 48000 = "30.29s" for a requested
//! 15s recording). Fixed by using the new drain_available_mono() method
//! on RecordingHandle, which correctly averages/de-interleaves channels.
//!
//! Usage:
//!   cargo run --release --example pitch_benchmark -- <label> <duration_seconds>
//!
//! Example:
//!   cargo run --release --example pitch_benchmark -- vibrato_test 15

use acapellastudio::audio::AudioEngine;
use acapellastudio::dsp::pitch::YinFftDetector;
use std::fs::File;
use std::io::Write;
use std::time::{Duration, Instant};

const RING_SIZE: usize = 2048;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <label> <duration_seconds>", args[0]);
        eprintln!("Example: {} vibrato_test 15", args[0]);
        std::process::exit(1);
    }
    let label = &args[1];
    let duration_secs: u64 = args[2].parse().expect("duration must be a whole number of seconds");

    println!("=== Pitch Detection Benchmark: '{}' ===", label);
    println!("Recording for {}s, then analyzing...", duration_secs);
    println!("Sing/hum the target condition now.\n");

    let engine = AudioEngine::new()
        .expect("failed to initialize audio engine - is a microphone connected?")
        .with_fixed_buffer_size(128);
    let sample_rate = engine.sample_rate() as f64;

    let mut recording_handle = engine
        .start_recording(sample_rate as usize * (duration_secs as usize + 2))
        .expect("failed to start recording");

    let start = Instant::now();
    let mut captured: Vec<f32> = Vec::new();
    while start.elapsed() < Duration::from_secs(duration_secs) {
        std::thread::sleep(Duration::from_millis(100));
        captured.extend(recording_handle.drain_available_mono());
    }
    // Final drain to catch anything still in the buffer
    std::thread::sleep(Duration::from_millis(100));
    captured.extend(recording_handle.drain_available_mono());

    println!("Captured {} samples ({:.2}s of audio)", captured.len(), captured.len() as f64 / sample_rate);

    if captured.is_empty() {
        eprintln!("No audio captured - check microphone input. Aborting analysis.");
        std::process::exit(1);
    }

    // Analyze via sliding window, real YinFftDetector, matching the same
    // 2048-sample window used throughout the Feasibility Study.
    let captured_f64: Vec<f64> = captured.iter().map(|&s| s as f64).collect();
    let mut detector = YinFftDetector::new();

    let filename = format!("benchmark_{}.csv", label);
    let mut file = File::create(&filename).expect("failed to create output CSV");
    writeln!(file, "sample_offset,time_seconds,frequency_hz,confidence,midi_note,detected").unwrap();

    let mut detections = 0u32;
    let mut total_windows = 0u32;
    let mut confidences = Vec::new();
    let mut frequencies = Vec::new();

    let step = 128; // matches the real-time buffer size used throughout this project
    let mut i = 0;
    while i + RING_SIZE <= captured_f64.len() {
        let window = &captured_f64[i..i + RING_SIZE];
        total_windows += 1;
        let time_seconds = i as f64 / sample_rate;

        match detector.detect_with_confidence(window, sample_rate) {
            Some(result) if result.frequency > 60.0 && result.frequency < 1200.0 => {
                detections += 1;
                confidences.push(result.confidence);
                frequencies.push(result.frequency);
                writeln!(
                    file,
                    "{},{:.4},{:.2},{:.4},{:.2},true",
                    i, time_seconds, result.frequency, result.confidence, result.midi_note
                ).unwrap();
            }
            _ => {
                writeln!(file, "{},{:.4},,,,false", i, time_seconds).unwrap();
            }
        }
        i += step;
    }

    println!("\n=== RESULTS ===");
    println!("Total analysis windows: {}", total_windows);
    println!("Windows with a plausible detection: {} ({:.1}%)", detections, detections as f64 / total_windows as f64 * 100.0);

    if !confidences.is_empty() {
        let mean_confidence: f64 = confidences.iter().sum::<f64>() / confidences.len() as f64;
        let min_freq = frequencies.iter().cloned().fold(f64::MAX, f64::min);
        let max_freq = frequencies.iter().cloned().fold(f64::MIN, f64::max);
        println!("Mean confidence (on successful detections): {:.4}", mean_confidence);
        println!("Frequency range detected: {:.1}Hz - {:.1}Hz", min_freq, max_freq);
    } else {
        println!("No successful detections - condition may be too difficult for current YIN implementation,");
        println!("or no audio signal was present. This is itself a real, useful result to record.");
    }

    println!("\nFull per-window results written to: {}", filename);
    println!("Record this file's summary in docs/03_feasibility_study.md or a new benchmark log,");
    println!("per roadmap Phase 12's 'Record measurable results' requirement.");
}
