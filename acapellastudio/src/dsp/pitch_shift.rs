//! Pitch shifting — REAL prototype logic, with a CONFIRMED, UNRESOLVED DEFECT.
//!
//! Ported from prototypes/psola-prototype/src/main.rs. Validated for pitch
//! accuracy on 5/6 test ratios (Feasibility Study §3.2): correction-scale
//! (+-20 cents) and most harmony-scale intervals (minor third, fifth,
//! octave down) landed within ~1.8-2.8 cents of target.
//!
//! CONFIRMED DEFECT, NOT FIXED: octave-up shifts (ratio = 2.0) produced NO
//! detectable output pitch at all in testing. The Result-returning interface
//! below exists specifically because of this real, found failure - a bare
//! Vec<f64> return type would let this failure mode pass through silently
//! as garbage audio. Callers MUST handle Err, not assume Ok always means
//! a musically valid result.
//!
//! ALSO UNRESOLVED: zero formant-preservation testing has been done. Every
//! test so far used a pure sine wave, which has no formants (Feasibility
//! Study §3.3). This module does not claim formant quality of any kind.
//!
//! Per Problem Statement §33 (Amendment 2): this module now operates
//! strictly post-recording, on a full captured buffer - it does not need
//! to meet a real-time deadline.

pub trait PitchShifter {
    fn shift(&self, input: &[f64], t0: f64, ratio: f64) -> Result<Vec<f64>, PitchShiftError>;
}

#[derive(Debug, PartialEq)]
pub enum PitchShiftError {
    /// Maps directly to the confirmed real defect: this ratio produced no
    /// detectable output pitch during testing (Feasibility Study §3.2).
    /// Currently only ratio = 2.0 (octave up) is known to trigger this;
    /// other large ratios have not been exhaustively tested and may also
    /// be unsafe - this is a real, open gap, not a complete list.
    UnsupportedRatio { ratio: f64 },
}

pub struct PsolaShifter;

fn hann_window(len: usize) -> Vec<f64> {
    (0..len)
        .map(|i| 0.5 * (1.0 - (2.0 * std::f64::consts::PI * i as f64 / (len as f64 - 1.0)).cos()))
        .collect()
}

impl PitchShifter for PsolaShifter {
    fn shift(&self, input: &[f64], t0: f64, ratio: f64) -> Result<Vec<f64>, PitchShiftError> {
        // Real, confirmed guard: reject the specific ratio known to fail
        // (Feasibility Study §3.2) rather than silently produce bad audio.
        // This is a narrow, evidence-based guard, not a general solution -
        // other untested ratios may have the same problem and are not yet
        // caught here.
        if (ratio - 2.0).abs() < 1e-9 {
            return Err(PitchShiftError::UnsupportedRatio { ratio });
        }

        let t0_samples = t0.round() as usize;
        let grain_len = (2.0 * t0).round() as usize;
        let window = hann_window(grain_len);

        let mut analysis_centers = Vec::new();
        let mut c = 0i64;
        while (c as usize) < input.len() {
            analysis_centers.push(c);
            c += t0_samples as i64;
        }

        let t_out = t0 / ratio;
        let output_len = input.len();
        let mut output = vec![0.0; output_len];
        let mut weight_sum = vec![0.0; output_len];

        let mut synth_pos: f64 = 0.0;
        while (synth_pos as usize) < output_len {
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

        for i in 0..output_len {
            if weight_sum[i] > 1e-6 {
                output[i] /= weight_sum[i].max(0.3);
            }
        }

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn octave_up_is_rejected_not_silently_broken() {
        let shifter = PsolaShifter;
        let input = vec![0.0; 8192];
        let result = shifter.shift(&input, 48000.0 / 220.0, 2.0);
        assert_eq!(result, Err(PitchShiftError::UnsupportedRatio { ratio: 2.0 }));
    }

    #[test]
    fn correction_scale_shift_succeeds() {
        let shifter = PsolaShifter;
        let sample_rate = 48000.0;
        let freq = 220.0;
        let t0 = sample_rate / freq;
        let input: Vec<f64> = (0..8192)
            .map(|i| (2.0 * std::f64::consts::PI * freq * i as f64 / sample_rate).sin())
            .collect();
        let ratio = 2f64.powf(20.0 / 1200.0); // +20 cents, per Feasibility Study §3.2
        let result = shifter.shift(&input, t0, ratio);
        assert!(result.is_ok(), "correction-scale shift should not be rejected");
    }
}
