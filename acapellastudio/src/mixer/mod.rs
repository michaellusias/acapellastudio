//! Mixing — roadmap Phase 11, Step 10 ("basic multitracking"). First real
//! logic in this module - previously a stub with only `unimplemented!()`
//! (docs/07_detailed_design.md §2.7, §4 module maturity table).
//!
//! SCOPE, HONEST: this is genuinely basic. It sums multiple sample buffers
//! together with simple clipping prevention. It does NOT yet implement
//! per-track volume, pan, mute, or solo (those fields exist on the `Track`
//! struct in track::mod, but are not read or applied here) - that's real,
//! remaining work, not something this basic mixer secretly does.

/// Mixes multiple equal-length (or padded) sample buffers into one, by
/// summing and then scaling down to prevent clipping.
///
/// SCOPE NOTE: "prevent clipping" here means a simple, fixed scale-down
/// (divide by the number of tracks) - not a real limiter or dynamic gain
/// staging. This will make a 2-track mix noticeably quieter than either
/// track alone, which is a known, honest tradeoff of the simplest correct
/// approach, not a bug. A better approach (e.g. peak-based normalization,
/// or leaving headroom management to the user via track gain) is real,
/// deferred work.
pub fn mix_tracks(tracks: &[Vec<f32>]) -> Vec<f32> {
    if tracks.is_empty() {
        return Vec::new();
    }
    let max_len = tracks.iter().map(|t| t.len()).max().unwrap_or(0);
    let mut mixed = vec![0.0f32; max_len];

    for track in tracks {
        for (i, &sample) in track.iter().enumerate() {
            mixed[i] += sample;
        }
    }

    let track_count = tracks.len() as f32;
    for sample in mixed.iter_mut() {
        *sample /= track_count;
    }

    mixed
}

pub struct Mixer;

impl Mixer {
    /// Real, basic implementation - see `mix_tracks()` above for the
    /// honest scope of what "mixing" means here right now.
    pub fn mix_down(&self, tracks: &[Vec<f32>]) -> Vec<f32> {
        mix_tracks(tracks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mixes_two_equal_length_tracks() {
        let track_a = vec![1.0, 1.0, 1.0];
        let track_b = vec![0.5, 0.5, 0.5];
        let result = mix_tracks(&[track_a, track_b]);
        // (1.0 + 0.5) / 2 = 0.75 for each sample
        assert_eq!(result, vec![0.75, 0.75, 0.75]);
    }

    #[test]
    fn handles_unequal_length_tracks_by_padding_with_silence() {
        let track_a = vec![1.0, 1.0, 1.0, 1.0];
        let track_b = vec![1.0]; // shorter - real case: tracks starting at
                                   // different times or different lengths
        let result = mix_tracks(&[track_a, track_b]);
        assert_eq!(result.len(), 4);
        assert_eq!(result[0], 1.0); // (1.0 + 1.0) / 2
        assert_eq!(result[1], 0.5); // (1.0 + 0.0) / 2 - track_b has nothing here
    }

    #[test]
    fn empty_track_list_produces_empty_output() {
        let result = mix_tracks(&[]);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn single_track_is_unchanged() {
        let track = vec![0.3, -0.5, 0.8];
        let result = mix_tracks(&[track.clone()]);
        assert_eq!(result, track);
    }
}

