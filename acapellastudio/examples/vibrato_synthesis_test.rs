//! Synthetic vibrato test — real, controlled ground-truth validation.
//!
//! Generates a sine wave with a KNOWN, programmed vibrato rate and depth
//! (no live-singing inconsistency, no real-world microphone noise), then
//! runs the real YinFftDetector over it and checks whether the detected
//! frequency sequence and this tool's own oscillation-rate estimator
//! (from pitch_benchmark.rs) correctly recover the known ground truth.
//!
//! This directly resolves the ambiguity from the live vibrato benchmark
//! runs (docs/09_dsp_design.md §2.3): those runs couldn't distinguish
//! between "the singer didn't hold a steady note" and "YIN has a real
//! octave-jumping/tracking problem." This test isolates the algorithm
//! from the performance by using a signal with known, controlled ground
//! truth - no microphone or live singing involved.

use acapellastudio::dsp::pitch::{frequency_to_midi_note, YinFftDetector};

const SAMPLE_RATE: f64 = 48000.0;
const RING_SIZE: usize = 2048;
const STEP: usize = 128; // matches the real-time buffer size used throughout this project

/// Generates a sine wave with sinusoidal frequency modulation (vibrato) at
/// a known rate and depth, around a known base frequency.
fn generate_vibrato_tone(
    base_freq: f64,
    vibrato_rate_hz: f64,
    depth_cents: f64,
    duration_secs: f64,
    sample_rate: f64,
) -> Vec<f64> {
    let n_samples = (duration_secs * sample_rate) as usize;
    let mut phase = 0.0f64;
    let mut samples = Vec::with_capacity(n_samples);

    for i in 0..n_samples {
        let t = i as f64 / sample_rate;
        let cents_offset = depth_cents * (2.0 * std::f64::consts::PI * vibrato_rate_hz * t).sin();
        let instantaneous_freq = base_freq * 2f64.powf(cents_offset / 1200.0);
        phase += 2.0 * std::f64::consts::PI * instantaneous_freq / sample_rate;
        samples.push(phase.sin());
    }
    samples
}

fn analyze_oscillation(frequencies: &[f64], window_step_seconds: f64) -> (f64, f64, f64) {
    let mean_freq: f64 = frequencies.iter().sum::<f64>() / frequencies.len() as f64;
    let cents_deviations: Vec<f64> = frequencies
        .iter()
        .map(|&f| 1200.0 * (f / mean_freq).log2())
        .collect();
    let variance: f64 =
        cents_deviations.iter().map(|d| d * d).sum::<f64>() / cents_deviations.len() as f64;
    let std_dev_cents = variance.sqrt();

    let mut crossings = 0;
    for i in 1..frequencies.len() {
        let prev_above = frequencies[i - 1] > mean_freq;
        let curr_above = frequencies[i] > mean_freq;
        if prev_above != curr_above {
            crossings += 1;
        }
    }
    let total_time = frequencies.len() as f64 * window_step_seconds;
    let estimated_rate_hz = (crossings as f64 / 2.0) / total_time;

    (mean_freq, std_dev_cents, estimated_rate_hz)
}

fn run_test(base_freq: f64, vibrato_rate_hz: f64, depth_cents: f64) {
    println!(
        "=== Synthetic vibrato test: base={}Hz, rate={}Hz, depth={}cents ===",
        base_freq, vibrato_rate_hz, depth_cents
    );

    let duration = 10.0;
    let tone = generate_vibrato_tone(base_freq, vibrato_rate_hz, depth_cents, duration, SAMPLE_RATE);

    let mut detector = YinFftDetector::new();
    let mut frequencies = Vec::new();
    let mut detections = 0u32;
    let mut total_windows = 0u32;
    let mut octave_errors = 0u32; // real check: detected freq far from expected octave

    let mut i = 0;
    while i + RING_SIZE <= tone.len() {
        let window = &tone[i..i + RING_SIZE];
        total_windows += 1;
        if let Some(result) = detector.detect_with_confidence(window, SAMPLE_RATE) {
            detections += 1;
            frequencies.push(result.frequency);

            // Real check: is this detection roughly the right octave? A
            // genuine octave error would show the detected frequency near
            // 0.5x or 2x the expected base, not just noisy around it.
            let ratio = result.frequency / base_freq;
            if !(0.7..1.43).contains(&ratio) {
                octave_errors += 1;
            }
        }
        i += STEP;
    }

    let detection_rate = detections as f64 / total_windows as f64 * 100.0;
    println!("Detection rate: {}/{} ({:.1}%)", detections, total_windows, detection_rate);
    println!("Likely octave errors (freq outside 0.7x-1.43x base): {} ({:.1}%)",
        octave_errors, octave_errors as f64 / detections.max(1) as f64 * 100.0);

    if !frequencies.is_empty() {
        let step_seconds = STEP as f64 / SAMPLE_RATE;
        let (mean_freq, std_dev_cents, estimated_rate) = analyze_oscillation(&frequencies, step_seconds);
        let expected_midi = frequency_to_midi_note(base_freq);
        let detected_midi = frequency_to_midi_note(mean_freq);

        println!("Expected base frequency: {:.1}Hz (MIDI {:.2})", base_freq, expected_midi);
        println!("Mean detected frequency: {:.1}Hz (MIDI {:.2})", mean_freq, detected_midi);
        println!("Expected vibrato depth: {:.1} cents | Detected std dev: {:.1} cents", depth_cents, std_dev_cents);
        println!("Expected vibrato rate: {:.2}Hz | Estimated rate: {:.2}Hz", vibrato_rate_hz, estimated_rate);

        let rate_error_pct = ((estimated_rate - vibrato_rate_hz) / vibrato_rate_hz * 100.0).abs();
        let freq_error_cents = 1200.0 * (mean_freq / base_freq).log2();
        println!("\nRate recovery error: {:.1}%", rate_error_pct);
        println!("Mean frequency error: {:.2} cents", freq_error_cents);

        if octave_errors > detections / 10 {
            println!("\n*** REAL FINDING: >10% likely octave errors on a CLEAN synthetic signal ***");
            println!("This would indicate a genuine YIN tracking problem, not a live-singing artifact.");
        } else if rate_error_pct < 20.0 {
            println!("\nRate recovered within 20% of ground truth on clean synthetic input.");
            println!("This suggests the earlier live-mic ambiguity was more likely a performance/");
            println!("recording issue than a fundamental YIN vibrato-tracking problem - though this");
            println!("does NOT rule out real-world factors (noise, formants, breathiness) still");
            println!("degrading tracking on an actual voice.");
        } else {
            println!("\nRate NOT well recovered even on a clean synthetic signal - this points toward");
            println!("a real algorithmic limitation in tracking this rate/depth of modulation, not");
            println!("just a live-singing performance issue.");
        }
    }
    println!();
}

fn main() {
    // Real, known ground-truth test cases, spanning typical vocal vibrato
    // parameters (rate ~3-9Hz per common references, depth ~50-100 cents)
    // plus one deliberately larger-depth case to see how depth affects
    // tracking.
    run_test(440.0, 5.5, 80.0);   // typical vibrato: A4, 5.5Hz, 80 cents
    run_test(220.0, 6.0, 60.0);   // lower voice, faster/shallower
    run_test(440.0, 5.5, 250.0);  // same rate, MUCH larger depth (comparable
                                    // to the ~285 cents seen in the live tests)
    run_test(330.0, 4.0, 100.0);  // slower vibrato, moderate depth
}
