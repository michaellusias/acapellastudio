// FFT-based fast YIN implementation - AcapellaStudio Phase 3 Feasibility Study
//
// Goal: reduce YIN's difference-function computation from O(n * max_tau) to
// O(n log n) using FFT-based autocorrelation, while producing results
// IDENTICAL (within floating-point tolerance) to the naive implementation
// already validated against a real reference tone on Michael's hardware.
//
// Confirmed real-world precedent: aubio's "yinfast" method is documented as
// "yielding results identical to the original YIN algorithm, while reducing
// its computational cost from O(n^2) to O(n log(n))" (source: aubio manpages,
// checked in this session). This is distinct from "yinfft" (a different,
// non-identical tapered/spectral variant per Brossier's 2006 PhD thesis).
// We are implementing the identical-results variant, not yinfft.
//
// MATH (standard DSP, not novel):
// d(tau) = sum_{j=0}^{W-1} (x[j] - x[j+tau])^2
//        = A + B(tau) - 2*C(tau)
// where:
//   A       = sum_{j=0}^{W-1} x[j]^2                (constant, O(1) after one pass)
//   B(tau)  = sum_{j=0}^{W-1} x[j+tau]^2             (windowed sum of squares,
//                                                      O(1) per tau via prefix sums)
//   C(tau)  = sum_{j=0}^{W-1} x[j] * x[j+tau]        (cross-correlation - the
//                                                      expensive term, computed
//                                                      for ALL tau at once via FFT)
//
// This is NOT claimed to be faster or correct until actually measured and
// verified below, against the same naive implementation used elsewhere in
// this project.

use rustfft::{FftPlanner, num_complex::Complex};

const THRESHOLD: f64 = 0.10;

// ===== Naive implementation (ground truth, copied unchanged from the
// already-validated prototype) =====

fn naive_difference_function(buffer: &[f64], max_tau: usize) -> Vec<f64> {
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

// ===== FFT-based fast implementation =====

fn fft_difference_function(buffer: &[f64], max_tau: usize) -> Vec<f64> {
    let n = buffer.len();
    let w = n - max_tau; // integration window length, matches naive's convention

    // A: constant term
    let a: f64 = buffer[0..w].iter().map(|x| x * x).sum();

    // B(tau): prefix sums of squares for O(1) windowed sum lookup
    let mut prefix_sq = vec![0.0; n + 1];
    for i in 0..n {
        prefix_sq[i + 1] = prefix_sq[i] + buffer[i] * buffer[i];
    }
    // B(tau) = sum_{j=tau}^{tau+w-1} x[j]^2 = prefix_sq[tau+w] - prefix_sq[tau]

    // C(tau) for all tau via FFT-based cross-correlation.
    // Correlate segment x[0..w) against the full buffer x[0..n), which gives
    // C(tau) = sum_{j=0}^{w-1} x[j] * x[j+tau] for tau = 0..max_tau-1.
    let fft_len = (n + w).next_power_of_two();
    let mut planner = FftPlanner::<f64>::new();
    let fft = planner.plan_fft_forward(fft_len);
    let ifft = planner.plan_fft_inverse(fft_len);

    // Signal A' = reversed first-w segment, zero-padded (this sets up the
    // FFT multiplication to compute cross-correlation, not circular
    // convolution, via the standard "correlation via convolution with a
    // reversed, conjugated kernel" identity).
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

    // The correlation result for lag tau appears at index (w - 1 + tau) in
    // this convolution-of-reversed-kernel construction.
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

// ===== Shared CMNDF / threshold / interpolation (unchanged from naive version) =====

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

fn yin_from_difference(d: &[f64], sample_rate: f64, threshold: f64) -> Option<f64> {
    let d_prime = cmndf(d);
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

fn main() {
    let sample_rate = 48000.0;
    let buffer_size = 2048;
    let max_tau = buffer_size / 2;

    println!("=== Step 1: Correctness verification (naive vs FFT-based) ===\n");

    let test_frequencies = [82.41, 110.00, 220.00, 440.00, 880.00, 1046.50];
    let mut all_match = true;

    for freq in test_frequencies.iter() {
        let buffer = generate_sine(*freq, sample_rate, buffer_size);

        let d_naive = naive_difference_function(&buffer, max_tau);
        let d_fft = fft_difference_function(&buffer, max_tau);

        // Compare the raw difference functions directly first (strongest check)
        let mut max_abs_diff: f64 = 0.0;
        for i in 1..max_tau {
            let diff = (d_naive[i] - d_fft[i]).abs();
            if diff > max_abs_diff { max_abs_diff = diff; }
        }

        let freq_naive = yin_from_difference(&d_naive, sample_rate, THRESHOLD);
        let freq_fft = yin_from_difference(&d_fft, sample_rate, THRESHOLD);

        let match_status = match (freq_naive, freq_fft) {
            (Some(fn_), Some(ff)) => {
                let cents_diff = 1200.0 * (ff / fn_).log2();
                if cents_diff.abs() < 0.01 { "MATCH" } else { all_match = false; "MISMATCH" }
            }
            (None, None) => "MATCH (both None)",
            _ => { all_match = false; "MISMATCH (one detected, one didn't)" }
        };

        println!("Freq {:>8.2}Hz | naive={:?} fft={:?} | max|d_naive-d_fft|={:.6e} | {}",
            freq, freq_naive, freq_fft, max_abs_diff, match_status);
    }

    println!("\nOverall correctness: {}", if all_match { "ALL MATCH" } else { "MISMATCH DETECTED - DO NOT USE" });

    if !all_match {
        println!("\nStopping here - the FFT implementation does not match the naive one.");
        println!("This must be fixed before any speed comparison is meaningful.");
        return;
    }

    println!("\n=== Step 2: Speed comparison (only meaningful since correctness passed) ===\n");

    let buffer = generate_sine(440.0, sample_rate, buffer_size);
    let n_iterations = 1000;

    let start = std::time::Instant::now();
    for _ in 0..n_iterations {
        let _ = naive_difference_function(&buffer, max_tau);
    }
    let naive_elapsed = start.elapsed();

    let start = std::time::Instant::now();
    for _ in 0..n_iterations {
        let _ = fft_difference_function(&buffer, max_tau);
    }
    let fft_elapsed = start.elapsed();

    let naive_per_call = naive_elapsed.as_micros() as f64 / n_iterations as f64;
    let fft_per_call = fft_elapsed.as_micros() as f64 / n_iterations as f64;

    println!("Naive difference function: {:.2} microseconds/call", naive_per_call);
    println!("FFT-based difference function: {:.2} microseconds/call", fft_per_call);
    println!("Speedup: {:.2}x", naive_per_call / fft_per_call);
    println!("\n(This is this SANDBOX's CPU, not the reference hardware - must be");
    println!("re-measured on Michael's actual machine before drawing conclusions.)");
}
