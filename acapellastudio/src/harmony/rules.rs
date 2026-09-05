//! Rule-based harmony generation — REAL prototype logic, with a CONFIRMED,
//! UNRESOLVED DEFECT (this is FR-008a from the SRS).
//!
//! Ported from prototypes/harmony-prototype/src/main.rs. Uses a diatonic
//! third (alternates major/minor by scale degree - NOT a fixed chromatic
//! interval, deliberately avoiding the naive "melody + fixed offset"
//! approach the Project Vision §12 warned against).
//!
//! CONFIRMED DEFECT, NOT FIXED: when neither a diatonic third below nor
//! above the melody note fits the given vocal range, the original prototype
//! silently defaulted to unison (Feasibility Study §5.3, the C3 test case).
//! Unison is not harmony. The Result-returning interface below exists
//! specifically because of this real, found failure.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Note {
    pub pitch_class: i32,
    pub octave: i32,
}

impl Note {
    pub fn midi(&self) -> i32 {
        (self.octave + 1) * 12 + self.pitch_class
    }
}

pub struct Key {
    pub tonic_pitch_class: i32,
    pub is_major: bool,
}

const MAJOR_SCALE: [i32; 7] = [0, 2, 4, 5, 7, 9, 11];
const MINOR_SCALE: [i32; 7] = [0, 2, 3, 5, 7, 8, 10];

fn scale_degrees(key: &Key) -> Vec<i32> {
    let intervals = if key.is_major { MAJOR_SCALE } else { MINOR_SCALE };
    intervals
        .iter()
        .map(|i| (key.tonic_pitch_class + i).rem_euclid(12))
        .collect()
}

fn degree_index(pc: i32, scale: &[i32]) -> Option<usize> {
    scale.iter().position(|&d| d == pc.rem_euclid(12))
}

#[derive(Debug, PartialEq)]
pub enum HarmonyError {
    /// Maps directly to the confirmed real defect found in Feasibility
    /// Study §5.3 - this melody note has no diatonic third (below or
    /// above) that satisfies the given vocal range.
    NoValidHarmonyForRange { note_index: usize },
}

pub trait HarmonyGenerator {
    fn generate(
        &self,
        melody: &[Note],
        key: &Key,
        vocal_range: (i32, i32),
    ) -> Result<Vec<Note>, HarmonyError>;
}

pub struct DiatonicThirdHarmonizer;

fn diatonic_third_for_note(melody: Note, scale: &[i32], min_midi: i32, max_midi: i32) -> Option<Note> {
    let deg_idx = degree_index(melody.pitch_class, scale)?;

    let below_deg_idx = (deg_idx + 7 - 2) % 7;
    let below_pc = scale[below_deg_idx];
    let mut below_octave = melody.octave;
    if below_deg_idx > deg_idx {
        below_octave -= 1;
    }
    let below = Note { pitch_class: below_pc, octave: below_octave };
    if below.midi() >= min_midi && below.midi() <= max_midi {
        return Some(below);
    }

    let above_deg_idx = (deg_idx + 2) % 7;
    let above_pc = scale[above_deg_idx];
    let mut above_octave = melody.octave;
    if above_deg_idx < deg_idx {
        above_octave += 1;
    }
    let above = Note { pitch_class: above_pc, octave: above_octave };
    if above.midi() >= min_midi && above.midi() <= max_midi {
        return Some(above);
    }

    None
}

impl HarmonyGenerator for DiatonicThirdHarmonizer {
    fn generate(
        &self,
        melody: &[Note],
        key: &Key,
        vocal_range: (i32, i32),
    ) -> Result<Vec<Note>, HarmonyError> {
        let scale = scale_degrees(key);
        let (min_midi, max_midi) = vocal_range;
        let mut harmony = Vec::with_capacity(melody.len());

        for (i, &m) in melody.iter().enumerate() {
            match diatonic_third_for_note(m, &scale, min_midi, max_midi) {
                Some(h) => harmony.push(h),
                // Real fix from the design in 07_detailed_design.md: return
                // Err instead of the original prototype's silent unison
                // fallback (Feasibility Study §5.3's confirmed defect).
                None => return Err(HarmonyError::NoValidHarmonyForRange { note_index: i }),
            }
        }
        Ok(harmony)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_diatonic_thirds_for_c_major_melody() {
        let melody: Vec<Note> = vec![0, 2, 4, 5, 7, 4, 0, 2]
            .iter()
            .map(|&pc| Note { pitch_class: pc, octave: 4 })
            .collect();
        let key = Key { tonic_pitch_class: 0, is_major: true };
        let harmonizer = DiatonicThirdHarmonizer;
        let result = harmonizer.generate(&melody, &key, (53, 72));
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), melody.len());
    }

    /// Real regression test for the confirmed FR-008a defect (Feasibility
    /// Study §5.3): a C3 melody note with this range has no valid diatonic
    /// third in either direction, and this MUST now return Err, not a
    /// silent unison fallback.
    #[test]
    fn range_fallback_failure_returns_err_not_silent_unison() {
        let melody = vec![Note { pitch_class: 0, octave: 3 }]; // C3
        let key = Key { tonic_pitch_class: 0, is_major: true };
        let harmonizer = DiatonicThirdHarmonizer;
        let result = harmonizer.generate(&melody, &key, (53, 72)); // F3-C5, same as
                                                                      // the original test
        assert_eq!(result, Err(HarmonyError::NoValidHarmonyForRange { note_index: 0 }));
    }
}
