# AcapellaStudio — Feasibility Study

**Document:** 03_feasibility_study.md
**Status:** In progress — §2 (Pitch Detection) has real synthetic-signal results; §1, §3, §4, §5 require real hardware/microphone/singing-voice recordings and are not yet started; §6 (ML dataset feasibility) is desk research and can proceed independently.

> **Standing rule:** this document contains only results that were actually produced by running real code or performing real searches in this session. Nowhere does it contain invented measurements. Sections requiring a physical microphone, real singing-voice recordings, or Michael's actual reference hardware are explicitly marked NOT YET TESTED rather than filled in with plausible-sounding numbers.

---

# 1. Audio Feasibility (Microphone input, buffers, backend, latency, CPU, dropouts)

**Status: NOT YET TESTED.**

This requires a real microphone, real audio backend (PipeWire/JACK on Kubuntu), and Michael's actual HP EliteBook hardware. None of this can be honestly tested in this sandboxed development container, which has no audio input device. This is the natural point where work moves from documentation/prototyping-on-synthetic-data to hands-on setup on Michael's own machine, one step at a time, per the operating charter.

**What this section will require when we get there:**
- Installing Rust audio I/O crate (`cpal`) on the actual dev machine
- Confirming PipeWire/JACK availability and configuration on Kubuntu
- Opening a real input stream and measuring actual round-trip latency at various buffer sizes (64/128/256/512 samples)
- Measuring actual CPU usage and checking for real buffer underruns/overruns
- Documenting the actual reference hardware per NFR-RT-010 (CPU model, cores, RAM, audio interface — built-in laptop mic/output initially, presumably)

---

# 2. Pitch Detection Feasibility

**Status: Partial — synthetic-signal prototype complete and run for real. Singing-voice testing NOT YET DONE (requires a microphone/recordings).**

## 2.1 What was built

A real YIN implementation in Rust, following de Cheveigné & Kawahara (2002) exactly: difference function → cumulative mean normalized difference function (CMNDF) → absolute threshold with local-minimum search → parabolic interpolation for sub-sample tau accuracy. Source: `prototypes/yin-prototype/src/main.rs`. This is a naive, unoptimized O(n × max_tau) implementation — a correctness prototype, not production-quality code.

## 2.2 What was tested (real, run in this session)

