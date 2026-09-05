//! Manual pitch editing — NO REAL IMPLEMENTATION. Design only, per
//! docs/07_detailed_design.md §2.4. This module intentionally contains
//! only the data shapes from that design, not working logic, to keep the
//! module compiling (Phase 10's "empty application" deliverable) without
//! pretending anything here has been built or tested.

pub struct PitchEdit {
    pub region_start: u64,
    pub region_end: u64,
    pub target_frequency: f64,
    pub is_manual_override: bool, // per Problem Statement R-007
}

// EditablePitchContour and its effective_pitch_at() logic (07_detailed_design.md
// §2.4) are deliberately NOT stubbed here yet - even a stub would need to
// depend on dsp::pitch's PitchContour type, which itself has no real
// definition yet (dsp::pitch only exposes a detector, not a persisted
// contour structure). Left undefined rather than guessed at.
