//! Clip editing — NO REAL IMPLEMENTATION. Design only, per
//! docs/07_detailed_design.md §2.5.

use std::path::PathBuf;

pub struct AudioClip {
    pub source_path: PathBuf, // per NFR-AUDIO-004: never modified in place
    pub trim_start: u64,
    pub trim_end: u64,
    pub gain_db: f64,
}

pub enum ClipEditOp {
    Trim { start: u64, end: u64 },
    Split { at: u64 },
    Move { new_position: u64 },
    Gain { db: f64 },
}