Synthetic sine waves at six frequencies spanning the vocal range (E2 82Hz low bass through C6 1047Hz high soprano), sample rate 48kHz, buffer size 2048 samples (~42.7ms), threshold 0.10 (the paper's standard value).

**Test 1 — Clean sine waves, no noise:**

| Note | True (Hz) | Detected (Hz) | Error (cents) |
|---|---:|---:|---:|
| E2 | 82.41 | 82.44 | +0.67 |
| A2 | 110.00 | 110.05 | +0.72 |
| A3 | 220.00 | 220.09 | +0.73 |
| A4 | 440.00 | 440.19 | +0.76 |
| A5 | 880.00 | 876.37 | **-7.15** |
| C6 | 1046.50 | 1045.10 | -2.32 |

Mean absolute error: **2.06 cents**.

**Test 2 — Same tones with additive synthetic noise (amplitude 0.05, deterministic pseudo-noise, not a calibrated SNR figure):**

Mean absolute error: **2.28 cents**. Zero detection failures across 6/6 tones.

**Test 3 — Processing time (2048-sample buffer, A4=440Hz, 1000 iterations, this container's CPU):**

Average: **0.82ms per call**, or **1.92% of the 42.7ms buffer duration** at this buffer size.

## 2.3 Honest interpretation — what this does and does not tell us

**What it tells us:**
- The algorithm is correctly implemented — it converges to accurate frequency estimates on clean synthetic signals, consistent with the literature's own claims about YIN's basic accuracy (Literature Review §1.1).
- Mild additive noise barely degrades accuracy on synthetic tones — a reasonable initial sign, not a conclusion about real-world robustness.
- Even this naive, unoptimized implementation processes a buffer in under 1ms — comfortably within real-time budgets at commonly-used buffer sizes, on this container's CPU. This is a promising sign but must be re-measured on the actual reference hardware before being treated as a real performance number (NFR-RT-010).

**What it does NOT tell us, and must not be assumed:**
- **Real singing voice is not a sine wave.** It has vibrato (periodic pitch modulation), breathiness (aperiodic noise mixed with the periodic signal), formants (resonance peaks that don't affect pitch but affect the waveform shape), and onset/offset transients. None of this is represented by a clean or lightly-noised sine wave. The literature review already flagged this exact gap (§1.1: YIN's own benchmark was on speech, not singing).
- **The A5 anomaly (-7.15 cents) is unexplained and needs investigation, not dismissal.** My own working hypothesis — not sourced, just my reasoning, and explicitly flagged as such — is that at higher frequencies, the pitch period (tau, in samples) is shorter, so the same absolute quantization/interpolation error becomes a larger proportional (cents) error. This needs to actually be checked (e.g., by testing more buffer sizes and more frequencies to see if the error scales the way that hypothesis predicts) rather than accepted as an explanation.
- **This CPU timing is meaningless for our actual acceptance decision** until re-measured on Michael's HP EliteBook, per NFR-RT-010's explicit requirement for a documented reference hardware configuration.

## 2.4 Carried-forward action items

1. Re-run this exact prototype on the actual reference hardware once we're doing hands-on setup, to get a real CPU-timing baseline (not this container's).
2. Investigate the A5 accuracy anomaly with additional buffer sizes/frequencies before accepting or rejecting the current implementation.
3. Once microphone input exists (§1), test against **real recorded singing** — this is the actual test that matters and cannot be substituted with more synthetic-signal testing, however good the synthetic numbers look.
4. Compare against pYIN and SwiftF0 (per Literature Review §1.2/§1.4) using the same real singing-voice recordings, once available — not before.

---

# 3. Pitch-Shifting Feasibility

**Status: NOT YET TESTED.** Requires a real audio signal (recorded voice or at minimum recorded/synthesized harmonic material more complex than a sine wave) and a PSOLA or phase-vocoder implementation, neither of which exist yet. This is next in line for prototyping once §2's singing-voice testing is underway, since pitch-shifting quality evaluation is most meaningful when done on the same real vocal material pitch detection will be tested against.

---

# 4. Key Detection Feasibility

**Status: NOT YET TESTED.** Requires real recorded melodies (not synthetic tones) to build a genuine chromagram and test Krumhansl-Schmuckler correlation, per Literature Review §4A. This is the section most directly informed by our own identified literature gap (no study found benchmarking key detection on sparse monophonic vocal input specifically) — meaning this prototype, when built, will be producing a genuinely novel data point, not confirming an existing published number.

---

# 5. Harmony Feasibility

**Status: NOT YET TESTED.** Depends on §2 (pitch detection working on real melodies) and §4 (key detection) being further along first, per the roadmap's stated dependency (melody + key + range → harmony notes). Premature to prototype before those inputs are real rather than synthetic.

---

# 6. ML Feasibility — Dataset Investigation

**Status: Complete for this pass.** Real desk research, all sources checked and cited below.

## 6.1 Symbolic (non-audio) harmonization datasets

**JSB Chorales dataset** (Boulanger-Lewandowski, Vincent, & Bengio, 2012; hosted at github.com/czhuang/JSB-Chorales-dataset)

- ~382 four-part J.S. Bach chorales (per multiple citing sources), in symbolic/MIDI/piano-roll format, with explicit SATB voice separation (`Jsb16thSeparated.npz`/`.json` — one pitch per voice per timestep, silence marked distinctly).
- **Licensing: not explicitly stated as a formal license in any source checked in this pass.** The underlying music (Bach, d. 1750) is public domain, and the dataset compilation itself is widely re-hosted (GitHub, MusPy, CoCalc, multiple ML course repos) without an accompanying LICENSE file found. **This should not be assumed "freely usable for any purpose" — treat as "widely used in academic ML research without apparent restriction, but formal licensing status unconfirmed" until directly verified**, per the operating charter's "never invent" rule applied to licensing claims specifically.
- **Real limitation for our purposes, previously flagged in the Literature Review: this is symbolic data only — no audio.** It tells us nothing about audio quality, timbre, or vocal-specific rendering; it's only useful for training/testing symbolic harmony-decision logic (which chord/voicing to choose), which is a real but partial answer to R-008.

**Bach Choral Harmony dataset** (UCI ML Repository, id 298)

- Smaller: 60 chorales, 5665 chord-labeled events, MIDI-derived pitch-class + chord-label annotations. Same public-domain-source, unconfirmed-formal-license situation as above. Same audio-vs-symbolic limitation.

## 6.2 Real audio multitrack a capella datasets — genuinely closer to our actual need

This corrects/upgrades the Literature Review's Pass-1 stated gap ("no dedicated a capella-specific harmonization literature found") — real a capella **audio** multitrack datasets do exist:

**JaCappella corpus** (Hugging Face: `jaCappella/jaCappella`)

- 35 songs, 34 minutes total audio, **6 aligned vocal stems per song: Lead Vocal, Soprano, Alto, Tenor, Bass, Vocal Percussion** — this is architecturally almost exactly the lead+harmony+percussion stem alignment our MVP and Harmony Wizard vision would want for training or evaluation material.
- Real limitations, stated plainly: Japanese-language, arranged from out-of-copyright Japanese children's songs (narrow genre/style, not general pop/contemporary a capella), quite small (34 minutes — the SepACap paper found in this pass had to apply heavy data augmentation, expanding it to 105k samples/145 hours, to make it usable for their own separation-model training). 48kHz mono WAV, downloadable directly via `huggingface-cli`.
- **License:** not confirmed in this pass — needs direct verification on the Hugging Face dataset card before any use.

**Dagstuhl ChoirSet (DCS)**

- 55 minutes, amateur vocal ensemble, 2 full choir pieces (SATB) plus vocal exercises (scales, long notes, chords, cadences, intonation drills) from a real pedagogical source (Alldahl's *Choral Intonation*). Real multitrack recordings with per-voice stems.
- Published as a proper MIR research dataset (TISMIR journal, peer-reviewed) — more academically documented than JaCappella in terms of methodology, but Western classical choral style specifically, not pop/contemporary a capella either.
- The exercise recordings (scales, long notes, intonation drills) could be genuinely useful as **simple, controlled test material for our own pitch-detection and key-detection prototyping** (§2, §4) even before we get to harmony-generation training — worth considering as an actual near-term resource, not just a future ML dataset.

**Vocal92**

- 146.73 hours of real solo a cappella singing (not harmony-stem-aligned — built for singer recognition, not harmonization). ~92 amateur singers, multiple songs each, WAV/MP3/M4A, 48kHz-ish sampling.
- **Not directly useful for harmony-pair training** (no aligned harmony stems), but potentially a genuinely useful large source of **real solo singing audio for testing our own pitch-detection algorithm** (§2) against real vibrato/breathiness/noise, rather than only our current synthetic sine-wave tests. Worth flagging as a near-term resource for Feasibility Study §2, separate from its original singer-recognition purpose.

## 6.3 Honest assessment against R-008

**R-008 is now partially, not fully, resolved by this research:**
- Real, aligned lead+harmony **audio** stem datasets do exist (JaCappella, Dagstuhl ChoirSet) — this is better evidence than what the Literature Review found in Pass 1, which only turned up symbolic (non-audio) datasets.
- **But none found are a good genre/scale match for AcapellaStudio's actual target use case** (contemporary pop/vocal a capella, English or genre-agnostic, larger scale). JaCappella is Japanese children's songs; Dagstuhl ChoirSet is Western classical choral repertoire. Both are small (34–55 minutes).
- If ML harmony generation is pursued in Phase 3+ per the roadmap, **realistic options are:** (a) use these existing datasets for initial technique validation/prototyping despite the genre mismatch, accepting that a production model would need genre-matched data; (b) investigate constructing a small custom dataset (as the Problem Statement's R-008 mitigation already anticipated), now with a clearer sense of what "aligned multitrack a capella audio" actually looks like structurally, since we've seen two real examples of how it's organized (per-voice stem WAVs, consistent 48kHz sampling); or (c) treat rule-based harmony as the sole shipping method, per the Problem Statement's explicit fallback position.

## 6.4 Action items carried forward

1. Directly verify licensing terms for JaCappella (Hugging Face dataset card) and JSB Chorales/Bach Choral Harmony before any actual use — "widely used without apparent restriction" is not the same as a confirmed license.
2. Consider Vocal92 and/or Dagstuhl ChoirSet's exercise recordings as a real near-term resource for testing pitch detection (§2) against actual singing voice, once we're past synthetic-signal-only testing — this is a genuinely faster path than waiting entirely on Michael's own recordings, though his own voice should still be tested too since it's the actual target use case.
3. If Phase 3+ ML harmony work begins, budget real time for the likely need to either accept a genre mismatch initially or construct custom data — don't assume JaCappella/Dagstuhl ChoirSet are "good enough" without evaluating that mismatch's actual impact first.

---

# 7. Feasibility Summary So Far

| Area | Status | Real evidence produced? |
|---|---|---|
| Audio I/O feasibility | Not started | No — needs Michael's hardware |
| Pitch detection | Partial | **Yes** — real synthetic-signal test, real code, real numbers, real unexplained anomaly flagged |
| Pitch shifting | Not started | No — needs real audio + PSOLA/vocoder implementation |
| Key detection | Not started | No — needs real melody recordings |
| Harmony generation | Not started | No — depends on above |
| ML dataset feasibility | **Complete for this pass** | **Yes** — real datasets identified (JaCappella, Dagstuhl ChoirSet, Vocal92, JSB Chorales, Bach Choral Harmony), genre/scale mismatches honestly assessed, licensing flagged as unconfirmed rather than assumed |

**This document is intentionally incomplete rather than padded with plausible-sounding placeholder numbers.** Per the operating charter, an honest partial Feasibility Study is the correct output at this point — not a complete-looking document built on invented data.

---

# 8. Recommended Next Step

One track (§6) is now done for this pass. The remaining path runs through hands-on hardware setup:

**Hands-on hardware setup** (§1) — moving to Michael's actual Kubuntu machine, one step at a time, to get real microphone input working via `cpal` + PipeWire/JACK, which unblocks §1, and then real singing-voice recording, which unblocks completing §2, and eventually §3–§5. Vocal92 and Dagstuhl ChoirSet's exercise recordings (§6.2) could supplement Michael's own recordings for pitch-detection testing specifically, once we're ready for that step.
