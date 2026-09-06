# AcapellaStudio — System Architecture

**Document:** 06_architecture.md
**Status:** Draft v1 — Phase 6
**Predecessor documents:** 03_feasibility_study.md, 04_requirements.md, 05_system_analysis.md

> **A real, self-critical finding driving part of this design:** the Feasibility Study's prototypes used `std::sync::Mutex` inside the real-time audio callback (for the ring buffer and counters in the pitch-detection and endurance tests). This technically violates NFR-RT-005 ("no unbounded locks in the real-time path"), even though testing showed stable results (Feasibility Study §1.11, §2.11) — likely because contention was low in practice (effectively single-writer/single-reader), not because the design was actually real-time-safe by construction. This document does not carry that shortcut forward silently; §4.3 addresses it directly.

---

# 1. High-Level Component Diagram

Refined from the master roadmap's skeleton, with real component names grounded in what's been prototyped:

```text
                              AcapellaStudio
                                    │
                          ┌─────────┴─────────┐
                          │ Application Layer │
                          └─────────┬─────────┘
                                    │
        ┌───────────────┬──────────┼──────────┬───────────────┐
        │               │          │          │               │
        ▼               ▼          ▼          ▼               ▼
  Audio Engine    Harmony Engine  Project Engine  UI Layer   (Future: ML Engine,
        │               │          │              │           Phase 3+, not MVP)
        │               │          │              │
   ┌────┴────┐    ┌─────┴─────┐  ┌─┴──────┐  ┌───┴────┐
   │         │    │           │  │        │  │        │
   ▼         ▼    ▼           ▼  ▼        ▼  ▼        ▼
Audio I/O  DSP   Rule       Voice Track   Clip  Pitch    Mix
(cpal)   Engine  Engine    Manager Mgr   Mgr   Editor  Console
           │       │
      ┌────┴────┐  │
      ▼         ▼  ▼
   Pitch     Pitch  (produces symbolic
  Detector  Shifter  Note sequences,
  (YIN)     (PSOLA)  see §3.2)
```

## 1.1 Component-to-prototype mapping (honest status)

| Component | Real prototype exists? | Reference |
|---|---|---|
| Audio I/O | **Yes** — cpal + native PipeWire backend, real device enumeration, real buffer control | `prototypes/audio-io-prototype/` |
| DSP Engine → Pitch Detector | **Yes** — naive + FFT-optimized YIN, both validated | `prototypes/yin-prototype/`, `prototypes/yin-fft-prototype/` |
| DSP Engine → Pitch Shifter | **Yes, with a known bug** — TD-PSOLA, octave-up failure unresolved | `prototypes/psola-prototype/` |
| Harmony Engine → Rule Engine | **Yes, with a known bug** — diatonic-third harmonizer, range-fallback failure unresolved | `prototypes/harmony-prototype/` |
| Harmony Engine → Key detection | **Yes** — Krumhansl-Schmuckler, algorithm-verified | `prototypes/key-detection-prototype/` |
| Harmony Engine → Voice Manager | No — MVP is a single fixed voice, no manager needed yet | — |
| Project Engine (Track/Clip Mgr) | No | — |
| UI Layer (any part) | No | — |
| ML Engine | No — explicitly Phase 3+, not MVP | — |

---

# 2. Real-Time / Non-Real-Time Separation

Per the roadmap's stated principle and the Problem Statement's NFR-RT-005/007.

**Amended (Problem Statement §33, Amendment 2): automatic pitch correction has moved out of the real-time path entirely.** The diagram below reflects this — it is a real, substantive change from the version of this document originally drafted, not a relabeling.

```text
REAL-TIME PATH (deadline-critical — must complete within the buffer interval)

Microphone
   → Audio Callback (cpal)
   → Pitch Detection (YIN — validated: 28% CPU with FFT optimization, §2.11 — MAY still
      run here for live visual feedback, but no longer drives real-time correction)
   → Real-Time Mixing (NOT YET BUILT — for monitoring only; raw, uncorrected signal)
   → Output (raw, uncorrected monitoring — the singer hears themselves without correction)


NON-REAL-TIME PATH (may take longer than one buffer interval — runs POST-RECORDING)

Automatic Pitch Correction   (MOVED HERE from the real-time path — see rationale below.
                               NOT YET BUILT as a "detect + shift toward nearest scale
                               tone" pipeline; PSOLA shifting exists in isolation, §3)
Manual Pitch Editing        (NOT YET BUILT)
Clip Editing                (NOT YET BUILT)
Key Detection                (real algorithm exists — Krumhansl-Schmuckler is
                               inherently non-real-time; needs a full melody,
                               not per-buffer data)
Harmony Generation           (real algorithm exists — diatonic-third harmonizer
                               operates on a full melody, not per-buffer)
Harmony Audio Rendering       (PSOLA pitch-shifting the melody into harmony
                               audio — real in isolation, not yet wired to the
                               harmony decision output)
Project Save/Load             (NOT YET BUILT)
Export                        (NOT YET BUILT)
```

