//! Pitch detection — REAL, TESTED implementation.
//!
//! Ported from prototypes/yin-fft-prototype/src/main.rs, which was verified
//! mathematically identical to the naive YIN implementation to within
//! floating-point precision (Feasibility Study §2.9), and real-hardware-
//! tested at 28% CPU during live operation (Feasibility Study §2.11).
//!
//! KNOWN FIX APPLIED HERE (not present in the original prototype): the
//! FftPlanner is owned by the YinFftDetector struct as long-lived state,
//! not created inside detect(). The prototype's per-call planner creation
//! was identified as the likely cause of the real CPU figure (28%) falling
//! short of the sandbox-measured speedup projection (Feasibility Study
//! §2.11-§2.13). This has NOT yet been re-measured on real hardware with
//! this exact fix - that re-measurement is a real, open action item.

use rustfft::{num_complex::Complex, FftPlanner};

pub trait PitchDetector {
    fn detect(&mut self, buffer: &[f64], sample_rate: f64) -> Option<f64>;
}

pub struct YinFftDetector {
    planner: FftPlanner<f64>,
    threshold: f64, // real: 0.10, de Cheveigné & Kawahara (2002)
}

impl YinFftDetector {
    pub fn new() -> Self {
        Self {
            planner: FftPlanner::new(),
            threshold: 0.10,
        }
    }

    fn fft_difference_function(&mut self, buffer: &[f64], max_tau: usize) -> Vec<f64> {
        let n = buffer.len();
        let w = n - max_tau;

        let a: f64 = buffer[0..w].iter().map(|x| x * x).sum();

        let mut prefix_sq = vec![0.0; n + 1];
        for i in 0..n {
            prefix_sq[i + 1] = prefix_sq[i] + buffer[i] * buffer[i];
        }

        let fft_len = (n + w).next_power_of_two();
        let fft = self.planner.plan_fft_forward(fft_len);
        let ifft = self.planner.plan_fft_inverse(fft_len);

        let mut sig_full: Vec<Complex<f64>> =
            buffer.iter().map(|&x| Complex::new(x, 0.0)).collect();
        sig_full.resize(fft_len, Complex::new(0.0, 0.0));

        let mut kernel: Vec<Complex<f64>> = buffer[0..w]
            .iter()
            .rev()
            .map(|&x| Complex::new(x, 0.0))
            .collect();
        kernel.resize(fft_len, Complex::new(0.0, 0.0));

        fft.process(&mut sig_full);
        fft.process(&mut kernel);

        let mut product: Vec<Complex<f64>> = sig_full
            .iter()
            .zip(kernel.iter())
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
}

impl Default for YinFftDetector {
    fn default() -> Self {
        Self::new()
    }
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

impl PitchDetector for YinFftDetector {
    fn detect(&mut self, buffer: &[f64], sample_rate: f64) -> Option<f64> {
        let max_tau = buffer.len() / 2;
        let threshold = self.threshold;
        let d = self.fft_difference_function(buffer, max_tau);
        let d_prime = cmndf(&d);
        let tau = absolute_threshold(&d_prime, threshold)?;
        let refined_tau = parabolic_interpolation(&d_prime, tau);
        if refined_tau <= 0.0 {
            return None;
        }
        Some(sample_rate / refined_tau)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Real regression test, directly encoding the reference-tone result
    /// from Feasibility Study §2.7 (0.99 cents mean error on a real 440Hz
    /// tone through the actual acoustic chain). This synthetic version
    /// checks the algorithm still behaves correctly on a clean 440Hz sine,
    /// as a regression guard - it does NOT re-run the real hardware test,
    /// which requires a microphone and cannot run in CI.
    #[test]
    fn detects_440hz_within_5_cents() {
        let sample_rate = 48000.0;
        let buffer: Vec<f64> = (0..2048)
            .map(|i| (2.0 * std::f64::consts::PI * 440.0 * i as f64 / sample_rate).sin())
            .collect();
        let mut detector = YinFftDetector::new();
        let detected = detector.detect(&buffer, sample_rate).expect("should detect a pitch");
        let cents_error = 1200.0 * (detected / 440.0).log2();
        assert!(cents_error.abs() < 5.0, "cents error {} exceeds 5 cent bar", cents_error);
    }
}
