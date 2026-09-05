//! Key/scale detection — REAL, TESTED implementation.
//!
//! Ported from prototypes/key-detection-prototype/src/main.rs. Krumhansl-
//! Kessler profile values independently verified via 2 real sources
//! (Feasibility Study §4.1). Verified correct on 6/6 synthetic symbolic
//! test cases, including a deliberate relative-minor-confusion ambiguity
//! test (Feasibility Study §4.2).
//!
//! Real audio/singing-voice accuracy remains untested (Feasibility Study
//! §4.3) - this module's correctness on real chroma vectors derived from
//! actual singing has not been demonstrated, only on hand-constructed
//! symbolic input.

const MAJOR_PROFILE: [f64; 12] = [
    6.35, 2.23, 3.48, 2.33, 4.38, 4.09, 2.52, 5.19, 2.39, 3.66, 2.29, 2.88,
];
const MINOR_PROFILE: [f64; 12] = [
    6.33, 2.68, 3.52, 5.38, 2.60, 3.53, 2.54, 4.75, 3.98, 2.69, 3.34, 3.17,
];

pub struct KeyCandidate {
    pub tonic_pitch_class: u8,
    pub is_major: bool,
    pub correlation: f64,
}

pub struct KeyDetector;

fn rotate(profile: &[f64; 12], shift: usize) -> [f64; 12] {
    let mut out = [0.0; 12];
    for i in 0..12 {
        out[i] = profile[(i + 12 - shift) % 12];
    }
    out
}

fn pearson_correlation(a: &[f64; 12], b: &[f64; 12]) -> f64 {
    let mean_a: f64 = a.iter().sum::<f64>() / 12.0;
    let mean_b: f64 = b.iter().sum::<f64>() / 12.0;
    let mut cov = 0.0;
    let mut var_a = 0.0;
    let mut var_b = 0.0;
    for i in 0..12 {
        let da = a[i] - mean_a;
        let db = b[i] - mean_b;
        cov += da * db;
        var_a += da * da;
        var_b += db * db;
    }
    if var_a < 1e-12 || var_b < 1e-12 {
        return 0.0;
    }
    cov / (var_a.sqrt() * var_b.sqrt())
}

impl KeyDetector {
    /// Returns candidates sorted by correlation, descending. Callers should
    /// present at least the top 1-2 candidates for user confirmation
    /// (Problem Statement §2.3), not just the single best match - the real
    /// E minor/E major test case (Feasibility Study §4.2) showed a narrow
    /// margin (0.8213 vs 0.8132) even on clean synthetic data, so treating
    /// the top result as certain would be dishonest to what was found.
    pub fn detect(&self, chroma: &[f64; 12]) -> Vec<KeyCandidate> {
        let mut candidates = Vec::new();
        for shift in 0..12u8 {
            let major_rot = rotate(&MAJOR_PROFILE, shift as usize);
            let minor_rot = rotate(&MINOR_PROFILE, shift as usize);
            candidates.push(KeyCandidate {
                tonic_pitch_class: shift,
                is_major: true,
                correlation: pearson_correlation(chroma, &major_rot),
            });
            candidates.push(KeyCandidate {
                tonic_pitch_class: shift,
                is_major: false,
                correlation: pearson_correlation(chroma, &minor_rot),
            });
        }
        candidates.sort_by(|a, b| b.correlation.partial_cmp(&a.correlation).unwrap());
        candidates
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_chroma(notes: &[(usize, f64)]) -> [f64; 12] {
        let mut chroma = [0.0; 12];
        for (pc, dur) in notes {
            chroma[*pc] += dur;
        }
        chroma
    }

    /// Real regression test directly from Feasibility Study §4.2's C major
    /// scale test case.
    #[test]
    fn detects_c_major_scale() {
        let chroma = build_chroma(&[
            (0, 1.0), (2, 1.0), (4, 1.0), (5, 1.0), (7, 1.0), (9, 1.0), (11, 1.0), (0, 1.0),
        ]);
        let detector = KeyDetector;
        let candidates = detector.detect(&chroma);
        let best = &candidates[0];
        assert_eq!(best.tonic_pitch_class, 0);
        assert!(best.is_major);
    }

    /// Regression test for the real, narrow-margin E minor/E major case
    /// found in Feasibility Study §4.2 - checks the correct answer still
    /// wins, without asserting a specific margin (since that's sensitive
    /// to exact input construction).
    #[test]
    fn detects_e_minor_despite_narrow_margin() {
        let chroma = build_chroma(&[
            (4, 3.0), (7, 1.0), (11, 2.0), (4, 3.0), (9, 1.0), (11, 2.0), (2, 0.5), (4, 2.0),
        ]);
        let detector = KeyDetector;
        let candidates = detector.detect(&chroma);
        let best = &candidates[0];
        assert_eq!(best.tonic_pitch_class, 4);
        assert!(!best.is_major);
    }
}