**Why this changed, and what it resolves:** the Feasibility Study found real-time round-trip latency (~15.76–16.00ms, §1.9) already exceeding the original ≤10ms corrected-monitoring target, and PSOLA had never been tested inside a live callback at all — its real-time feasibility was completely unknown, not just imperfect (this was the open question raised immediately below, in the original version of this note). Moving correction to a post-recording step removes both problems: raw monitoring only needs to solve the comparatively easier uncorrected-latency case, and PSOLA/harmony rendering gain unlimited processing time — meaningfully lowering technical risk for the MVP. This does not abandon real-time correction forever; it is deferred as a later-phase enhancement (Problem Statement §33), once post-recording correction is proven.

**What did NOT change:** pitch detection may still run in the real-time path for live visual feedback (e.g. showing the singer their pitch contour as they sing), and remains confirmed real-time-safe (Feasibility Study §2.11). Raw monitoring latency (NFR-RT-002) remains a real, still-open requirement — it did not go away, it's simply now the *only* real-time latency target that matters, rather than one of two.

---

# 3. Component Definitions

## 3.1 Audio Engine

**Responsibility:** own the real-time audio callback; own audio device I/O; host the deadline-critical DSP chain.
**Real basis:** `prototypes/audio-io-prototype/` — confirmed native PipeWire backend, confirmed 128-sample buffer honored, confirmed stable timing under both idle and real-DSP-load conditions.
**Known open issue, RESOLVED in Phase 11 (see Detailed Design §2.1 and Architecture §4.3):** the prototype's ring buffer used `std::sync::Mutex`, not real-time-safe by guarantee. Fixed using `rtrb` (wait-free SPSC ring buffer) in the real Phase 11 implementation.

## 3.2 DSP Engine

**Sub-component: Pitch Detector**
**Responsibility:** given an audio buffer, return a detected fundamental frequency (or none).
**Real basis:** two validated implementations exist — naive YIN (`prototypes/yin-prototype/`) and FFT-optimized YIN (`prototypes/yin-fft-prototype/`, verified mathematically identical to the naive version to within floating-point precision, Feasibility Study §2.9). The FFT version is the real-hardware-tested candidate for production (28% CPU, Feasibility Study §2.11), pending the plan-caching fix.

**Sub-component: Pitch Shifter**
**Responsibility:** given an audio buffer and a target pitch shift ratio, return pitch-shifted audio.
**Real basis:** `prototypes/psola-prototype/` — validated for pitch accuracy on 5/6 test ratios (correction-scale and most harmony-scale intervals), with a **confirmed, unresolved bug** at octave-up shifts (no output pitch detected at all). Formant preservation has never been tested (no formant-bearing test signal has been used). **This component is not production-ready.**

## 3.3 Harmony Engine

