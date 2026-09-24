# AcapellaStudio — Project Status

**Current phase:** Phase 12 — Pitch Detection (6 of 7 real test conditions
gathered: registers, vibrato, quiet, loud, breathiness, background noise;
different_singers deferred). Algorithm Decision recorded below.
**Next phase:** Either Phase 13, or compare YIN against pYIN/SwiftF0 per
NEXT_STEPS.md item 2 (motivated directly by the breathiness/low-register
findings below) — decision pending.

## Phase 12 Algorithm Decision

**Pitch Detection Module:** `src/dsp/pitch.rs` — FFT-accelerated YIN,
threshold 0.10 (de Cheveigné & Kawahara 2002), real-hardware-verified at
28% CPU (Feasibility Study §2.11).

**Benchmark Results summary** (full per-condition detail below):

| Condition | Detection | Confidence | Notes |
|---|---|---|---|
| Quiet singing | 97.1% | 0.984 | Clean, tight range |
| Loud singing | 97.5% | 0.981 | Clean, tight range |
| Vibrato (synthetic) | 100% | — | <1% rate error, fully verified |
| Vibrato (real) | inconclusive | — | 2 attempts, neither controlled |
| Breathiness | **76.7%** | 0.951 | Clear octave errors |
| Background noise | 96.4% | 0.974 | Detection OK, pitch instability |
| Register (high) | 98.8% | 0.969 | Clean |
| Register (low) | 88–97% | 0.945–0.951 | Weaker than high; 1 octave error |
| Different singers | — | — | Deferred |

**Decision:** YIN is **adequate for clean, controlled input** — quiet,
loud, and high-register sustained singing all land consistently at
97–99% detection with ~0.97–0.98 confidence, and synthetic vibrato
tracking is fully verified (100% detection, <1% rate error). YIN is
**not adequate as a standalone solution** for two conditions that are
realistically expected in actual a cappella recording: breathy vocal
technique (76.7% detection, clear octave-error pattern) and low-register
singing (weaker confidence, one confirmed octave error). Both failure
modes match YIN's documented weaknesses — sensitivity to aperiodic
noise, and fewer waveform cycles per analysis window at low frequency.

Per the roadmap's own branch ("If YIN is inadequate, evaluate
alternatives"), this is a **conditional pass**: ship YIN as the
baseline pitch detector, but evaluate pYIN or SwiftF0 specifically
against the breathiness and low-register cases before considering
pitch detection fully closed. Real-voice vibrato and different-singer
testing remain open and should be folded into that evaluation.

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

### Benchmark harness (real infrastructure)
- `examples/pitch_benchmark.rs` — records a labeled test condition for N seconds, runs the
  real YinFftDetector over a sliding window, logs per-window results (frequency, confidence,
  MIDI note, detection success) to a CSV file, prints summary stats
- This directly supports roadmap Phase 12's explicit test list: different singers, different
  registers, vibrato, quiet singing, loud singing, breathiness, background noise
- Now run under all conditions except different_singers (deferred) — see Algorithm Decision
  above and per-condition results below

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
trailing partial frame (dropped, not corrupted). Re-verified against real stereo hardware via
the full benchmark run set below.

## Phase 12 benchmark results (real)

### quiet_singing
`cargo run --release --example pitch_benchmark -- quiet_singing 15`
- 5664 windows, 5502 detected (97.1%), mean confidence 0.9842
- Frequency range: 237.8–278.8Hz
- Data: `benchmark_quiet_singing.csv`

### loud_singing
`cargo run --release --example pitch_benchmark -- loud_singing 15`
Two attempts — CSV only holds the second (same filename overwritten):
- Attempt 1: 94.5% detection, frequency range 70.6–578.5Hz (~3 octaves) —
  Michael confirmed this wasn't a controlled sustained note, so the wide
  range is not attributable to a loud-signal octave-error artifact; left
  unresolved/inconclusive, not re-testable (data overwritten).
- Attempt 2 (sustained note + vibrato, controlled): 5664 windows, 5525
  detected (97.5%), mean confidence 0.9808, frequency range 244.9–284.2Hz
  — consistent with the quiet_singing result, no sign of loud-input octave
  errors on this controlled take.

Data: `benchmark_loud_singing.csv` (attempt 2 only)

### breathiness
`cargo run --release --example pitch_benchmark -- breathiness 15`
First attempt was not a valid test (same clean technique as other
conditions, mislabeled) — discarded, not logged as data.

Second attempt (deliberately breathy/airy vocal technique, same pitch/mic/room):
- 5663 windows, 4342 detected (76.7%) — a real drop from ~97-99% on clean
  singing conditions
