# AcapellaStudio — Technology Selection

**Document:** 08_technology_selection.md (roadmap Phase 7 — Technology Selection)
**Status:** Draft v1
**Predecessor documents:** 03_feasibility_study.md, 06_architecture.md

> **Standing rule for this document:** a decision is only marked "Selected" when real project evidence or real current research backs it. Where no such evidence exists yet, the decision is marked "Open" rather than filled in with a plausible-sounding default.

---

# 1. Language — Rust

**Decision: Selected.**
**Evidence:** the entire Feasibility Study (03) was built and tested in Rust, on the actual reference hardware, across five working prototypes (audio I/O, YIN, YIN-FFT, PSOLA, key detection, harmony generation). This isn't a hypothetical choice being validated after the fact — it's already load-bearing for everything demonstrated so far. No alternative was seriously evaluated in this project (the original rationale — memory safety without GC, real-time suitability — was established in the Project Vision/Problem Statement phase, before any code existed).

---

# 2. Audio Backend

**Decision: Selected — cpal with the native PipeWire backend.**
**Evidence:**
- Real confirmation that cpal's `pipewire` feature selects the native PipeWire host over the ALSA fallback (Feasibility Study §1.2: `Available audio hosts: [PipeWire, Alsa]` / `Using host: PipeWire`).
- Real confirmation of a working, honored 128-sample fixed buffer (§1.5).
- Real round-trip latency data (§1.9, §1.11) and real CPU/timing stability data under both idle and DSP-load conditions.
- JACK, WASAPI, ASIO, and Core Audio were not evaluated — no evidence exists for or against them in this project, and per the Problem Statement's own scoping (§17, "prioritize native desktop execution" on one initial OS), this is consistent with the current single-OS (Kubuntu/Linux) target, not a cross-platform decision.

**Explicitly out of scope for now:** Windows (WASAPI/ASIO) and macOS (Core Audio) backend selection — no evidence exists, and per the Problem Statement, cross-platform support is not claimed until validated (§17, item 13). This should be revisited only when/if the project's OS scope expands.

---

# 3. DSP

## 3.1 Pitch Detection — Selected, with a caveat

**Decision: Selected — FFT-optimized YIN.**
**Evidence:** validated mathematically identical to naive YIN (Feasibility Study §2.9, differences at the 10⁻¹² floating-point-noise level), real-hardware-tested at 28% CPU during live real-time operation (§2.11), and validated for accuracy on a controlled reference tone (0.99 cents mean error, §2.7).
**Caveat, not hidden:** the 28% CPU figure includes a known, unfixed inefficiency (per-callback `FftPlanner` creation, §2.11) — the plan-caching fix was designed (§2.13 in the earlier session, code updated) but not yet re-measured on real hardware. **The final CPU figure for this decision is not yet confirmed.**
**Alternatives considered but not tested:** pYIN and SwiftF0 were identified in the Literature Review (§1.2, §1.4) as candidates but never implemented or compared against our own YIN implementation on real singing voice. This selection is therefore provisional in the sense that a lighter/more accurate alternative has not been ruled out — it simply hasn't been tested.

## 3.2 Pitch Shifting — Open, not yet selected

**Decision: Open.** TD-PSOLA is the only pitch-shifting approach implemented so far, and it has a **confirmed, unresolved bug** (Feasibility Study §3.2/§3.3: complete detection failure at octave-up shifts) and **zero formant-preservation testing** (the only test signal used had no formants at all). This does not meet the bar for "Selected" — it's the leading, most-tested candidate, but selecting it now would overstate the evidence.
**Phase vocoder** (Literature Review §2.2) remains an untested alternative.
**Action required before this can be marked Selected:** fix the octave-up bug, build a formant-bearing test signal, and re-test — per Feasibility Study §3.4's own action items, none of which have been completed yet.

## 3.3 Time Stretching — Open, not yet started

**Decision: Open.** No time-stretching work has been done in this project at all — not researched, not prototyped. This is listed in the roadmap's Phase 7 scope but has no evidence base whatsoever. Flagged as a real gap, not silently skipped.

## 3.4 Formant Preservation — Open, not yet started

**Decision: Open.** As stated in the Architecture document (06, §5.3) and Feasibility Study (§3.3), **zero testing has been done on formant preservation** — every pitch-shift test so far used a pure sine wave, which has no formants. This is arguably the single most important open technical question for harmony-scale shifting specifically (per the Literature Review §2.3 and Problem Statement R-010), and it remains completely untouched.

---

# 4. GUI Framework

**Decision: Iced is the leading candidate — not yet finalized, no prototype built.**

