# AcapellaStudio — Detailed Software Design

**Document:** 07_detailed_design.md (roadmap Phase 8)
**Status:** Draft v1
**Predecessor documents:** 06_architecture.md, 08_technology_selection.md, 01_problem_statement.md §33 (post-recording correction amendment)

> **Standing rule for this document:** where a module's interface is copied or directly adapted from real, tested prototype code, that's stated explicitly with a file reference. Where a module is fresh design with no prototype behind it, that's stated too. Nothing here should read as more "done" than it is.

---

# 1. Module Layout

```text
src/
├── audio/          — real-time I/O (cpal), buffer management
├── dsp/
│   ├── pitch/      — pitch DETECTION (real-time-capable; may run live for feedback)
│   └── pitch_shift/ — pitch SHIFTING (post-recording only, per Amendment 2)
├── pitch_edit/     — manual pitch editing (view/drag pitch contour)
├── clip/           — non-destructive clip editing (trim/split/fade/gain)
├── track/          — track management (multitrack coordination)
├── mixer/          — volume/pan/mute/solo, mix-down
├── harmony/
│   ├── key/        — key/scale detection
│   └── rules/      — rule-based harmony generation
├── project/        — save/load, project file format
├── export/         — audio file export
└── gui/            — UI layer (framework not yet finalized, Tech Selection §4)
```

---

# 2. Module Specifications

## 2.1 `audio/` — Real-Time Audio I/O

**Real basis:** directly adapted from `prototypes/audio-io-prototype/src/main.rs`. This is the closest thing to "already designed" in this document.

```rust
pub struct AudioEngine {
    host: cpal::Host,
    input_device: cpal::Device,
    output_device: cpal::Device,
    input_config: cpal::StreamConfig,
    output_config: cpal::StreamConfig,
}

impl AudioEngine {
    pub fn new() -> Result<Self, AudioEngineError>;
    pub fn with_buffer_size(self, size: u32) -> Self;  // real: BufferSize::Fixed(128)
                                                          // confirmed honored, Feasibility
                                                          // Study §1.5
    pub fn start_recording(&mut self, sink: impl AudioSink) -> Result<StreamHandle, AudioEngineError>;
    pub fn start_monitoring(&mut self) -> Result<StreamHandle, AudioEngineError>; // raw,
                                                          // uncorrected only, per Amendment 2
}
```

**RESOLVED in Phase 11** (this design document predates that implementation work): the real `AudioEngine` implementation (`acapellastudio/src/audio/mod.rs`) uses `rtrb`, a wait-free SPSC ring buffer, in place of the `AudioSink` trait design sketched below — `Producer::push()` inside the real-time callback never blocks or allocates. The trait sketch below is retained for historical context (it correctly identified the requirement, before a concrete solution existed) rather than as the shipped design.

```rust
pub trait AudioSink: Send {
    fn write(&mut self, samples: &[f32]); // superseded by rtrb::Producer in the
                                            // real implementation - see above
}
```

## 2.2 `dsp::pitch` — Pitch Detection

**Real basis:** directly adapted from `prototypes/yin-fft-prototype/src/main.rs`, the FFT-optimized, mathematically-verified-identical-to-naive implementation (Feasibility Study §2.9).

```rust
pub trait PitchDetector {
    fn detect(&mut self, buffer: &[f64], sample_rate: f64) -> Option<f64>;
}

pub struct YinFftDetector {
    planner: rustfft::FftPlanner<f64>,  // REAL KNOWN ISSUE: must be created once and
                                          // reused, not per-call - the per-callback
                                          // creation bug (Feasibility Study §2.11) is
                                          // exactly the mistake this struct field is
                                          // designed to prevent, by owning the planner
                                          // as long-lived state rather than a local
    threshold: f64,                      // real: 0.10, per de Cheveigné & Kawahara (2002)
}

impl PitchDetector for YinFftDetector {
    fn detect(&mut self, buffer: &[f64], sample_rate: f64) -> Option<f64> {
        // real algorithm: difference_function (FFT-based) -> cmndf ->
        // absolute_threshold -> parabolic_interpolation
        // see prototypes/yin-fft-prototype/src/main.rs for the validated implementation
    }
}
```

