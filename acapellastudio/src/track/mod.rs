//! Track management — NO REAL IMPLEMENTATION. Design only, per
//! docs/07_detailed_design.md §2.6.

pub type TrackId = u64;

pub enum TrackType {
    LeadVocal,
    HarmonyVoice,
    VocalPercussion,
}

pub struct Track {
    pub id: TrackId,
    pub track_type: TrackType,
    pub volume: f64,
    pub pan: f64,
    pub mute: bool,
    pub solo: bool,
}
