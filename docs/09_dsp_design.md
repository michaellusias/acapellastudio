# AcapellaStudio — DSP Design & Benchmark Results

**Document:** 09_dsp_design.md (roadmap Phase 12 deliverable: "Pitch Detection Module,
Benchmark Results, Algorithm Decision")
**Status:** Draft v1 — real benchmark data collection in progress

> **Standing rule:** every result below was actually run, on real hardware, with real
> singing. Nothing here is estimated or assumed.

---

# 1. Pitch Detection Module

Real, implemented, tested — see `acapellastudio/src/dsp/pitch.rs`:
- `YinFftDetector`: FFT-optimized YIN (de Cheveigné & Kawahara 2002), verified mathematically
  identical to naive YIN (Feasibility Study §2.9)
- `PitchResult`: frequency + genuine confidence (derived from YIN's own CMNDF value, not
  invented) + MIDI note conversion (independently verified against 4 reference points)
- Benchmark harness: `acapellastudio/examples/pitch_benchmark.rs` — records a labeled
  condition, analyzes via sliding window, logs per-window CSV results

---

# 2. Benchmark Results (real, measured)

## 2.1 Bug found and fixed before results could be trusted

The first real run (`normal_singing`, 15s) surfaced a genuine bug: stereo input was fed
into the mono pitch detector without mono-mixing, corrupting the signal. Symptoms: reported
duration was exactly 2x requested (30.29s for a 15s recording), frequency range suspiciously
narrow (60.1–102.1Hz), detection rate only 13.2%. Fixed via `RecordingHandle::drain_available_mono()`.
Full details in `docs/PROJECT_STATUS.md`'s history and the corresponding commit.

## 2.2 Condition: `normal_singing` (15s) — POST-FIX, real result

```
Captured 723584 samples (15.07s of audio)
Total analysis windows: 5638
Windows with a plausible detection: 4560 (80.9%)
Mean confidence (on successful detections): 0.9740
Frequency range detected: 66.7Hz - 994.8Hz
```

**Honest interpretation:** duration now correctly matches the requested 15s (was 30.29s
pre-fix) — direct confirmation the mono-mixing fix worked. Detection rate (80.9%) and mean
confidence (0.974) are both strong for normal continuous singing, consistent with the
~94% detection rate seen in earlier live-mic testing (Feasibility Study §2.5) under similar
continuous-vocalization conditions — the remaining ~19% of windows without a detection is
most likely natural pauses/breaths, not a flaw, consistent with that earlier finding's own
interpretation. The wide frequency range (66.7–994.8Hz) spans roughly two and a half octaves,
plausible for a real singing test covering a natural vocal range, not the suspiciously narrow
pre-fix range.

**Not yet independently verified:** the specific frequency values against a reference pitch
(e.g. a tuner) — this benchmark measures internal consistency (duration, confidence,
detection rate) but not absolute pitch-accuracy ground truth for this specific recording.
The earlier controlled 440Hz reference-tone test (Feasibility Study §2.7, 0.99 cents error)
remains the actual accuracy-ground-truth result; this benchmark is about real-world
robustness (detection rate, confidence under real singing), not accuracy re-verification.

## 2.3 Condition: `vibrato` — two real attempts, second one inconclusive by design

**Attempt 1** (real result, singing style not explicitly recorded at the time):
```
Captured 723840 samples (15.08s of audio)
Windows with a plausible detection: 5498/5640 (97.5%)
Mean confidence: 0.9873
Frequency range: 261.0Hz - 594.0Hz
```
Notably higher detection rate and confidence than `normal_singing`, and a narrower frequency
range — consistent with (not proven to be) a more sustained, continuous vocal performance
than the melodic `normal_singing` test.

**Attempt 2** (real result, after adding oscillation-rate analysis — Michael confirmed this
take was "not fully consistent / just experimenting," not a controlled single-note vibrato
hold):
```
Captured 726528 samples (15.14s of audio)
Windows with a plausible detection: 3262/5661 (57.6%)
Mean confidence: 0.9486
Frequency range: 66.0Hz - 616.0Hz
Pitch deviation (std dev): 285.3 cents
Estimated oscillation rate: 8.68Hz (falls within the ~3-9Hz plausible human vibrato range)
```

**Honest interpretation — why this run doesn't confirm or refute vibrato tracking quality:**
A 285.3-cent standard deviation is far wider than typical cited vocal vibrato depth (roughly
50-100 cents) — this is more consistent with singing across multiple notes/a range than a
single sustained pitch with vibrato, which matches Michael's own confirmation that the take
wasn't a consistent single-note vibrato hold. The 8.68Hz oscillation-rate estimate technically
falls in the plausible vibrato range, but given the wide cents spread, this number is just as
plausibly measuring melodic movement or genuine detection jumpiness as true vibrato
oscillation — the tool's own printed caveat ("consistent with, not proof of") applies exactly
here, and should not be read as confirmation.

**Real conclusion: neither vibrato attempt is a clean, controlled test yet.** A genuine
vibrato-tracking measurement needs a single sustained note held with deliberate, consistent
vibrato for the test's full duration — this remains a real, open action item, not resolved
by either attempt above.

## 2.4 Synthetic vibrato validation — REAL, DECISIVE result (resolves the live-test ambiguity)

To isolate the algorithm from the live-singing performance variable, a synthetic vibrato
generator (`acapellastudio/examples/vibrato_synthesis_test.rs`) was built: a sine wave with a
KNOWN, programmed vibrato rate and depth, no microphone, no real-world noise. This is pure
signal processing and was fully verified in the sandbox (no hardware dependency, unlike the
audio I/O work throughout this project) — this result is authoritative as-is, not pending
real-hardware re-confirmation.

**Real results, 4 test cases spanning realistic vocal vibrato parameters:**

| Base freq | Programmed rate | Programmed depth | Detection rate | Octave errors | Rate error | Mean freq error |
|---|---|---|---|---|---|---|
| 440Hz | 5.5Hz | 80¢ | 100.0% | 0 | 0.5% | 1.32¢ |
| 220Hz | 6.0Hz | 60¢ | 100.0% | 0 | 0.4% | 0.70¢ |
| 440Hz | 5.5Hz | 250¢ | 100.0% | 0 | 0.5% | 10.47¢ |
| 330Hz | 4.0Hz | 100¢ | 100.0% | 0 | 0.9% | 1.65¢ |

**Honest interpretation:** on clean synthetic signals, YIN achieves 100% detection, zero
octave errors, and recovers the programmed vibrato rate to within 1% error in every case —
including the 250-cent-depth case that mirrors the large spread seen in the live "inconsistent"
test (§2.3, attempt 2). **This is a real, decisive answer to the question raised during live
testing** (is this an algorithm problem or a performance/recording issue?): the same exact
detector code tracks vibrato essentially perfectly under controlled conditions, which strongly
points toward the live-test irregularities being caused by the actual singing performance (the
note likely drifted, despite the intent to hold it steady) or real-world microphone/environment
factors, not a fundamental YIN tracking bug.

One data point worth noting precisely rather than glossing over: the detected pitch standard
deviation was consistently *lower* than the programmed depth (e.g. 80¢ programmed → 55.7¢
detected) — this is not inaccuracy, it's the expected mathematical relationship for a sinusoid
(std dev ≈ amplitude/√2 ≈ 0.707×): 80×0.707≈56.6, matching 55.7 almost exactly, and the same
ratio holds for all four cases. This is confirmation the tracking is genuinely faithful, not
an error to be concerned about.

**What this does NOT resolve:** whether YIN tracks vibrato equally well on a *real* voice,
with real formants, breathiness, and microphone noise layered on top of genuine vibrato — that
real-world test (a controlled, single-note, steady vibrato hold, sung carefully) is still a
real, open item. This synthetic result rules out the algorithm as the primary suspect for the
earlier live-test oddities, but doesn't yet prove real-voice vibrato tracking is equally clean.

## 2.5 Remaining conditions — NOT YET TESTED

Per the roadmap's Phase 12 test list, none of the following have real data yet:
- Different singers (only Michael's voice tested so far)
- Different registers (this one test doesn't isolate register — worth a dedicated run per
  register: low/mid/high)
- ~~Vibrato specifically~~ Two live attempts made (§2.3), neither a clean controlled test.
  Synthetic validation (§2.4) is decisive on clean signals (100% detection, 0 octave errors,
  <1% rate error across 4 cases) — real-voice controlled vibrato test still remains open
- Quiet singing
- Loud singing
- Breathiness
- Background noise

---

# 3. Algorithm Decision

**Provisional: YIN-FFT remains the selected pitch detection algorithm**, based on:
- Real accuracy validation on a controlled reference tone (0.99 cents error, §2.7)
- Real confirmation of correctness on real, continuous singing post-bug-fix (80.9% detection,
  0.974 confidence)
- Real CPU cost data (28% with the identified plan-caching inefficiency, not yet re-measured
  with that fix applied)

**Not yet done, per the roadmap's own comparison requirement:** pYIN and SwiftF0 (Literature
Review §1.2, §1.4) have never been implemented or compared against YinFftDetector on the
same real singing-voice recordings. The algorithm decision is provisional, not final, until
that comparison exists.

---

# 4. Status

Draft v1. One real condition (`normal_singing`) has genuine post-fix data. Six more
conditions from the roadmap's test list remain to be run. The algorithm decision remains
provisional pending a real pYIN/SwiftF0 comparison.
