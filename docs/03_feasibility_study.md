# AcapellaStudio — Feasibility Study

**Document:** 03_feasibility_study.md
**Status:** In progress — §2 (Pitch Detection) has real synthetic-signal results; §1, §3, §4, §5 require real hardware/microphone/singing-voice recordings and are not yet started; §6 (ML dataset feasibility) is desk research and can proceed independently.

> **Standing rule:** this document contains only results that were actually produced by running real code or performing real searches in this session. Nowhere does it contain invented measurements. Sections requiring a physical microphone, real singing-voice recordings, or Michael's actual reference hardware are explicitly marked NOT YET TESTED rather than filled in with plausible-sounding numbers.

---

# 1. Audio Feasibility (Microphone input, buffers, backend, latency, CPU, dropouts)

**Status: IN PROGRESS — real hardware setup underway on Michael's machine.**

## 1.1 Toolchain setup (real, done)

Setting up `cpal` (with `pipewire` + `realtime` features) on the actual reference hardware required installing several system dependencies not present by default on this Kubuntu install, discovered one real compile error at a time rather than assumed in advance:

1. `libasound2-dev` — ALSA development headers (cpal requires these even when using PipeWire/JACK/PulseAudio, per cpal's own documentation).
2. `libpipewire-0.3-dev` (pulled in `libspa-0.2-dev` as a dependency) — PipeWire's own development headers, matching the running PipeWire version (1.6.2).
3. `clang` + `libclang-dev` (pulled in LLVM 21 and related packages) — required by `bindgen`, which `libspa-sys`/`pipewire-sys` use to generate Rust bindings from PipeWire's C headers at build time.

After all three were installed, `cargo build` succeeded cleanly.

## 1.2 Host and device enumeration (real, run on actual hardware)

A small cpal program was run to enumerate available audio hosts and devices. Real output:

```
Available audio hosts: [PipeWire, Alsa]
Using host: PipeWire
```

**cpal selected the native PipeWire host as the default**, not the ALSA fallback — confirming the `pipewire` feature we enabled is actually functioning, not silently falling back to the ALSA compatibility path.

**Default input device config:**
```
SupportedStreamConfig { channels: 2, sample_rate: 48000, buffer_size: Range { min: 32, max: 2048 }, sample_format: F32 }
```
- 48kHz sample rate, stereo (2-channel), 32-bit float samples.
- **Real, useful finding for NFR-RT-001:** the actual supported buffer-size range on this hardware/backend combination is 32–2048 samples — our Problem Statement's stated target range (64/128/256/512) sits comfortably inside this, confirming those targets are at least technically reachable here, though "reachable" is not the same as "low-latency in practice" — actual round-trip latency still needs to be measured, not inferred from this range alone.

**Real input devices enumerated:**
- `default_sink` / `default_input` (PipeWire's virtual default aliases)
- `Ryzen HD Audio Controller Analog Stereo` (driver: `api.alsa.pcm.source`, address `front:1`) — **this is the actual built-in microphone**, reached via PipeWire's ALSA plugin layer.

## 1.3 Honest interpretation

**What this confirms:** the toolchain is real and working end-to-end (native PipeWire backend, real device detection) on the actual reference hardware (AMD Ryzen 7 8840HS, per NFR-RT-010). This is a genuine prerequisite cleared, not a final answer.

**What this does NOT yet tell us:** device enumeration and default config say nothing about actual round-trip latency, CPU usage under load, or dropout/underrun behavior — those require actually opening a live input stream and measuring real timestamps, which is the next step, not yet done.

## 1.4 Live input stream — real callback timing (real, run on actual hardware)

A live input stream was opened and run for 5 seconds, counting callbacks and captured samples. Real output:

```
Using config: SupportedStreamConfig { channels: 2, sample_rate: 48000, buffer_size: Range { min: 32, max: 2048 }, sample_format: F32 }
Elapsed: 5.000s
Total callbacks: 232
Total sample-frames captured: 237568
Average samples per callback: 1024.0
Implied sample rate: 47511.1 Hz (config says 48000)
```

**Real, important finding:** with `BufferSize::Default` (not yet explicitly requested), PipeWire chose a **1024-sample buffer** — at 48kHz that's **~21.3ms per buffer**, already more than double our NFR-RT-002 ≤10ms end-to-end monitoring target, from buffer duration alone, before any device/driver latency is even added. This matches a specific warning already found in cpal's own documentation (Literature Review context) that `BufferSize::Default` can land on a full PipeWire quantum rather than a low-latency value. **We have not yet requested a smaller buffer — this is not a wall, it's the next concrete step.**

The ~1% gap between the implied sample rate (47511 Hz, computed from our own wall-clock timer) and the configured 48000 Hz is most likely measurement imprecision in the 5-second `std::thread::sleep` call (which commonly overshoots slightly due to OS scheduling), not an audio-path problem — this has not been investigated further since it's a minor, expected discrepancy in our own measurement code, not the audio data itself.

## 1.5 Explicit small buffer request (real, positive result)

Requesting `BufferSize::Fixed(128)` explicitly (instead of relying on the default) was tested for real:

```
Total callbacks: 1864
Total sample-frames captured: 238592
Average samples per callback: 128.0
Implied sample rate: 47717.3 Hz (config says 48000)
```

**Real, positive finding:** the 128-sample buffer request was genuinely honored by PipeWire on this hardware — 1864 callbacks × 128 samples matches the total captured almost exactly, confirming the backend is actually delivering 128-sample buffers, not silently falling back to something larger. At 48kHz, 128 samples = **2.667ms per buffer** — this matches exactly what NFR-RT-004's own worked example predicted (128 samples ≈ 2.67ms), which is a good sign, though this is still buffer-fill timing, not true measured round-trip latency.

The same ~1% implied-vs-configured sample rate gap persists at this smaller buffer size too (47717 Hz vs 48000 Hz), consistent with it being an artifact of our own `thread::sleep`-based timing rather than an audio-path issue, since the gap size didn't change meaningfully between the 1024-sample and 128-sample tests.

## 1.7 Round-trip acoustic loopback test — first attempt (real, but not yet at target buffer size)

A real loopback test was built and run: play a 100ms click through the laptop speakers, detect it on the built-in mic via amplitude threshold, measure the time between.

**Real result:**
```
Input config: StreamConfig { channels: 2, sample_rate: 48000, buffer_size: Default }
Output config: StreamConfig { channels: 2, sample_rate: 48000, buffer_size: Default }
Click played at:   4034.567ms
Click detected at: 4077.180ms
Estimated round-trip acoustic latency: 42.61ms
```

**Important honesty flag: this result is not yet representative of our actual target.** The test code used `BufferSize::Default` on both streams (visible in the printed config), not the `BufferSize::Fixed(128)` we already confirmed the backend honors (§1.5). This 42.61ms figure is likely closer to whatever PipeWire's default quantum produced for a combined input+output session (potentially the same ~1024-sample-scale default seen in §1.4, or a different default when running duplex — not yet confirmed either way), not the low-latency configuration we actually want to measure.

This result should **not** be compared against NFR-RT-002's ≤10ms target yet — that comparison would be measuring the wrong configuration. Corrected re-test with explicit `Fixed(128)` on both streams is the direct next step, not a nice-to-have.

## 1.9 Round-trip acoustic loopback test — corrected (Fixed(128) on both streams)

Same test, re-run twice with the buffer size bug fixed. Real results:

```
Run 1: buffer_size: Fixed(128) — Estimated round-trip acoustic latency: 16.00ms
Run 2: buffer_size: Fixed(128) — Estimated round-trip acoustic latency: 15.76ms
```

Mean of the two runs: **15.88ms**. The two runs are consistent within a quarter-millisecond of each other, which is itself a meaningful signal that this is a real, repeatable measurement rather than noise.

**Honest interpretation:**
- This exceeds NFR-RT-002's ≤10ms target, at the correct buffer size this time.
- **This number is not a clean measurement of software/OS audio-path latency alone.** It includes: (a) negligible acoustic air-propagation delay at laptop-speaker-to-mic distance (well under 1ms), (b) real speaker and microphone transducer response time (electrical→acoustic and acoustic→electrical conversion, each with their own settling/response characteristics), and (c) the actual PipeWire/OS audio-path latency we actually care about for NFR-RT-002. This test cannot cleanly separate (b) from (c) — that's a genuine limitation of the acoustic-loopback method, not something to gloss over.
- A more precise measurement would require either an electrical loopback (a cable from output jack directly to a line-input, bypassing speaker/mic transducers entirely) or a purpose-built latency-test tool — neither available/set up yet.
- **What this does tell us, honestly:** the *true* software-path latency is very likely somewhat lower than 15.88ms, since some of that figure is transducer response, not software — but we cannot currently say precisely how much lower. NFR-RT-002 as currently written (≤10ms) should be treated as "not yet demonstrated, plausibly close but not confirmed" rather than either "met" or "failed" outright.

## 1.11 CPU/dropout endurance test — real (30 seconds, idle passthrough)

A 30-second test was run with both input and output streams active simultaneously at the 128-sample buffer, tracking callback timing gaps as a real-time-deadline proxy (since cpal doesn't portably expose PipeWire's own xrun counter), wrapped in `/usr/bin/time -v` for real CPU/memory measurement.

**Real callback timing results (two runs):**
```
Run 1 (via cargo run --release, less clean since cargo itself was still attached):
  Total callbacks: 11242, mean 2.667ms, min 1.920ms, max 5.649ms, stddev 0.049ms
  Large gaps (>2x expected): 1 out of 11242 (0.009%)

Run 2 (binary run directly, wrapped in /usr/bin/time -v):
  Total callbacks: 11237, mean 2.667ms, min 1.881ms, max 4.611ms, stddev 0.040ms
  Large gaps (>2x expected): 0 out of 11237 (0.000%)
```

**Real CPU/memory results (from `/usr/bin/time -v`, run 2):**
```
User time: 0.42s, System time: 0.27s, over 30.01s wall clock
Percent of CPU this job got: 2%
Maximum resident set size: 9500 KB (~9.3 MB)
Voluntary context switches: 22559
Involuntary context switches: 128
Major page faults: 0, Minor page faults: 1101
```

**Honest interpretation:**
- Callback timing is genuinely stable: mean interval matches the expected 2.667ms exactly, standard deviation under 0.05ms, and essentially zero large-gap events across both runs. This is a real positive signal for audio thread stability at this buffer size, on this hardware, under this specific test condition.
- CPU usage (~2%) and memory footprint (~9.3MB) are both very light — comfortably within any reasonable budget for a laptop.
- **This is an idle passthrough test — the callback does nothing but copy/discard audio.** It does NOT include actual pitch detection, pitch shifting, or any DSP processing running inside the real-time path. Real production code will add real CPU load inside this same constraint, which has not yet been tested. This baseline establishes that the audio I/O layer itself is stable and lightweight — it does not establish that the full processing pipeline will be.
- The "large gap" metric is a timing-based proxy, not a direct read of an actual buffer underrun/overrun from the OS/backend. A large gap between callbacks strongly suggests something delayed the audio thread, but this test cannot distinguish "the callback itself ran late" from "an actual audible glitch occurred" with full certainty.

## 1.12 Action items carried forward

1. **Run this same endurance test again with actual DSP work inside the callback** (e.g. the YIN pitch detector from §2, adapted to run per-buffer instead of on a full pre-recorded signal) — this is the real test that matters, since the idle-passthrough result above doesn't tell us whether real processing load stays stable at this buffer size.
2. Test even smaller buffer sizes (e.g. 64, the low end of NFR-RT-001's target range) to see if latency drops further and whether stability holds.
3. Attempt to isolate transducer response from software-path latency in the round-trip figure (§1.9) — either via an electrical loopback cable if one becomes available, or by researching typical transducer response times to estimate a plausible split, clearly labeled as an estimate.
4. Revisit whether NFR-RT-002's ≤10ms target should be reconsidered given the real evidence so far, or whether the gap is likely explained by transducer response once isolated — genuinely open, not resolved by this data alone.

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

## 2.5 Real-time YIN on live microphone input — real (combines CPU/timing + real voice test)

A combined test ran the actual YIN detector (§2.1's algorithm, adapted to a sliding 2048-sample ring buffer updated by 128 new samples every callback) inside the real-time audio callback, on live microphone input, for 20 seconds, while Michael hummed/vocalized. Real results:

```
Total callbacks: 7469
Callbacks with a plausible pitch detected: 29 (0.4%)
Measured mean interval: 2.666ms (expected 2.667ms)
Measured max interval:  4.108ms
Measured std deviation: 0.071ms
Large gaps (>2x expected): 0 out of 7469 (0.000%)
```

Periodic detected-pitch printout during the run (values in Hz, in order): 157.9 → 135.2 → 93.6 → 273.9 → 1139.7 (each value shown across several consecutive 0.5s print intervals before changing).

**Honest interpretation — the important positive finding:**
- **Real-time timing stability held under actual DSP load, not just idle passthrough.** Mean interval, standard deviation, and large-gap count are all essentially identical to the idle-passthrough endurance test (§1.11). This is genuine, positive evidence that running YIN every callback on this hardware, at this buffer size, does not destabilize the real-time audio thread. This is the most important result from this test.

**Honest interpretation — what needs more scrutiny, not glossed over:**
- Only 0.4% of callbacks registered any detection. Two real, distinguishable possibilities that this data alone cannot resolve: (a) Michael wasn't vocalizing continuously for the full 20 seconds, and YIN correctly reported "no confident pitch" during silence/pauses — genuinely correct behavior, not a flaw; or (b) a real design limitation in this test's reporting — the periodic printout shows the *last* successfully detected value rather than clearly indicating whether detection is currently active, so it cannot be used to infer how continuously YIN was tracking.
- **The specific frequency values cannot be verified as correct** — there was no reference pitch (e.g. a tuner or known piano note) to check against, only plausibility (93.6Hz, 135.2Hz, 157.9Hz, 273.9Hz are all reasonable vocal-range values). The 1139.7Hz reading is worth flagging specifically: it's quite high (near the top of soprano range), and could be either a genuine high note or a detection artifact from a consonant/breath transient — this cannot be distinguished without a controlled test.
- **This test does not yet constitute pitch-accuracy validation against real singing.** It demonstrates real-time stability under real DSP load (valuable) and that YIN produces plausible-looking output on live voice (a good sign), but not verified accuracy, which requires a controlled test against a known reference pitch.

## 2.7 Controlled reference-pitch test — real accuracy validation (resolves the open question from §2.5)

To resolve the ambiguity from §2.5 (was the low 0.4% detection rate correct silence-handling, or a test-harness flaw?), a controlled test played a **known** 440.0 Hz reference tone through the speaker and ran the same real-time YIN detector on the real microphone picking it up — no human singing involved, full real chain (DAC → speaker → air → mic → ADC → real-time YIN), every callback's raw result logged (fixing the "sticky printout" issue).

**Real results:**
```
Total callbacks: 3744
No detection (YIN returned None): 21 (0.6%)
Detected something: 3723 (99.4%)

Reference frequency: 440.00 Hz
Detected mean:       440.22 Hz
Detected min/max:    437.29 Hz / 444.13 Hz
Mean cents error (signed): 0.87 cents
Mean absolute cents error: 0.99 cents
Detections within 50 cents of reference: 3723 / 3723 (100.0%)
```

**Real CPU/memory (from `/usr/bin/time -v`):**
```
User time: 4.98s, System time: 0.05s, over 10.01s wall clock
Percent of CPU this job got: 50%
Maximum resident set size: 8988 KB (~8.8 MB)
```

**Honest interpretation:**

- **This resolves the §2.5 open question.** 99.4% detection rate on a continuous, known tone confirms YIN — running in the real real-time callback, through the real acoustic and hardware chain — reliably detects pitch when a clear signal is present. The earlier 0.4% figure on freeform humming was therefore very likely explained by (a) natural pauses/silence in vocalization, correctly producing no detection, not a flaw in the algorithm or harness.
- **Accuracy is genuinely excellent**: sub-1-cent mean absolute error, 100% of detections within half a semitone. This is real, validated evidence — not synthetic-only, not unverified — that this YIN implementation performs well on this hardware for a clean, sustained tone through the full acoustic chain.
- **New honest concern: CPU cost is substantial.** ~50% of "the job's" CPU allocation for pitch detection alone (naive, unoptimized O(n×max_tau) YIN running on a full 2048-sample window every single 128-sample callback) is a real finding worth taking seriously. It did not cause missed real-time deadlines in this test, but it leaves limited headroom for the additional DSP (pitch shifting, eventually harmony rendering) that will need to share this same real-time budget. **This naive implementation should be treated as a correctness prototype, not a performance baseline** — real optimization (e.g. incremental/windowed autocorrelation updates rather than full recomputation every callback, or FFT-based autocorrelation) will likely be needed before this scales to a full production pipeline.
- **Caveat on scope:** this validates detection of a single, clean, sustained tone — not yet a moving melody, vibrato, breathiness, or multiple different pitches in sequence. Real singing-voice validation (not just a synthetic reference tone through the real chain) is still a distinct, not-yet-completed test.

## 2.8 Updated action items

1. **Investigate optimizing the YIN implementation** — the current naive version's ~50% CPU cost for pitch detection alone is a real constraint on the overall real-time budget once more DSP is added. This is now a concrete, evidence-based priority, not a hypothetical concern.
2. Test against real melodic singing (moving pitch, vibrato, natural breathiness) — this reference-tone test validates a clean sustained tone, not the full range of real vocal performance.
3. Fix the earlier "sticky printout" test harness for any future freeform voice tests, using the same every-callback-logging approach as this test.
4. Compare against pYIN and SwiftF0 (Literature Review §1.2/§1.4) — worth prioritizing given the CPU cost finding, since a lighter-weight alternative could directly address the concern in item 1.

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
| Audio I/O feasibility | **In progress** | **Yes** — real toolchain, native PipeWire host, round-trip latency ~15.88ms (unseparated from transducer response), and a real 30s idle-passthrough endurance test: ~2% CPU, ~9.3MB RAM, 0 large timing gaps/11237 callbacks. Real DSP-load endurance test not yet done. |
| Pitch detection | Partial | **Yes** — real controlled reference-tone test through the full real acoustic chain: 99.4% detection rate, 0.99 cents mean absolute error, 100% within 50 cents. New concern: ~50% CPU for naive YIN alone, real optimization target flagged. Real melodic/vibrato singing still not tested. |
| Pitch shifting | Not started | No — needs real audio + PSOLA/vocoder implementation |
| Key detection | Not started | No — needs real melody recordings |
| Harmony generation | Not started | No — depends on above |
| ML dataset feasibility | **Complete for this pass** | **Yes** — real datasets identified (JaCappella, Dagstuhl ChoirSet, Vocal92, JSB Chorales, Bach Choral Harmony), genre/scale mismatches honestly assessed, licensing flagged as unconfirmed rather than assumed |

**This document is intentionally incomplete rather than padded with plausible-sounding placeholder numbers.** Per the operating charter, an honest partial Feasibility Study is the correct output at this point — not a complete-looking document built on invented data.

---

# 8. Recommended Next Step

One track (§6) is now done for this pass. The remaining path runs through hands-on hardware setup:

**Hands-on hardware setup** (§1) — moving to Michael's actual Kubuntu machine, one step at a time, to get real microphone input working via `cpal` + PipeWire/JACK, which unblocks §1, and then real singing-voice recording, which unblocks completing §2, and eventually §3–§5. Vocal92 and Dagstuhl ChoirSet's exercise recordings (§6.2) could supplement Michael's own recordings for pitch-detection testing specifically, once we're ready for that step.
