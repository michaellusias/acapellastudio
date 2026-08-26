// YIN pitch detection prototype — AcapellaStudio Phase 3 Feasibility Study
//
// Implements the YIN algorithm as described in:
// de Cheveigné, A., & Kawahara, H. (2002). "YIN, a fundamental frequency
// estimator for speech and music." JASA, 111(4), 1917-1930.
//
// This prototype tests against SYNTHETIC sine waves at known frequencies,
// NOT real singing voice. Real singing-voice testing requires a microphone
// and recordings, which this sandboxed environment does not have. Synthetic
// testing establishes whether the core algorithm is implemented correctly
// and gives a real (not fabricated) best-case accuracy baseline before
// real-world (noisy, breathy, vibrato) singing-voice testing.

const SAMPLE_RATE: f64 = 48000.0;
const THRESHOLD: f64 = 0.10; // standard YIN absolute threshold from the paper

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

fn generate_sine(freq: f64, sample_rate: f64, n_samples: usize) -> Vec<f64> {
    (0..n_samples)
        .map(|i| (2.0 * std::f64::consts::PI * freq * (i as f64) / sample_rate).sin())
        .collect()
}

fn generate_sine_with_noise(freq: f64, sample_rate: f64, n_samples: usize, noise_amplitude: f64) -> Vec<f64> {
    let mut state: u64 = 88172645463325252;
    let mut next_rand = move || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        ((state as f64) / (u64::MAX as f64)) * 2.0 - 1.0
    };
    (0..n_samples)
        .map(|i| {
            let clean = (2.0 * std::f64::consts::PI * freq * (i as f64) / sample_rate).sin();
            clean + noise_amplitude * next_rand()
        })
        .collect()
}

fn cents_error(true_freq: f64, detected_freq: f64) -> f64 {
    1200.0 * (detected_freq / true_freq).log2()
}

fn main() {
    println!("=== AcapellaStudio Phase 3 Feasibility Study ===");
    println!("YIN Pitch Detection Prototype — SYNTHETIC signal testing only\n");
    println!("Sample rate: {} Hz, threshold: {}\n", SAMPLE_RATE, THRESHOLD);

    let test_frequencies = [
        ("E2 (low bass)", 82.41),
        ("A2", 110.00),
        ("A3", 220.00),
        ("A4 (concert pitch)", 440.00),
        ("A5", 880.00),
        ("C6 (high soprano)", 1046.50),
    ];

    let buffer_size = 2048;

    println!("--- Test 1: Clean sine waves (no noise) ---");
    println!("{:<20} {:>12} {:>14} {:>12}", "Note", "True (Hz)", "Detected (Hz)", "Error (cents)");
    let mut clean_errors = Vec::new();
    for (name, freq) in test_frequencies.iter() {
        let buffer = generate_sine(*freq, SAMPLE_RATE, buffer_size);
        match yin_detect(&buffer, SAMPLE_RATE, THRESHOLD) {
            Some(detected) => {
                let err = cents_error(*freq, detected);
                clean_errors.push(err.abs());
                println!("{:<20} {:>12.2} {:>14.2} {:>12.2}", name, freq, detected, err);
            }
            None => {
                println!("{:<20} {:>12.2} {:>14} {:>12}", name, freq, "NO DETECT", "N/A");
            }
        }
    }
    let mean_clean_error: f64 = clean_errors.iter().sum::<f64>() / clean_errors.len() as f64;
    println!("\nMean absolute error (clean): {:.3} cents\n", mean_clean_error);

    println!("--- Test 2: Sine waves with additive noise (amplitude 0.05) ---");
    println!("{:<20} {:>12} {:>14} {:>12}", "Note", "True (Hz)", "Detected (Hz)", "Error (cents)");
    let mut noisy_errors = Vec::new();
    let mut noisy_failures = 0;
    for (name, freq) in test_frequencies.iter() {
        let buffer = generate_sine_with_noise(*freq, SAMPLE_RATE, buffer_size, 0.05);
        match yin_detect(&buffer, SAMPLE_RATE, THRESHOLD) {
            Some(detected) => {
                let err = cents_error(*freq, detected);
                noisy_errors.push(err.abs());
                println!("{:<20} {:>12.2} {:>14.2} {:>12.2}", name, freq, detected, err);
            }
            None => {
                noisy_failures += 1;
                println!("{:<20} {:>12.2} {:>14} {:>12}", name, freq, "NO DETECT", "N/A");
            }
        }
    }
    if !noisy_errors.is_empty() {
        let mean_noisy_error: f64 = noisy_errors.iter().sum::<f64>() / noisy_errors.len() as f64;
        println!("\nMean absolute error (noisy): {:.3} cents", mean_noisy_error);
    }
    println!("Detection failures (noisy): {}/{}\n", noisy_failures, test_frequencies.len());

    println!("--- Test 3: Processing time per buffer (2048 samples, A4=440Hz) ---");
    let buffer = generate_sine(440.0, SAMPLE_RATE, buffer_size);
    let n_iterations = 1000;
    let start = std::time::Instant::now();
    for _ in 0..n_iterations {
        let _ = yin_detect(&buffer, SAMPLE_RATE, THRESHOLD);
    }
    let elapsed = start.elapsed();
    let per_call_micros = elapsed.as_micros() as f64 / n_iterations as f64;
    println!("Average time per YIN call: {:.2} microseconds ({:.4} ms)", per_call_micros, per_call_micros / 1000.0);
    let buffer_duration_ms = (buffer_size as f64 / SAMPLE_RATE) * 1000.0;
    println!("Buffer duration at this sample rate: {:.3} ms", buffer_duration_ms);
    println!("Processing time as % of buffer duration: {:.4}%", (per_call_micros / 1000.0) / buffer_duration_ms * 100.0);

    println!("\n=== NOTES ===");
    println!("- This is a naive O(n * max_tau) unoptimized implementation, not production code.");
    println!("- Tested on SYNTHETIC signals only. Real singing voice has vibrato, breathiness,");
    println!("  formants, and noise characteristics not captured here.");
    println!("- CPU timing measured on THIS CONTAINER's CPU, NOT the reference hardware");
    println!("  (Michael's HP EliteBook running Kubuntu) — must be re-measured there.");
}