**Data structure:**
```rust
pub struct PitchContour {
    pub notes: Vec<DetectedNote>,
}

pub struct DetectedNote {
    pub frequency: f64,
    pub onset_sample: u64,
    pub duration_samples: u64,
    pub confidence: f64,  // NOT YET DEFINED how this is computed - open item
}
```

**Honest gap:** `confidence` is listed because a real system needs it (e.g. to decide whether to trust a detection for harmony generation), but no prototype has ever computed or used a confidence value — YIN's threshold check is currently a hard yes/no, not a graded confidence. This field is aspirational, not implemented.

## 2.3 `dsp::pitch_shift` — Pitch Shifting

**Amended (Problem Statement §33): this module now operates strictly post-recording, on a full captured buffer, not inside the real-time callback.** This relaxes correctness pressure but does not fix the module's real, known defect below.

**Real basis:** directly adapted from `prototypes/psola-prototype/src/main.rs`.

```rust
pub trait PitchShifter {
    fn shift(&self, input: &[f64], t0: f64, ratio: f64) -> Vec<f64>;
}

pub struct PsolaShifter;

impl PitchShifter for PsolaShifter {
    fn shift(&self, input: &[f64], t0: f64, ratio: f64) -> Vec<f64> {
        // real algorithm: analysis grains at known/detected period T0, Hann-windowed,
        // resynthesized at spacing T0/ratio - see prototypes/psola-prototype/src/main.rs
    }
}
```

**⚠️ Real, confirmed, unresolved defect (Feasibility Study §3.2):** octave-up shifts (`ratio = 2.0`) produce no detectable output pitch at all. **This interface must not be treated as safe to call with arbitrary ratios until this is fixed.** A production version of this trait should probably return `Result<Vec<f64>, PitchShiftError>` rather than a bare `Vec<f64>`, specifically so this failure mode can be surfaced rather than silently producing garbage audio — this is a design correction being made *because* of the real bug found, not a hypothetical.

```rust
pub enum PitchShiftError {
    UnsupportedRatio { ratio: f64 },  // maps directly to the confirmed octave-up failure
}
```

**Also unresolved:** no formant-preservation logic exists in this module at all (Technology Selection §3.4) — the current implementation shifts pitch with no regard for timbre/formant quality, untested in that dimension entirely.

## 2.4 `pitch_edit/` — Manual Pitch Editing

**Real basis:** none. Fresh design, no prototype exists.

```rust
pub struct PitchEdit {
    pub region_start: u64,
    pub region_end: u64,
    pub target_frequency: f64,
    pub is_manual_override: bool,  // per Problem Statement R-007 - prevents automatic
                                     // correction from silently overwriting this
}

pub struct EditablePitchContour {
    pub detected: PitchContour,           // from dsp::pitch
    pub overrides: Vec<PitchEdit>,        // user edits layered on top
}

impl EditablePitchContour {
    pub fn effective_pitch_at(&self, sample: u64) -> f64 {
        // real requirement (Problem Statement R-007): check overrides first,
        // fall back to detected pitch - NOT YET IMPLEMENTED, design only
    }
}
```

## 2.5 `clip/` — Clip Editing

**Real basis:** none.

```rust
pub struct AudioClip {
    pub source_path: PathBuf,       // per NFR-AUDIO-004: original recording, never
                                      // modified in place
    pub trim_start: u64,
    pub trim_end: u64,
    pub fades: FadeSettings,
    pub gain_db: f64,
}

pub enum ClipEditOp {
    Trim { start: u64, end: u64 },
    Split { at: u64 },
    Move { new_position: u64 },
    Fade { in_samples: u64, out_samples: u64 },
    Gain { db: f64 },
}
```