**Real research findings (this session, current sources):**
- `iced_audio` is a real, existing extension crate providing pre-built audio-specific widgets, referenced directly in a maintained audio-plugin-framework reference list (github.com/BillyDM/awesome-audio-dsp). This is directly relevant to AcapellaStudio's actual UI needs (pitch-contour editing, mixer controls) — egui has no equivalently-named, purpose-built audio-widget extension in the sources found.
- egui is immediate-mode (redraws every frame); a 2025 comparative review notes it "can become CPU-hungry with many redraws." **This is a real, project-specific concern**, not a generic style preference — the Feasibility Study already found the DSP path alone consuming meaningful CPU (28% for pitch detection); an immediate-mode GUI competing for the same CPU budget is a real risk worth weighing, not just a taste question.
- Iced is retained-mode/reactive (Elm-inspired), described in current (2026) sources as having "a more native feel than egui but requires more setup," with some noted documentation gaps.
- A January 2026 comparative benchmark (Medium, "why I chose Electron") found real problems across egui, Iced, Slint, and GTK for a heavy-data-table desktop tool and ultimately chose Electron instead — **not directly analogous to our use case** (a data-table-heavy business app is a different UI demand profile than an audio pitch-editor), but a real caution that none of these frameworks are problem-free.

**Why this is "leading candidate," not "Selected":** none of egui, Iced, or any alternative has actually been prototyped in this project. The research above is real and current, but it's desk research, not our own tested evidence — consistent with this document's standing rule, that distinction is being preserved rather than blurred into a final decision.

**Action required before this can be marked Selected:** build a minimal prototype of the pitch-contour visual editor (the most demanding, most custom-widget-dependent UI piece) in both Iced and egui, and compare real development friction and rendering behavior — not just desk research.

---

# 5. Machine Learning Framework — Deferred, not evaluated

**Decision: Deferred, per the Problem Statement's own MVP scope (ML harmony generation is explicitly Phase 3+, not MVP).** ONNX Runtime, Candle, and Burn are named in the roadmap as future candidates but have not been researched at all in this project. No decision, provisional or otherwise, exists yet — this is intentionally left blank rather than pre-filled with a guess this far ahead of need.

---

# 6. Project Storage Format

**Decision: Proposed — directory-based project structure. Not yet implemented or tested.**

Based directly on the domain model already specified (System Analysis §5.1 — Project/Track/AudioClip/PitchContour/HarmonyArrangement) and the roadmap's suggested structure:

```text
ProjectName.acappella/
├── project.json          (metadata: sample rate, key, tracks list)
├── tracks/
│   └── <track-id>.json   (track type, volume/pan/mute/solo state)
├── recordings/
│   └── <clip-id>.wav     (source audio, untouched — non-destructive editing
│                           requires this to never be modified in place,
│                           per NFR-AUDIO-004)
├── edits/
│   └── <clip-id>.json    (trim/fade/gain state, applied on top of the
│                           untouched source at playback/render time)
├── pitch/
│   └── <clip-id>.json    (detected pitch contour + any manual overrides,
│                           per FR-006/Problem Statement R-007)
└── generated/
    └── <harmony-id>.wav  (rendered harmony audio — regenerable from
                            recordings/ + pitch/ + harmony decision data,
                            not treated as a irreplaceable source)
```

**Rationale:** a plain directory (not a single opaque binary/zip file) keeps source recordings directly accessible as standard WAV files (useful for recovery, external tool interop, and matches the "original recording remains recoverable" requirement, NFR-AUDIO-004, literally rather than just logically) and is straightforward to version with git if a user wanted to (consistent with this project's own working style).
**Honest status:** this is a reasonable, evidence-adjacent design decision (grounded in the real domain model), but it has not been implemented or tested — no project save/load code exists yet (Architecture §3.4 already flagged the Project Engine as entirely undesigned in detail).

---

# 7. Technology Decision Matrix

| Area | Decision | Status | Real evidence? |
|---|---|---|---|
| Language | Rust | **Selected** | Yes — 5 working prototypes |
| Audio backend | cpal + native PipeWire | **Selected** (Linux/Kubuntu only) | Yes — real hardware tests |
| Pitch detection | FFT-optimized YIN | **Selected, with open CPU re-measurement** | Yes, mostly — plan-caching fix unconfirmed |
| Pitch shifting | TD-PSOLA (leading candidate) | **Open** — confirmed bug, zero formant testing | Partial — real bug found, real gap remains |
| Time stretching | — | **Open** — no work done | No |
| Formant preservation | — | **Open** — no work done | No |
| GUI framework | Iced (leading candidate) | **Open** — desk research only, no prototype | Research-based, not project-tested |
| ML framework | — | **Deferred** — not MVP scope | No, intentionally |
| Storage format | Directory-based project structure | **Proposed** — not implemented | Design-grounded, not tested |

---

# 8. Status

Draft v1. Of the roadmap's Phase 7 scope (Language, Audio, DSP, GUI, ML, Storage), only **Language** and **Audio backend** meet the bar for "Selected" on real project evidence. Pitch detection is close, pending one open re-measurement. Everything else — pitch shifting, time stretching, formant preservation, GUI, storage — remains open, proposed, or deferred, each for a stated, specific reason rather than by omission.