- Mean confidence: 0.9514 (also lower than clean-condition ~0.98)
- Frequency range: 61.6–565.8Hz (~3 octaves), std dev 306.9 cents — strongly
  suggests octave errors introduced by the aperiodic breath noise, not a
  genuine 3-octave vocal range in one held note
- **First real evidence that breathiness meaningfully degrades YIN detection
  accuracy**, consistent with Literature Review §1.1's flagged concern

Data: `benchmark_breathiness.csv` (second attempt only)

### background_noise
`cargo run --release --example pitch_benchmark -- background_noise 15`
Real ambient noise present: fan running, window open, music playing in
background, while singing normally.
- 5663 windows, 5461 detected (96.4%) — close to clean-condition detection
  rates, noise did not meaningfully suppress raw detection
- Mean confidence: 0.9743
- Frequency range: 60.9–266.6Hz, std dev 127.3 cents — wide range similar
  in shape to the breathiness result, likely occasional octave errors from
  background noise interference rather than a genuine multi-octave sung
  range
- Detection rate held up better than breathiness (96.4% vs 76.7%), but
  pitch-tracking stability shows the same octave-error symptom
- Note: the harness's oscillation-rate estimate here is not genuine
  vibrato — none was sung for this test; the reading is an artifact of
  the same octave-error instability

Data: `benchmark_background_noise.csv`

### register_high
`cargo run --release --example pitch_benchmark -- register_high 15`
Two attempts — CSV only holds the second (same filename overwritten):
- Attempt 1 (sustained high note, controlled): 5661 windows, 5595 detected
  (98.8%), mean confidence 0.9689, frequency range 219.3–490.3Hz, std dev
  57.6 cents. Normal variation for a real sustained note in a higher
  register — no clear octave-error pattern.
- Attempt 2 (deliberately bending off-pitch, not a controlled single-note
  test): 5661 windows, 5557 detected (98.2%), mean confidence 0.9716,
  frequency range 88.1–499.2Hz, std dev 149.1 cents. Wider range expected
  given deliberate pitch movement — not directly comparable to attempt 1
  or the other conditions' held-note tests.

Data: `benchmark_register_high.csv` (attempt 2 only)

### register_low
`cargo run --release --example pitch_benchmark -- register_low 15`
Two attempts — CSV only holds the second (same filename overwritten):
- Attempt 1 (sustained low note): 5663 windows, 4994 detected (88.2%),
  mean confidence 0.9449, frequency range 110.6–122.5Hz (tight, no
  octave-error signature)
- Attempt 2 (sustained low note): 5664 windows, 5497 detected (97.1%),
  mean confidence 0.9511, frequency range 60.0–125.9Hz — 60Hz is ~half of
  120Hz, a likely YIN octave error (mistaking fundamental for one octave
  down), a known YIN weakness at low pitch

Both attempts show lower confidence than register_high (0.9449/0.9511 vs
0.9689) and an anomalous ~14Hz "oscillation" reading the harness itself
flags as outside normal vibrato range — likely detection jitter at low
pitch, not real modulation.

**Real finding: YIN detection quality degrades at low pitch relative to
high pitch** in this testing (lower confidence, lower/inconsistent
detection rate, and an observed octave error) — consistent with the
known limitation that low-frequency signals fit fewer waveform cycles
per analysis window, weakening the autocorrelation estimate.

Data: `benchmark_register_low.csv` (attempt 2 only)

### different_singers (deferred)
Skipped — requires access to a second singer's voice, not available at
test time. Left open; revisit if/when another voice becomes available.

## Sandbox note
- The sandbox environment reset entirely between sessions (lost repo + Rust toolchain).
  Recovered by re-cloning from GitHub (confirmed as the authoritative source) and
  reinstalling Rust. No project work was actually lost — Michael's local machine had the
  most recent state (multitrack commit) that hadn't yet reached the sandbox's stale clone.

## Real, open gaps still remaining (updated cumulative list)
- pYIN/SwiftF0 comparison not yet done (motivated by breathiness/low-register findings above)
- Real-voice controlled vibrato testing still open (synthetic result only rules out the
  algorithm as primary suspect for the earlier live-test oddities)
- different_singers test condition deferred — needs a second voice
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
- Phase 12 (Pitch Detection): Algorithm Decision recorded — conditional pass, pYIN/SwiftF0
  evaluation recommended before closing

## Operating rules in effect
- One step at a time for any command-line/setup instructions
- No fabricated papers, datasets, benchmarks, or measurements — ever; every claim sourced,
  actually run, or flagged as unverified/not-yet-measured/open
- Real academic/primary sources required for technical claims
- Real-time audio callback isolation rules apply from Phase 1 onward
- Git commit convention: feat/fix/test/docs/refactor, descriptive messages
- Every significant requirement traceable: Requirement → Design → Component → Implementation → Test → Result