**Non-destructive guarantee (NFR-AUDIO-004):** `ClipEditOp` variants describe transformations *on top of* `source_path`, which is never rewritten — this satisfies the requirement by construction, provided the render/playback path always re-reads `source_path` + current edit state rather than baking edits into a new file in place. **This is a design intention, not yet verified by any implementation.**

## 2.6 `track/` — Track Management

**Real basis:** none.

```rust
pub enum TrackType {
    LeadVocal,
    HarmonyVoice,
    VocalPercussion,
}

pub struct Track {
    pub id: TrackId,
    pub track_type: TrackType,
    pub clips: Vec<AudioClip>,
    pub volume: f64,
    pub pan: f64,
    pub mute: bool,
    pub solo: bool,
}

pub struct TrackManager {
    tracks: Vec<Track>,
}

impl TrackManager {
    pub fn create_track(&mut self, track_type: TrackType) -> TrackId;
    pub fn delete_track(&mut self, id: TrackId) -> Result<(), TrackError>;
    pub fn reorder_track(&mut self, id: TrackId, new_index: usize) -> Result<(), TrackError>;
}
```

## 2.7 `mixer/` — Mixing

**Real basis:** none.

```rust
pub struct Mixer {
    // NOT YET DESIGNED IN DETAIL beyond this stub - real-time mixing summation logic,
    // gain-staging, and clipping prevention have received no design work at all
}

impl Mixer {
    pub fn mix_down(&self, tracks: &[Track]) -> Vec<f32> {
        unimplemented!("no design exists for this yet")
    }
}
```

**Honest note:** this is the least-designed module in the whole document. The roadmap's Phase 8 goal calls for this module to exist; a real design pass is still needed, not just a stub.

## 2.8 `harmony::key` — Key Detection

**Real basis:** directly adapted from `prototypes/key-detection-prototype/src/main.rs`, verified on 6/6 synthetic symbolic test cases (Feasibility Study §4.2).

```rust
pub struct KeyDetector {
    major_profile: [f64; 12],  // real, independently-verified values (Feasibility
    minor_profile: [f64; 12],  // Study §4.1): Krumhansl-Kessler
}

pub struct KeyCandidate {
    pub tonic_pitch_class: u8,
    pub is_major: bool,
    pub correlation: f64,
}

impl KeyDetector {
    pub fn detect(&self, chroma: &[f64; 12]) -> Vec<KeyCandidate> {
        // real algorithm: 24-way rotated Pearson correlation, sorted descending
        // see prototypes/key-detection-prototype/src/main.rs
    }
}
```

**Real, important design implication from testing:** because the E minor test case had a narrow real margin against E major (0.8213 vs. 0.8132, Feasibility Study §4.2), `detect()` returns a **ranked list of candidates**, not a single answer — directly reflecting Problem Statement §2.3's user-confirmation requirement and the real evidence that a single silent answer would sometimes be misleading.

## 2.9 `harmony::rules` — Rule-Based Harmony Generation

**Real basis:** directly adapted from `prototypes/harmony-prototype/src/main.rs`, including its real, confirmed defect.

```rust
pub trait HarmonyGenerator {
    fn generate(&self, melody: &[Note], key: &Key, vocal_range: (i32, i32)) -> Result<Vec<Note>, HarmonyError>;
}

pub struct DiatonicThirdHarmonizer;

pub enum HarmonyError {
    NoValidHarmonyForRange { note_index: usize },  // maps directly to the real,
                                                     // confirmed FR-008a defect found
                                                     // in Feasibility Study §5.3 (the
                                                     // C3 test case) - this variant
                                                     // exists because we found this
                                                     // failure through real testing
}

impl HarmonyGenerator for DiatonicThirdHarmonizer {
    fn generate(&self, melody: &[Note], key: &Key, vocal_range: (i32, i32)) -> Result<Vec<Note>, HarmonyError> {
        // real algorithm: diatonic third below, falls back to third above if out of
        // range - prototypes/harmony-prototype/src/main.rs's diatonic_third_harmony()
        // REAL DEFECT, NOT YET FIXED: when neither direction fits, the prototype
        // silently defaulted to unison. This trait signature (returning Result) is
        // the fix-in-progress: production code MUST return Err here, not a silent
        // degraded Ok, per the real failure already found.
    }
}
```

