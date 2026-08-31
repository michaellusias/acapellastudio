// TD-PSOLA (time-domain pitch-synchronous overlap-add) pitch shifting
// AcapellaStudio Phase 3 Feasibility Study - Section 3 (Pitch Shifting)
//
// HONEST METHOD NOTE: this is a basic, simplified TD-PSOLA implementation
// for feasibility testing, not a production-quality or literature-exact
// implementation of any specific paper's precise algorithm. It uses known
// analysis pitch periods (since we control the synthetic test signal
// generation and know the true period exactly) rather than a real pitch-mark
// detector - real vocal input would need actual pitch-mark detection first,
// which is a separate, not-yet-built component. This tests whether the core
// PSOLA resynthesis concept produces a correctly-shifted output, verified
// using our own already-validated YIN detector - not whether a full
// real-vocal pitch-shifting pipeline works end-to-end yet.
//
// ALGORITHM (standard TD-PSOLA, not novel):
// 1. Analysis: extract Hann-windowed grains of length 2*T0 centered at each
//    multiple of the original pitch period T0.
// 2. Synthesis: place windowed copies of the nearest analysis grain at new
//    synthesis marks spaced by T_out = T0 / shift_ratio, overlap-adding into
//    the output buffer. This naturally repeats grains (pitch shift up) or
//    skips grains (pitch shift down) while preserving output duration.

const SAMPLE_RATE: f64 = 48000.0;
const YIN_THRESHOLD: f64 = 0.10;

// ===== Re-used, already-validated YIN implementation (unchanged) =====

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

fn cents_error(true_freq: f64, detected_freq: f64) -> f64 {
    1200.0 * (detected_freq / true_freq).log2()
}

// ===== Hann window =====

fn hann_window(len: usize) -> Vec<f64> {
    (0..len)
        .map(|i| 0.5 * (1.0 - (2.0 * std::f64::consts::PI * i as f64 / (len as f64 - 1.0)).cos()))
        .collect()
}

// ===== TD-PSOLA pitch shift =====

fn psola_pitch_shift(input: &[f64], t0: f64, shift_ratio: f64) -> Vec<f64> {
    let t0_samples = t0.round() as usize;
    let grain_len = (2.0 * t0).round() as usize;
    let window = hann_window(grain_len);

    // Analysis grain centers: every T0 samples
    let mut analysis_centers = Vec::new();
    let mut c = 0i64;
    while (c as usize) < input.len() {
        analysis_centers.push(c);
        c += t0_samples as i64;
    }

    let t_out = t0 / shift_ratio;
    let output_len = input.len();
    let mut output = vec![0.0; output_len];
    let mut weight_sum = vec![0.0; output_len]; // for proper overlap-add normalization

    let mut synth_pos: f64 = 0.0;
    while (synth_pos as usize) < output_len {
        // Find nearest analysis grain center to this synthesis position
        // (maps synthesis time to the closest available analysis grain -
        // this is what causes grain repetition/skipping for pitch shifts).
        let nearest_idx = ((synth_pos / t0_samples as f64).round() as i64)
            .clamp(0, analysis_centers.len() as i64 - 1) as usize;
        let center = analysis_centers[nearest_idx];

        let half = (grain_len / 2) as i64;
        for k in 0..grain_len {
            let src_idx = center - half + k as i64;
            if src_idx < 0 || src_idx as usize >= input.len() {
                continue;
            }
            let out_idx_signed = synth_pos as i64 - half + k as i64;
            if out_idx_signed < 0 || out_idx_signed as usize >= output_len {
                continue;
            }
            let out_idx = out_idx_signed as usize;
            let sample = input[src_idx as usize] * window[k];
            output[out_idx] += sample;
            weight_sum[out_idx] += window[k];
        }

        synth_pos += t_out;
    }

    // Normalize by accumulated window weight to avoid amplitude artifacts
    // from overlap-add (a standard PSOLA normalization step).
    for i in 0..output_len {
        if weight_sum[i] > 1e-6 {
            output[i] /= weight_sum[i].max(0.3); // floor avoids extreme boosting at edges
        }
    }

    output
}

fn main() {
    println!("=== AcapellaStudio Phase 3 Feasibility Study - Section 3 ===");
    println!("TD-PSOLA Pitch Shifting Prototype - validated against our own YIN detector\n");

    let test_cases: Vec<(&str, f64, f64)> = vec![
        ("Correction-scale: +20 cents (small)", 220.0, 2f64.powf(20.0 / 1200.0)),
        ("Correction-scale: -20 cents (small)", 220.0, 2f64.powf(-20.0 / 1200.0)),
        ("Harmony-scale: minor third up (+300 cents)", 220.0, 2f64.powf(300.0 / 1200.0)),
        ("Harmony-scale: perfect fifth up (+700 cents)", 220.0, 2f64.powf(700.0 / 1200.0)),
        ("Harmony-scale: octave down (-1200 cents)", 220.0, 0.5),
        ("Harmony-scale: octave up (+1200 cents)", 220.0, 2.0),
    ];

    let buffer_size = 8192; // longer buffer needed since PSOLA needs several periods
    println!("{:<45} {:>10} {:>12} {:>14} {:>12}", "Test", "Input Hz", "Target Hz", "Detected Hz", "Error(cents)");

    for (name, input_freq, ratio) in test_cases.iter() {
        let input = generate_sine(*input_freq, SAMPLE_RATE, buffer_size);
        let t0 = SAMPLE_RATE / input_freq;
        let output = psola_pitch_shift(&input, t0, *ratio);

        let target_freq = input_freq * ratio;

        // Detect on a stable middle section of the output, away from edge artifacts
        let analysis_start = buffer_size / 4;
        let analysis_end = analysis_start + 2048;
        let analysis_window = &output[analysis_start..analysis_end.min(output.len())];

        match yin_detect(analysis_window, SAMPLE_RATE, YIN_THRESHOLD) {
            Some(detected) => {
                let err = cents_error(target_freq, detected);
                println!("{:<45} {:>10.2} {:>12.2} {:>14.2} {:>12.2}", name, input_freq, target_freq, detected, err);
            }
            None => {
                println!("{:<45} {:>10.2} {:>12.2} {:>14} {:>12}", name, input_freq, target_freq, "NO DETECT", "N/A");
            }
        }
    }

    println!("\n=== NOTES ===");
    println!("- Analysis grain centers use the KNOWN true period (synthetic signal), not a real");
    println!("  pitch-mark detector - real vocal input needs that as a separate component.");
    println!("- This is a basic, simplified PSOLA - not literature-exact or production quality.");
    println!("- No formant/timbre-quality assessment here - only whether the OUTPUT PITCH matches");
    println!("  the target, verified via our own already-validated YIN detector.");
}