**Sub-component: Rule Engine**
**Responsibility:** given a melody (as detected notes) and a key, produce a symbolic harmony voice (sequence of notes).
**Real basis:** `prototypes/harmony-prototype/` — diatonic-third harmonization, real voice-leading check (though flagged as weak evidence given the technique's structure, §5.2), and a **confirmed, unresolved bug** in the range-fallback path (FR-008a — silently defaults to unison when no diatonic third fits the vocal range).

**Sub-component: Key Detector** (logically part of Harmony Engine's inputs, listed separately for clarity)
**Responsibility:** given a melody, estimate the most likely key/scale.
**Real basis:** `prototypes/key-detection-prototype/` — Krumhansl-Schmuckler, verified correct on 6/6 synthetic symbolic test cases. **Real singing-voice accuracy is untested** — this is the literature's identified, still-open gap (Literature Review §4A.3).

**Sub-component: Voice Manager**
**Responsibility:** manage multiple harmony voices (Phase 2+ scope — MVP is a single fixed voice, per the Problem Statement).
**Real basis:** none — not needed for MVP, not designed in detail here.

## 3.4 Project Engine

**Sub-components:** Track Manager, Clip Manager, Project Storage.
**Responsibility:** own the project data model (§5.1 of the System Analysis document — Project/Track/AudioClip/etc.), persistence, and coordination between tracks.
**Real basis:** none. This is entirely undesigned-in-detail and unimplemented. The domain model in `05_system_analysis.md` §5 is the closest thing to a specification that exists.

## 3.5 UI Layer

**Responsibility:** all user-facing interaction — recording controls, pitch-editing view, Harmony Wizard, mixer, export.
**Real basis:** none. No GUI framework has even been selected yet (deferred to Phase 7 — Technology Selection, per the roadmap).

---

# 4. Interface Definitions

## 4.1 Audio Engine ↔ DSP Engine

```text
trait PitchDetector {
    fn detect(&mut self, buffer: &[f32], sample_rate: f64) -> Option<f64>;
}

trait PitchShifter {
    fn shift(&mut self, buffer: &[f32], ratio: f64) -> Vec<f32>;
}
```

**Real basis:** these signatures are directly modeled on the actual function signatures already used in the prototypes (`yin_detect`, `psola_pitch_shift`), generalized into traits for the first time here — this is a genuine design step, not a restatement of existing code, though it is grounded in real, working function shapes rather than invented from nothing.

## 4.2 Harmony Engine ↔ Project Engine

```text
trait HarmonyGenerator {
    fn generate(&self, melody: &[Note], key: &Key, vocal_range: (Midi, Midi)) -> Result<Vec<Note>, HarmonyError>;
}

enum HarmonyError {
    NoValidHarmonyForRange { note_index: usize }, // maps directly to the REAL,
                                                    // CONFIRMED FR-008a defect -
                                                    // this variant exists specifically
                                                    // because we found this failure
                                                    // mode through real testing, not
                                                    // speculative error handling
}
```

**Real basis:** the `Result`/`HarmonyError` design is a direct, evidence-driven response to the actual failure found in Feasibility Study §5.3 — the prototype's silent-unison fallback is exactly the kind of failure this interface is designed to make impossible to ignore (the caller must handle `Err`, not receive a silently-degraded `Ok`).

## 4.3 Real-Time Safety — RESOLVED (Phase 11)

~~Per NFR-RT-005 and the honest finding at the top of this document: **the real Audio Engine's ring buffer and any cross-thread communication in the deadline-critical path must use lock-free primitives**, not `std::sync::Mutex`. Candidate approaches (not yet evaluated in detail — flagged as a Phase 7/8 decision, not resolved here):
- A single-producer/single-consumer lock-free ring buffer (e.g. via a dedicated crate, to be evaluated in Phase 7 — Technology Selection).
- Atomic-based counters (already used correctly in the prototypes via `AtomicU64` for callback/gap counting — that part of the prototypes' design *does* meet NFR-RT-005, only the `Mutex`-guarded ring buffer and shared state do not).~~

**Resolved in Phase 11 (Audio Engine Prototype):** the real `AudioEngine::start_recording()` implementation in `acapellastudio/src/audio/mod.rs` now uses `rtrb`, a wait-free SPSC ring buffer purpose-built for real-time audio, selected after verifying its documentation and design intent (not assumed). `Producer::push()` inside the real-time callback never blocks or allocates — this genuinely satisfies NFR-RT-005, not just in intent but in the actual shipped code. The `rtrb`-specific push/pop/full-buffer-rejection logic was verified in isolation; the full `cpal`-integrated version awaits real-hardware confirmation (same pattern as all DSP work in this project). Retained struck-through above for change history.

**This is an explicit, acknowledged gap between the feasibility prototypes and production-ready architecture** — the prototypes were correctly scoped as feasibility tests (per the operating charter's own framing throughout the Feasibility Study), not production code, and this document does not pretend otherwise.

---

# 5. Architecture Gaps (Explicit, Not Hidden)

## 5.1 Automatic Pitch Correction Is Not Designed Yet
**Amended (§33 amendment, now post-recording, not real-time):** no component for "find nearest scale tone given a detected frequency and a key" exists in any prototype. This is a real, unaddressed gap between validated components (pitch detection, PSOLA shifting) and the feature the Problem Statement's FR-005 actually requires — the gap itself is unchanged by the real-time-to-post-recording move; only the deadline constraint around eventually building it has relaxed.

## 5.2 Harmony Audio Rendering Is Not Wired to Harmony Decision-Making
The Rule Engine produces symbolic notes (validated). The Pitch Shifter can shift audio to a target pitch (validated, with the octave-up caveat). **These two have never been connected as one pipeline** — there is no real evidence yet that feeding the Rule Engine's output into the Pitch Shifter, on real melody audio, produces usable harmony audio.

## 5.3 Real-Time Placement of Pitch Shifting — RESOLVED by architectural change

~~As noted in §2, PSOLA has never been tested inside a live audio callback. Whether it's fast enough for the real-time correction path, at the buffer sizes and hardware already characterized (Feasibility Study §1), is completely unknown.~~

**Resolved, not by testing but by removing the requirement (Problem Statement §33, Amendment 2): pitch correction and harmony rendering no longer run inside the real-time callback at all**, so PSOLA's real-time speed is no longer a blocking question for the MVP. Retained, struck through, for change history — this was a genuine open risk that is now moot by design, not by evidence.

---

# 6. Status

Draft v1. This architecture is grounded in real prototype evidence wherever it exists, and explicitly flags the substantial portions (Project Engine, UI Layer, real-time correction logic, harmony-to-audio integration, and real-time-safe concurrency) that remain undesigned or unresolved. The component-to-prototype mapping table (§1.1) is the fastest way to see exactly how much of this diagram is backed by real, tested code versus specification alone.