## 2.10 `project/` — Project Persistence

**Real basis:** none implemented; format proposed in Technology Selection §6 (directory-based structure), grounded in the domain model from System Analysis §5.

```rust
pub struct Project {
    pub sample_rate: u32,
    pub tracks: Vec<Track>,
    pub detected_key: Option<KeyCandidate>,
}

pub struct ProjectManager;

impl ProjectManager {
    pub fn save(&self, project: &Project, path: &Path) -> Result<(), ProjectError> {
        unimplemented!("no serialization design exists yet beyond the directory layout proposal")
    }
    pub fn load(&self, path: &Path) -> Result<Project, ProjectError> {
        unimplemented!()
    }
}
```

## 2.11 `export/` — Audio Export

**Real basis:** none.

```rust
pub struct Exporter;

impl Exporter {
    pub fn export(&self, project: &Project, format: ExportFormat, path: &Path) -> Result<(), ExportError> {
        unimplemented!("no design work done - format support, encoding, and the
                         mix-down dependency (mixer/, also undesigned) are both open")
    }
}
```

## 2.12 `gui/` — UI Layer

**Real basis:** none. Framework not finalized (Technology Selection §4 — Iced is the leading candidate, not selected). No module design is attempted here beyond noting the dependency: this module cannot be meaningfully designed until the framework decision is closed.

---

# 3. Cross-Cutting Data Structures

```rust
pub struct Note {
    pub pitch_class: i32,  // 0-11, matches prototypes/harmony-prototype's convention
    pub octave: i32,
}

pub struct Key {
    pub tonic_pitch_class: u8,
    pub is_major: bool,
}

pub type TrackId = u64;
```

---

# 4. Module Maturity Summary

| Module | Real prototype behind it? | Known defects | Design completeness |
|---|---|---|---|
| `audio/` | Yes | ~~Mutex-based ring buffer not RT-safe~~ RESOLVED (Phase 11, rtrb) | High — closest to production-ready design |
| `dsp::pitch` | Yes | None found; `confidence` field unimplemented | High |
| `dsp::pitch_shift` | Yes | **Octave-up failure (confirmed)**; zero formant testing | Medium — interface redesigned around the known defect |
| `pitch_edit/` | No | — | Low — first-pass design only |
| `clip/` | No | — | Low |
| `track/` | No | — | Low |
| `mixer/` | Yes (Phase 11) | No per-track volume/pan/mute/solo yet — pure summation only | Basic — real, tested summation logic; no gain-staging sophistication |
| `harmony::key` | Yes | Real accuracy gap on sung audio (open) | High |
| `harmony::rules` | Yes | **Range-fallback failure (confirmed, FR-008a)** | Medium — interface redesigned around the known defect |
| `project/` | No | — | Low — format proposed, not implemented |
| `export/` | No | — | Low |
| `gui/` | No | — | **None — blocked on framework selection** |

---

# 5. Status

Draft v1. Per the honest pattern established throughout this project: modules with real prototype code (`audio/`, `dsp::pitch`, `dsp::pitch_shift`, `harmony::key`, `harmony::rules`) have concrete, tested-algorithm-backed designs, and their two confirmed real bugs (PSOLA octave-up, harmony range-fallback) are reflected directly in their error-handling interfaces rather than designed around optimistically. Everything else — `pitch_edit/`, `clip/`, `track/`, `mixer/`, `project/`, `export/`, `gui/` — is genuinely first-pass or stub-level design, with `mixer/` and `gui/` the least developed of all.
