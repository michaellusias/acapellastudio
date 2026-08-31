// Krumhansl-Schmuckler key detection prototype
// AcapellaStudio Phase 3 Feasibility Study - Section 4 (Key Detection)
//
// Values verified via two independent real sources in this session:
// arXiv:2104.04143 ("Heaps' Law and Vocabulary Richness in the History of
// Classical Music Harmony") and an independent Python implementation
// write-up (astroneko404.github.io) - both give identical numbers.
//
// HONEST SCOPE NOTE: this tests the algorithm on SYNTHETIC, SYMBOLIC melody
// data (pitch-class + duration pairs I construct directly), not audio, and
// not real recorded singing. This validates the algorithm's correctness on
// clean, unambiguous test cases before the harder, currently-unanswered
// question the Literature Review flagged: how this performs on sparse,
// real, monophonic SUNG audio, which no study we found actually benchmarks.

const MAJOR_PROFILE: [f64; 12] = [6.35, 2.23, 3.48, 2.33, 4.38, 4.09, 2.52, 5.19, 2.39, 3.66, 2.29, 2.88];
const MINOR_PROFILE: [f64; 12] = [6.33, 2.68, 3.52, 5.38, 2.60, 3.53, 2.54, 4.75, 3.98, 2.69, 3.34, 3.17];

const NOTE_NAMES: [&str; 12] = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];

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

/// Returns (key_name, is_major, correlation) for the best match, sorted candidates too.
fn detect_key(chroma: &[f64; 12]) -> (String, bool, f64, Vec<(String, f64)>) {
    let mut candidates = Vec::new();
    for shift in 0..12 {
        let major_rot = rotate(&MAJOR_PROFILE, shift);
        let minor_rot = rotate(&MINOR_PROFILE, shift);
        let major_corr = pearson_correlation(chroma, &major_rot);
        let minor_corr = pearson_correlation(chroma, &minor_rot);
        candidates.push((format!("{} major", NOTE_NAMES[shift]), major_corr));
        candidates.push((format!("{} minor", NOTE_NAMES[shift]), minor_corr));
    }
    candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    let (best_name, best_corr) = candidates[0].clone();
    let is_major = best_name.contains("major");
    (best_name, is_major, best_corr, candidates)
}

/// Build a chroma (pitch-class duration) vector from (pitch_class, duration) pairs.
fn build_chroma(notes: &[(usize, f64)]) -> [f64; 12] {
    let mut chroma = [0.0; 12];
    for (pc, dur) in notes {
        chroma[*pc] += dur;
    }
    chroma
}

fn main() {
    println!("=== AcapellaStudio Phase 3 Feasibility Study - Section 4 ===");
    println!("Krumhansl-Schmuckler Key Detection - synthetic symbolic melody tests\n");

    // Pitch classes: C=0, C#=1, D=2, D#=3, E=4, F=5, F#=6, G=7, G#=8, A=9, A#=10, B=11

    struct TestCase {
        name: &'static str,
        notes: Vec<(usize, f64)>, // (pitch class, duration)
        expected_key: &'static str,
    }

    let test_cases = vec![
        TestCase {
            name: "C major scale, equal durations (unambiguous, all 7 diatonic notes equal)",
            notes: vec![(0,1.0),(2,1.0),(4,1.0),(5,1.0),(7,1.0),(9,1.0),(11,1.0),(0,1.0)],
            expected_key: "C major",
        },
        TestCase {
            name: "A natural minor scale, equal durations",
            notes: vec![(9,1.0),(11,1.0),(0,1.0),(2,1.0),(4,1.0),(5,1.0),(7,1.0),(9,1.0)],
            expected_key: "A minor",
        },
        TestCase {
            name: "G major scale, equal durations",
            notes: vec![(7,1.0),(9,1.0),(11,1.0),(0,1.0),(2,1.0),(4,1.0),(6,1.0),(7,1.0)],
            expected_key: "G major",
        },
        TestCase {
            name: "C major, tonic-weighted (realistic: tonic/dominant emphasized, matches K-K design intent)",
            notes: vec![(0,3.0),(4,1.0),(7,2.0),(0,3.0),(5,1.0),(7,2.0),(11,0.5),(0,2.0)],
            expected_key: "C major",
        },
        TestCase {
            name: "E minor, tonic-weighted",
            notes: vec![(4,3.0),(7,1.0),(11,2.0),(4,3.0),(9,1.0),(11,2.0),(2,0.5),(4,2.0)],
            expected_key: "E minor",
        },
        TestCase {
            name: "AMBIGUOUS: C major scale but missing the leading tone B (relative minor confusion risk)",
            notes: vec![(0,2.0),(2,1.0),(4,1.0),(5,1.0),(7,1.0),(9,1.0),(0,1.0)],
            expected_key: "C major (relative minor A minor is the expected confusion risk)",
        },
    ];

    let mut correct = 0;
    for tc in &test_cases {
        let chroma = build_chroma(&tc.notes);
        let (best_name, _is_major, best_corr, candidates) = detect_key(&chroma);
        let matches = best_name.starts_with(&tc.expected_key.split(' ').take(2).collect::<Vec<_>>().join(" "));
        if matches { correct += 1; }

        println!("--- {} ---", tc.name);
        println!("Expected: {}", tc.expected_key);
        println!("Detected: {} (r = {:.4}) [{}]", best_name, best_corr, if matches { "MATCH" } else { "MISMATCH" });
        println!("Top 3 candidates: {:?}", &candidates[0..3]);
        println!();
    }

    println!("=== SUMMARY ===");
    println!("{}/{} test cases matched expected key", correct, test_cases.len());
    println!("\nNOTE: All tests use synthetic SYMBOLIC (pitch-class + duration) data, not audio.");
    println!("Real sung monophonic audio would need actual pitch detection (already validated");
    println!("separately, Section 2) feeding into this chroma-building step, and would be");
    println!("far sparser/noisier than these clean synthetic test cases.");
}
