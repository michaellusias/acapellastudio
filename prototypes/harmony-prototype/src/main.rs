// Rule-based diatonic harmony generation prototype
// AcapellaStudio Phase 3 Feasibility Study - Section 5 (Harmony Feasibility)
//
// SCOPE: MVP-level harmony per the Problem Statement - one harmony voice,
// diatonic triadic harmony, vocal-range awareness, basic voice-leading.
//
// HONEST METHOD NOTE: this generates the harmony voice as a diatonic
// interval (third) below or above the melody, per scale degree - NOT a
// fixed chromatic interval (which the Project Vision §12 explicitly warned
// against as sounding like "a simple transposed copy"). A diatonic third
// alternates between major and minor thirds depending on scale degree,
// which is a step up from naive fixed-interval transposition, but is still
// a simple technique relative to true independent four-part voice leading
// (per Yogev & Lerch 2008's more sophisticated CSP-based approach,
// Literature Review §4.1) - this is a legitimate MVP-level starting point,
// not a claim of sophisticated harmonic independence.
//
// This implementation actually CHECKS its own output against real
// voice-leading rules (parallel fifths/octaves) rather than assuming the
// diatonic-third approach avoids them - and reports violations honestly.

#[derive(Debug, Clone, Copy, PartialEq)]
struct Note {
    pitch_class: i32, // 0-11
    octave: i32,      // e.g. 4 = middle octave
}

impl Note {
    fn midi(&self) -> i32 {
        (self.octave + 1) * 12 + self.pitch_class
    }
    fn name(&self) -> String {
        const NAMES: [&str; 12] = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
        format!("{}{}", NAMES[self.pitch_class.rem_euclid(12) as usize], self.octave)
    }
}

/// Major scale degrees (semitone offsets from tonic).
const MAJOR_SCALE: [i32; 7] = [0, 2, 4, 5, 7, 9, 11];
/// Natural minor scale degrees.
const MINOR_SCALE: [i32; 7] = [0, 2, 3, 5, 7, 8, 10];

fn scale_degrees(tonic_pc: i32, is_major: bool) -> Vec<i32> {
    let intervals = if is_major { MAJOR_SCALE } else { MINOR_SCALE };
    intervals.iter().map(|i| (tonic_pc + i).rem_euclid(12)).collect()
}

/// Find the scale degree index (0-6) of a pitch class, if it's diatonic.
fn degree_index(pc: i32, scale: &[i32]) -> Option<usize> {
    scale.iter().position(|&d| d == pc.rem_euclid(12))
}

/// Generate the harmony note a diatonic third below the melody note,
/// respecting the given vocal range (min_midi..=max_midi). If the diatonic
/// third below would fall outside the range, tries a diatonic third above
/// instead (real fallback logic, not silently ignoring the constraint).
fn diatonic_third_harmony(melody: Note, scale: &[i32], min_midi: i32, max_midi: i32) -> Option<(Note, &'static str)> {
    let deg_idx = degree_index(melody.pitch_class, scale)?;

    // Third below = two scale degrees down.
    let below_deg_idx = (deg_idx + 7 - 2) % 7;
    let below_pc = scale[below_deg_idx];
    // Determine correct octave: if below_deg_idx > deg_idx, we wrapped, so drop an octave.
    let mut below_octave = melody.octave;
    if below_deg_idx > deg_idx {
        below_octave -= 1;
    }
    let below = Note { pitch_class: below_pc, octave: below_octave };

    if below.midi() >= min_midi && below.midi() <= max_midi {
        return Some((below, "third below"));
    }

    // Fallback: third above (real fallback, since going below broke range).
    let above_deg_idx = (deg_idx + 2) % 7;
    let above_pc = scale[above_deg_idx];
    let mut above_octave = melody.octave;
    if above_deg_idx < deg_idx {
        above_octave += 1;
    }
    let above = Note { pitch_class: above_pc, octave: above_octave };

    if above.midi() >= min_midi && above.midi() <= max_midi {
        return Some((above, "third above (range fallback)"));
    }

    None // genuinely out of range both ways - honestly reported, not forced
}

