# AcapellaStudio — Feasibility Study

**Document:** 03_feasibility_study.md
**Status:** IN PROGRESS — §3.2 (pitch detection) and part of §3.6 (ML dataset) have real results; §3.1, §3.3, §3.4, §3.5, and the rest of §3.6 are honestly marked PENDING, not fabricated.
**Goal (per master roadmap):** Determine what is technically achievable before committing implementation effort.

> **Critical constraint discovered at the start of this phase:** several roadmap items (§3.1 audio hardware, §3.2 real singing-voice testing, §3.3 pitch-shifting on real vocals) require a real microphone, real audio hardware, and real recordings. This sandboxed development environment has none of these. Per the operating charter's rule against fabricating measurements, those items are marked PENDING here rather than invented. What *can* be done without hardware — algorithm implementation, testing against synthetic/generated signals, and dataset/licensing research — has been done for real, with actual code and actual output, not simulated results.

---

# 3.1 Audio Feasibility — **PENDING (requires real hardware)**

Not yet tested. This requires:
- A real microphone connected to actual hardware (Michael's HP EliteBook, Kubuntu)
- Real measurement of input/output latency, buffer underrun/overrun behavior, and CPU usage under load

**Cannot be honestly completed in this sandboxed environment.** This is the natural next step for hands-on work on your own machine, one step at a time, per the operating charter.

---

# 3.2 Pitch Detection Feasibility — **PARTIALLY COMPLETE (synthetic signals only)**

## Method

Implemented the full YIN algorithm (de Cheveigné & Kawahara, 2002) from scratch in Rust — difference function, cumulative mean normalized difference function (CMNDF), absolute threshold, and parabolic interpolation for sub-sample accuracy — exactly as described in the Literature Review (§1.1). Code lives at `prototypes/yin-prototype/src/main.rs` in this repo.

Tested against **synthetically generated sine waves** at six frequencies spanning typical vocal range (E2 82Hz to C6 1046Hz), both clean and with additive noise, at 48kHz sample rate, 2048-sample buffers (~42.7ms).

## Real Results (actual program output, not estimated)

### Clean sine waves

| Note | True (Hz) | Detected (Hz) | Error (cents) |
|---|---|---|---|
| E2 (low bass) | 82.41 | 82.44 | +0.67 |
| A2 | 110.00 | 110.05 | +0.72 |
| A3 | 220.00 | 220.09 | +0.73 |
| A4 (concert pitch) | 440.00 | 440.19 | +0.76 |
| A5 | 880.00 | 876.37 | **−7.15** |
| C6 (high soprano) | 1046.50 | 1045.10 | −2.32 |

**Mean absolute error (clean): 2.06 cents**

### Sine waves with additive noise (amplitude 0.05)

| Note | True (Hz) | Detected (Hz) | Error (cents) |
|---|---|---|---|
| E2 | 82.41 | 82.44 | +0.69 |
| A2 | 110.00 | 109.90 | −1.59 |
| A3 | 220.00 | 220.12 | +0.96 |
| A4 | 440.00 | 440.23 | +0.92 |
| A5 | 880.00 | 876.33 | **−7.24** |
| C6 | 1046.50 | 1045.14 | −2.26 |

**Mean absolute error (noisy): 2.28 cents. Zero detection failures (0/6) at this noise level.**

### Processing time (2048-sample buffer, 1000 iterations, this container's CPU)

- Average time per YIN call: **0.78 ms**
- Buffer duration at 48kHz/2048 samples: 42.67 ms
- Processing time as % of buffer duration: **1.83%**

## Honest Findings

1. **A genuine anomaly, not smoothed over:** error is not monotonic across the frequency range — A5 (880Hz) shows a notably larger error (−7.15 to −7.24 cents) than both the note below it (A4, +0.76¢) and above it (C6, −2.32¢). This is a real observed result from real code, not expected or explained away. **This needs investigation** — possibly a windowing/period-to-buffer-size interaction specific to that frequency, or a limitation in this particular (unoptimized, first-pass) implementation. Flagged as an open item, not resolved here.
2. **Clean-signal accuracy (~2 cents mean) is good** relative to commonly-cited "just noticeable difference" thresholds for pitch (often cited around 5-10 cents depending on context) — promising for the correction use case, on synthetic signals.
3. **Noise robustness at this modest noise level is good** (0/6 failures, error increased only slightly) — but this is a simple additive-noise model, not realistic vocal noise characteristics (breathiness, sibilance, room noise).
4. **Processing cost is low** (<2% of buffer duration) even in this naive, unoptimized O(n·max_tau) implementation — strong headroom for a real-time budget, though this must be re-measured on the actual reference hardware, not this container.
5. **This says nothing yet about real singing voice.** Vibrato, breathiness, formants, pitch glides between notes, and consonant/unvoiced segments are not represented by clean sine waves. The literature review already flagged (§1.1) that even the original YIN paper's benchmark was on speech, not singing — this prototype doesn't close that gap, it just confirms the base algorithm is correctly implemented before testing it against something harder.

## What's Still Needed

- Test against **real recorded singing** (requires microphone + your voice) — the actual next step, not yet done
- Investigate the A5 anomaly specifically
- Compare against pYIN as planned in the Literature Review action items
- Re-measure processing time on actual reference hardware

---

# 3.3 Pitch-Shifting Feasibility — **PENDING**

Not yet prototyped. Requires either real vocal recordings to shift, or at minimum synthetic signal testing of a PSOLA or phase-vocoder implementation — a legitimate next coding task that doesn't strictly require a microphone (could test on the same synthetic sine waves used in §3.2, extended to shifted-pitch comparison), but has not been attempted yet in this pass.

---

# 3.4 Key Detection Feasibility — **PENDING**

Not yet prototyped, though notably **this is another item that doesn't strictly require a microphone** — Krumhansl-Schmuckler (Literature Review §4A.1) operates on a chromagram/pitch-class distribution, which could be tested against a synthetically generated note sequence (e.g. a known C-major melody) without needing real audio. This is a reasonable candidate for the next coding session.

---

# 3.5 Harmony Feasibility — **PENDING**

Not yet prototyped. Depends on having a working note/pitch sequence representation (related to §3.4's output) before a harmony-rule prototype makes sense to build.

---

# 3.6 ML Feasibility — **PARTIALLY COMPLETE**

## Dataset investigation — real findings

**JSB Chorales dataset** (already identified in Literature Review §4C.1), verified in more depth this pass:

- **382 Bach chorales total**, confirmed via the czhuang/JSB-Chorales-dataset GitHub repository and the original Boulanger-Lewandowski (2012) ICML paper split (used by DeepBach and multiple other academic systems).
- **Source material is public domain** (Bach's compositions) — the dataset itself is freely available on GitHub with no licensing barrier found in this pass.
- **Format: symbolic/MIDI note data, NOT audio.** Provided as CSV or pickled Python arrays, at multiple temporal resolutions (quarter, 8th, 16th note). Each timestep records up to 4 simultaneous MIDI note numbers (soprano/alto/tenor/bass).
- **Small dataset: ~215KB total** (per the MusPy documentation source) — genuinely lightweight, easy to work with, but also means it's a narrow, stylistically specific (four-part 18th-century chorale) dataset, not a general-purpose harmony corpus.
- **Directly usable for a symbolic "decide the harmony notes" model**, which matches our own preferred architecture (Problem Statement §6.8: symbolic decision → pitch-shift render) — but would need to be paired with our own separate rendering pipeline, since the dataset contains no audio.

## What's still needed

- A capella/pop-vocal-style dataset search (flagged already in Literature Review action items — not yet done)
- Investigation of whether a lead/harmony **audio-paired** dataset exists at all (JSB Chorales being symbolic-only means it can't directly train an audio-domain harmony model, only a note-decision model)
- Compute/inference requirements research for whichever model architecture gets chosen — not started, since no model architecture has been chosen yet

---

# 4. Algorithm Decision Matrix (Preliminary — Incomplete)

| Component | Candidate | Status | Real evidence so far |
|---|---|---|---|
| Pitch detection | YIN | Synthetic testing done | ~2 cents mean error (clean), <2% CPU/buffer-duration cost, on synthetic signals only. Singing-voice testing pending. |
| Pitch detection | pYIN | Not yet prototyped | Literature only (Literature Review §1.2) |
| Pitch shifting | PSOLA | Not yet prototyped | Literature only (Literature Review §2.1) |
| Pitch shifting | Phase vocoder | Not yet prototyped | Literature only (Literature Review §2.2) |
| Key detection | Krumhansl-Schmuckler | Not yet prototyped | Literature only; expected lower accuracy on monophonic input (Literature Review §4A.3) |
| Harmony (rule-based) | Yogev & Lerch (2008)-style pipeline | Not yet prototyped | Literature only (Literature Review §4.1) |
| ML dataset | JSB Chorales | Verified real, available, public domain | Symbolic only, small, stylistically narrow — usable but limited |

**This matrix is intentionally incomplete.** Filling it in fully requires either your hardware (audio, latency) or further coding sessions that don't need a microphone (pitch-shifting on synthetic signals, key detection on synthetic note sequences). Both paths are legitimate next steps.

---

# 5. Technical Risks Identified This Phase

- **New:** the A5 pitch-detection anomaly (§3.2) is an unexplained result in our own first-pass implementation — needs investigation before YIN can be considered validated even on synthetic signals, let alone real voice.
- Carried forward from Literature Review: YIN IP status unverified, US Patent 8,168,877/8,618,402 IP risk (R-014), no singing-specific benchmark found for any pitch detector, no study on correction-scale vs. harmony-scale shift quality, no a capella-specific harmonization dataset found.
- Carried forward from Competitive Analysis: possible Antares connection to R-014, needs patent-assignee verification.

---

# 6. Status

**This phase is not complete.** Real progress made on §3.2 (pitch detection, synthetic) and part of §3.6 (ML dataset investigation). §3.1, §3.3, §3.4, §3.5, and the rest of §3.6 remain genuinely pending — either because they need your physical hardware and voice, or because they're legitimate next coding tasks not yet attempted. The Algorithm Decision Matrix (§4) reflects this honestly rather than presenting a false sense of completeness.

**Recommended next step:** since §3.4 (key detection) and the remainder of §3.3 (pitch-shifting on synthetic signals) don't require a microphone, those are reasonable to continue in this same sandboxed environment. §3.1 and real-voice testing for §3.2/§3.3 need to happen on your machine, one step at a time, when you're ready to move to hands-on setup.
