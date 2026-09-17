# AcapellaStudio — Project Status

**Current phase:** Phase 12 — Pitch Detection (bug found+fixed, first real post-fix benchmark
data recorded)
**Next phase:** Continue Phase 12 (remaining 6 test conditions: registers, vibrato, quiet/
loud, breathiness, background noise) before Phase 13, per master roadmap

## REAL VIBRATO TEST RESULTS — synthetic validation is decisive, live tests remain ambiguous

Two live attempts (97.5% detection / 261-594Hz range; 57.6% detection / 285.3¢ std dev / 8.68Hz
estimated rate) — Michael confirmed attempt 2 wasn't a controlled single-note hold. Neither
attempt cleanly resolved whether earlier oddities were performance or algorithm issues.

**Resolved via synthetic ground-truth testing (pure signal processing, fully verified in
sandbox, no hardware needed):** built `vibrato_synthesis_test.rs` — a sine wave with a KNOWN
programmed vibrato rate/depth. Real result across 4 test cases (440Hz/5.5Hz/80¢, 220Hz/6Hz/60¢,
440Hz/5.5Hz/250¢, 330Hz/4Hz/100¢): **100% detection, ZERO octave errors, rate recovery error
under 1% in every case.** This is decisive: YIN correctly tracks vibrato on a clean signal,
including at the same 250-cent depth seen in the ambiguous live test — strongly pointing to the
live-test oddities being a performance/recording issue, not a YIN algorithm bug. Real-voice
controlled vibrato testing remains open (this synthetic result rules out the algorithm as
primary suspect, doesn't prove real-voice tracking is equally clean). Full details:
`docs/09_dsp_design.md` §2.4.

## REAL BUG FOUND AND FIXED: stereo interleaving corrupted pitch analysis

The very first real run of `pitch_benchmark.rs` (normal_singing, 15s) surfaced a genuine bug,
not just noisy results:
- Reported "30.29s of audio" for a requested 15-second recording — exactly 2x, a real symptom
- Detected frequency range (60.1-102.1Hz) was suspiciously narrow/low for normal singing
- Detection rate only 13.2%, well below the ~94% seen in earlier live-mic prototype testing

**Root cause:** the input device is stereo (2 channels). `AudioEngine::start_recording()`
(rewritten in Phase 11) pushed raw interleaved L/R samples into the ring buffer with no
mono-mixing — a regression from the original live-mic YIN prototype, which correctly
mono-mixed before analysis. `pitch_benchmark.rs` then fed this raw interleaved data directly
into a mono pitch detector, corrupting the signal and doubling the apparent sample count.

**Fix:** added `RecordingHandle::drain_available_mono()` (uses a new, independently-verified
pure function `mono_mix()`) that correctly averages/de-interleaves multi-channel input.
`pitch_benchmark.rs` updated to use it. `drain_available()` (raw, un-mixed) is kept for
callers that genuinely want the original channel layout.

**Real verification:** `mono_mix()` isolated and tested in the sandbox (zero external deps) —
3/3 tests pass: stereo averaging correctness, mono passthrough, and correct handling of a
trailing partial frame (dropped, not corrupted). NOT yet re-verified against real stereo
hardware with a real re-run of the benchmark — that's the immediate next step.

## Real, new code this session

### Pitch pipeline gaps closed (real, testable without a mic)
- `PitchResult` struct: frequency + genuine confidence + MIDI note — closes the aspirational
  "confidence field, not implemented" gap from Detailed Design §2.2
- Confidence is REAL, not invented: derived directly from YIN's own internal CMNDF value at
  the selected tau (`1.0 - d_prime[tau]`), grounded in what the algorithm already computes
- `frequency_to_midi_note()` — standard 12-tone equal temperament formula, independently
  verified against 4 known reference points (A4=69/440Hz, Middle C=60/261.63Hz, A3=57/220Hz,
  A5=81/880Hz) before being added to the codebase — all matched to within 0.01
- 3 new real unit tests added: MIDI conversion against reference points, high-confidence
  on a clean tone, cross-check that midi_note corresponds to the detected frequency

### Benchmark harness (real infrastructure, not yet run under real conditions)
- `examples/pitch_benchmark.rs` — records a labeled test condition for N seconds, runs the
  real YinFftDetector over a sliding window, logs per-window results (frequency, confidence,
  MIDI note, detection success) to a CSV file, prints summary stats
- This directly supports roadmap Phase 12's explicit test list: different singers, different
  registers, vibrato, quiet singing, loud singing, breathiness, background noise
- **NOT YET RUN under any of these real conditions** — this is real, working infrastructure
  for gathering the data, not the data itself. Running it is the next real step.

## Sandbox note
- The sandbox environment reset entirely between sessions (lost repo + Rust toolchain).
  Recovered by re-cloning from GitHub (confirmed as the authoritative source) and
  reinstalling Rust. No project work was actually lost — Michael's local machine had the
  most recent state (multitrack commit) that hadn't yet reached the sandbox's stale clone.

## Real, open gaps still remaining (updated cumulative list)
- Phase 12's real varied-condition benchmark data has not been gathered yet (harness exists,
  data doesn't)
- mixer/ has real basic logic, but no per-track volume/pan/mute/solo application yet
- Automatic pitch correction ("nearest scale tone" logic) still has no design or prototype
- Harmony Rule Engine output never wired to the Pitch Shifter as one pipeline
- Formant preservation: zero testing exists anywhere in the project
- GUI framework: zero prototyping exists
- project/, export/, clip/, track/, pitch_edit/: still stub-only, no real logic
- Sample rate testing: only 48kHz has ever been used in any real test; 44.1kHz never tested
- No real endurance/CPU test has been run with the rtrb-based recording path

## Frozen / complete (docs)
- 00_project_vision.md, 00b_project_roadmap.md, 01_problem_statement.md (frozen v6 + Amendments 1-2)
- 02_literature_review.md, 02b_competitive_analysis.md — COMPLETE
- 03_feasibility_study.md, 04_requirements.md, 05_system_analysis.md, 06_architecture.md,
  07_detailed_design.md, 08_technology_selection.md, 08b_ui_ux_design.md — all Draft v1+

## Repo status
- Pushed to GitHub: https://github.com/michaellusias/acapellastudio
- Working copy: ~/Documents/acapella-daw/acapellastudio on Michael's machine
- Phase 11 (Audio Engine Prototype) COMPLETE, fully verified: 13/13 tests passing on real
  hardware, including recording, playback, and basic multitracking

## Operating rules in effect
- One step at a time for any command-line/setup instructions
- No fabricated papers, datasets, benchmarks, or measurements — ever; every claim sourced,
  actually run, or flagged as unverified/not-yet-measured/open
- Real academic/primary sources required for technical claims
- Real-time audio callback isolation rules apply from Phase 1 onward
- Git commit convention: feat/fix/test/docs/refactor, descriptive messages
- Every significant requirement traceable: Requirement → Design → Component → Implementation → Test → Result