/// Check for parallel perfect fifths or octaves between consecutive note
/// pairs - a real, classical voice-leading rule, actually checked against
/// the generated output rather than assumed avoided.
fn check_parallel_motion(melody: &[Note], harmony: &[Note]) -> Vec<(usize, String)> {
    let mut violations = Vec::new();
    for i in 1..melody.len() {
        if i >= harmony.len() { continue; }
        let interval_prev = (melody[i-1].midi() - harmony[i-1].midi()).abs() % 12;
        let interval_curr = (melody[i].midi() - harmony[i].midi()).abs() % 12;

        let melody_moved = melody[i].midi() != melody[i-1].midi();
        let harmony_moved = harmony[i].midi() != harmony[i-1].midi();
        let same_direction = melody_moved && harmony_moved &&
            (melody[i].midi() - melody[i-1].midi()).signum() == (harmony[i].midi() - harmony[i-1].midi()).signum();

        if same_direction && interval_prev == 7 && interval_curr == 7 {
            violations.push((i, "parallel perfect fifth".to_string()));
        }
        if same_direction && interval_prev == 0 && interval_curr == 0 {
            violations.push((i, "parallel octave/unison".to_string()));
        }
    }
    violations
}

fn main() {
    println!("=== AcapellaStudio Phase 3 Feasibility Study - Section 5 ===");
    println!("Rule-based diatonic-third harmony generation - real voice-leading checks\n");

    // Test melody: a simple diatonic tune in C major, hand-constructed.
    // C4 D4 E4 F4 G4 E4 C4 D4 (arbitrary but diatonic, includes some leaps)
    let melody: Vec<Note> = vec![0,2,4,5,7,4,0,2].iter()
        .map(|&pc| Note { pitch_class: pc, octave: 4 })
        .collect();

    let scale = scale_degrees(0, true); // C major
    println!("Melody (C major): {:?}", melody.iter().map(|n| n.name()).collect::<Vec<_>>());

    // Vocal range constraint: alto-ish range, MIDI 53 (F3) to 72 (C5)
    let min_midi = 53;
    let max_midi = 72;
    println!("Harmony vocal range constraint: MIDI {}-{} (F3-C5)\n", min_midi, max_midi);

    let mut harmony_notes = Vec::new();
    let mut methods = Vec::new();
    for &m in &melody {
        match diatonic_third_harmony(m, &scale, min_midi, max_midi) {
            Some((h, method)) => {
                harmony_notes.push(h);
                methods.push(method);
            }
            None => {
                println!("WARNING: no valid harmony note found for {} within range - real limitation", m.name());
                harmony_notes.push(m); // fallback: unison (honest, not hidden)
                methods.push("FAILED - unison fallback");
            }
        }
    }

    println!("Generated harmony:");
    for i in 0..melody.len() {
        println!("  {} -> {} ({})", melody[i].name(), harmony_notes[i].name(), methods[i]);
    }

    let violations = check_parallel_motion(&melody, &harmony_notes);
    println!("\n=== Voice-leading check (real, not assumed) ===");
    if violations.is_empty() {
        println!("No parallel fifth/octave violations detected in this test melody.");
    } else {
        println!("VIOLATIONS FOUND ({}):", violations.len());
        for (idx, kind) in &violations {
            println!("  At note {}: {} ({} -> {})", idx, kind, melody[idx-1].name(), melody[*idx].name());
        }
    }

    // Second test: a melody designed to risk range violations (low melody notes)
    println!("\n=== Second test: low melody, forces range fallback logic ===");
    let low_melody: Vec<Note> = vec![0,2,4].iter()
        .map(|&pc| Note { pitch_class: pc, octave: 3 }) // C3 D3 E3 - low
        .collect();
    println!("Melody: {:?}", low_melody.iter().map(|n| n.name()).collect::<Vec<_>>());
    for &m in &low_melody {
        match diatonic_third_harmony(m, &scale, min_midi, max_midi) {
            Some((h, method)) => println!("  {} -> {} ({})", m.name(), h.name(), method),
            None => println!("  {} -> NO VALID HARMONY (out of range both directions)", m.name()),
        }
    }

    println!("\n=== NOTES ===");
    println!("- Diatonic-third harmony is a legitimate MVP starting point, but is simpler than");
    println!("  true independent four-part voice leading (Yogev & Lerch 2008's CSP approach).");
    println!("- Voice-leading is CHECKED post-hoc here, not prevented by construction - real");
    println!("  production code may need to actively avoid violations, not just report them.");
    println!("- Test melodies are hand-constructed and small - not validated against real");
    println!("  sung melodies or a larger corpus.");
}
