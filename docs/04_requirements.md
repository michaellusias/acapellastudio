# AcapellaStudio — Software Requirements Specification (SRS)

**Document:** 04_requirements.md
**Status:** Draft v1 — Phase 4, grounded in Feasibility Study (03) real measurements
**Predecessor documents:** 01_problem_statement.md (frozen v6, MVP scope), 03_feasibility_study.md (real measured data)

> **How this document differs from the Problem Statement's requirements:** the Problem Statement's FR/NFR list (§6-§7 of `01_problem_statement.md`) was written *before* any real testing. This SRS carries those forward but grounds every measurable NFR in what the Feasibility Study actually measured, not what was originally hoped for. Where a real measurement fell short of, matched, or wasn't yet comparable to the original target, that is stated explicitly rather than silently kept as if already met.

---

# 1. Functional Requirements

## FR-001 — Record Audio
The system shall record audio from a selected microphone input device into a track.
**Acceptance criteria:** recorded audio, when played back, is bit-accurate (or lossless-format-equivalent) to what was captured at the configured sample rate/format.
**Status:** Feasibility confirmed — real audio capture via cpal/PipeWire demonstrated (Feasibility Study §1.2, §1.4).

## FR-002 — Playback Audio
The system shall play back recorded audio through a selected output device.
**Acceptance criteria:** playback starts within a defined latency budget (see NFR-RT-002) and produces no audible dropouts under the defined reference workload (see NFR-REL-001).
**Status:** Feasibility confirmed — real output stream demonstrated (Feasibility Study §1.9, §1.11).

## FR-003 — Create Tracks
The system shall support multiple independent audio tracks within a project.
**Acceptance criteria:** at least the MVP's minimum track count (lead vocal + one harmony voice + one percussion track) can coexist and play back in sync.
**Status:** Not yet implemented or tested — multitrack synchronization has not been prototyped.

## FR-004 — Detect Pitch
The system shall detect the fundamental frequency of vocal input in real time.
**Acceptance criteria:** mean absolute pitch error ≤5 cents on a clean, sustained reference tone; detection succeeds (returns a plausible in-range result) for genuine voiced input.
**Status:** **Exceeded on the reference-tone test** — real measured mean absolute error was 0.99 cents (Feasibility Study §2.7), well under the 5-cent bar. Real melodic/vibrato singing accuracy remains untested (§2.13 notes an informal, non-controlled positive impression only).

## FR-005 — Correct Pitch (Automatic)
**Amended (Problem Statement §33, Amendment 2): performed post-recording, not in the real-time monitoring path.**
The system shall automatically shift detected vocal pitch toward a target note within a selected key/scale, applied to a captured recording rather than live input.
**Acceptance criteria:** corrected output pitch lands within a defined cents tolerance of the target note; since this is no longer real-time, "no detectable dropouts in the real-time path" is replaced by a responsiveness bar (e.g. processing completes within a defined, user-acceptable wait time) — that specific bar is not yet defined and is an open item.
**Status:** Not yet implemented — pitch *detection* is validated (FR-004), but no automatic correction (pitch-shift-to-target-note logic) has been built or tested yet. PSOLA pitch-shifting itself has been tested in isolation (Feasibility Study §3), but not wired into a "detect, then correct toward nearest scale tone" pipeline. Moving this off the real-time path removes the previously-open real-time-feasibility question (Architecture §5.3) but does not by itself fix PSOLA's confirmed octave-up bug or the untested formant-preservation question.

## FR-006 — Manually Edit Pitch
The system shall allow the user to view and manually adjust detected pitch on recorded audio.
**Acceptance criteria:** user-initiated pitch edits are applied non-destructively and are recoverable (see NFR-AUDIO-004).
**Status:** Not yet implemented — no UI or editing-representation work has started.

## FR-007 — Edit Clips
The system shall support non-destructive trim, split, delete, move, fade, and gain adjustment on recorded clips.
**Acceptance criteria:** original recorded audio remains recoverable after any combination of these edits (see NFR-AUDIO-004).
**Status:** Not yet implemented.

## FR-008 — Generate Harmony
The system shall generate one additional harmony voice from a recorded melody, using diatonic-third rule-based harmonization within a vocal-range constraint.
**Acceptance criteria:** generated harmony notes are diatonically correct for the detected/selected key; harmony stays within the configured vocal range; the system does not silently fail when the range constraint cannot be satisfied (see FR-008a).
**Status:** **Partially demonstrated** — real diatonic-third harmonization logic was implemented and tested (Feasibility Study §5.2), including a real voice-leading check (found no violations, though this result was flagged as weak evidence given the technique's structure — §5.2). Not yet tested against real melodic input or real detected keys.

## FR-008a — Harmony Range-Fallback Handling
When no diatonic interval satisfies the vocal-range constraint for a given melody note, the system shall apply a defined, documented fallback strategy (not a silent unison default) and shall log or surface this condition rather than fail silently.
**Acceptance criteria:** a defined fallback (e.g. diatonic sixth) is attempted before any degraded default; the degraded case is distinguishable from normal operation.
**Status:** **Not met — a real, confirmed defect.** The current prototype's fallback (silent unison) was demonstrated to be inadequate (Feasibility Study §5.3: the C3 test case had no valid diatonic third in either direction, and fell back to unison, which is not harmony). This is a known, open defect, not a future risk.

## FR-009 — Create Separate Harmony Track
Generated harmony shall be represented as an independent, editable track, not merged into the lead vocal recording.
**Acceptance criteria:** the harmony track supports the same editing operations (FR-006, FR-007) as a recorded track.
**Status:** Not yet implemented — depends on FR-003 (multitrack) and FR-008 (harmony generation), neither of which is fully built yet.

## FR-010 — Mix Tracks
The system shall provide volume, mute, solo, and pan controls per track, and combine tracks into a stereo output.
**Acceptance criteria:** mix output correctly reflects per-track control states; no clipping under normal gain-staging conditions.
**Status:** Not yet implemented.

## FR-011 — Export Audio
The system shall export the mixed project to a standard audio file format.
**Acceptance criteria:** exported file plays correctly in third-party players and matches the in-app mix.
**Status:** Not yet implemented.

## FR-012 — Detect Key/Scale
The system shall estimate the key/scale of a recorded monophonic melody and allow user confirmation or override.
**Acceptance criteria:** for clean, unambiguous input, the estimated key matches the true key; for ambiguous input, the system surfaces the top candidates and their relative confidence rather than presenting a single silent answer.
**Status:** **Partially demonstrated** — Krumhansl-Schmuckler implementation verified correct on 6/6 clean synthetic symbolic test cases, including one deliberately ambiguous case (Feasibility Study §4.2). Real accuracy on actual sung audio — the literature's identified, unresolved gap — remains untested (§4.3).

---

# 2. Non-Functional Requirements

> **Reading key:** each NFR below states the original target (from the frozen Problem Statement), the real measured result (from the Feasibility Study, where one exists), and an explicit status. "Met," "Not met," and "Not yet measured" are used deliberately and are not interchangeable.

## NFR-RT-001 — Audio Buffer Size
**Target (from Problem Statement NFR-RT-001):** configurable buffer sizes including 64/128/256/512 samples.
**Real measurement:** 128-sample fixed buffer confirmed genuinely honored by PipeWire on the reference hardware (Feasibility Study §1.5). 64-sample buffer not yet tested.
**Status:** Partially met — 128 confirmed; full range not yet tested.

## NFR-RT-002 — Monitoring Latency
**Target (from Problem Statement):** ≤10ms end-to-end for uncorrected monitoring.
**Real measurement:** round-trip acoustic loopback latency measured at 15.76ms and 16.00ms (two consistent runs) at the 128-sample buffer (Feasibility Study §1.9). This figure includes real transducer response time that has not yet been isolated from pure software/OS-path latency.
**Status:** **Not yet met, and not yet confirmed as failed** — the honest status is "not demonstrated," since the measured figure cannot currently be cleanly separated into software-path latency vs. transducer response. This NFR should remain open, not silently downgraded or silently claimed as met, until an isolation method (e.g. electrical loopback) is available.
**Note (post-freeze Amendment 2, Problem Statement §33):** since automatic pitch correction has moved out of the real-time path entirely, this NFR is now the *only* real-time monitoring latency requirement in the project — there is no longer a separate, harder "corrected monitoring" real-time target to also satisfy. This simplifies the real-time latency problem, though it does not change this specific NFR's own status.

## NFR-RT-004 — Audio Processing Deadline (Idle Passthrough)
**Target:** processing time within the available buffer interval (2.667ms at 128 samples/48kHz).
**Real measurement:** idle-passthrough endurance test showed 0 large timing gaps out of 11,237 callbacks over 30 seconds, mean interval exactly matching the 2.667ms expected value (Feasibility Study §1.11).
**Status:** **Met, for idle passthrough.** Not yet confirmed for real DSP load beyond pitch detection (see NFR-RT-004a).

## NFR-RT-004a — Audio Processing Deadline (Real DSP Load)
**Target:** processing time within the buffer interval with real pitch-detection DSP running every callback.
**Real measurement:** naive YIN: ~50% CPU (Feasibility Study §2.7-adjacent real-time test), timing remained stable (0 large gaps). FFT-optimized YIN: 28% CPU (§2.11), timing remained stable, though with higher variance (stddev 0.161ms vs. naive passthrough's 0.071ms).
**Status:** Met for pitch detection alone, on this specific reference hardware. **Not yet tested with pitch-shifting or harmony-rendering DSP added on top** — the real per-callback CPU budget with a fuller processing chain is unknown.

## NFR-AUDIO-001 — Pitch Accuracy
**Target:** measurable pitch error via cents deviation.
**Real measurement:** 0.99 cents mean absolute error on a controlled 440Hz reference tone, through the real acoustic chain, 99.4% detection rate (Feasibility Study §2.7).
**Status:** Met, for a clean sustained tone. Not yet tested for real melodic/vibrato singing.

## NFR-AUDIO-003 — Formant Preservation
**Target:** minimize unwanted formant changes from pitch shifting; applies equally to correction-scale and harmony-scale shifts.
**Real measurement:** **none.** The PSOLA pitch-accuracy test (Feasibility Study §3.2) used a pure sine wave, which has no formants — this NFR has not been tested at all, in either direction.
**Status:** **Not yet measured — a genuine, open, and currently untouched requirement**, despite pitch-shift *frequency* accuracy being partially validated. These are different questions and must not be conflated.

## NFR-HARM-004 — Voice Leading
**Target (from Problem Statement):** evaluate/constrain transitions to reduce voice crossings, excessive leaps, unnecessary parallel motion.
**Real measurement:** a real parallel-fifth/octave check was run against generated harmony output; zero violations found on the test melody (Feasibility Study §5.2) — but this result was explicitly flagged as weak evidence, since the diatonic-third technique's constant interval type makes such violations structurally unlikely regardless of quality. The higher-risk case (fallback discontinuities between "third below" and "third above") was not directly exercised by voice-leading-violation testing.
**Status:** Not yet meaningfully measured — the test that ran doesn't yet probe the case that actually matters.

## NFR-ML-003 — ML Dataset Availability (informs future ML-harmony NFRs)
**Target:** identify whether adequate training data exists for ML-based harmony generation.
**Real finding:** real aligned-audio a capella datasets exist (JaCappella, Dagstuhl ChoirSet) but are genre/scale-mismatched with the target use case; a symbolic-only dataset (JSB Chorales) exists with unconfirmed licensing (Feasibility Study §6).
**Status:** Partially resolved — real options exist, none are a clean match. This remains an open decision point for Phase 3+ (per the Project Vision), not an MVP blocker since ML harmony is explicitly deferred.

---

# 3. Requirements Not Yet Given Measurable Acceptance Criteria

Per the roadmap's explicit instruction to avoid vague requirements, the following areas from the Problem Statement still need real acceptance criteria defined before they can be treated as proper NFRs, and are flagged here rather than silently ported forward as-is:

- **Usability** (Problem Statement NFR-USE-001 to 004) — no measurable criteria defined yet; needs real user-testing methodology before Phase 5+.
- **Reliability** (NFR-REL-001 to 003) — "stable audio playback" needs a defined stress-test protocol and quantitative dropout/crash tolerance, not yet specified.
- **Security/Privacy** (§13 of Problem Statement) — the offline-only requirements are behaviorally clear but have no defined *verification* method yet (e.g. how do we confirm no network calls are made, beyond code review?).

---

# 4. Traceability to Feasibility Study

| Requirement | Feasibility Study Section | Real Evidence Status |
|---|---|---|
| FR-004 (pitch detection) | §2.7 | Exceeded target |
| FR-005 (auto-correction) | — | Not yet built |
| FR-008 (harmony generation) | §5.2, §5.3 | Partially demonstrated; real defect found (FR-008a) |
| FR-012 (key detection) | §4.2, §4.3 | Algorithm verified; real-world accuracy gap open |
| NFR-RT-002 (latency) | §1.9 | Measured, not yet confirmed met or failed |
| NFR-RT-004a (DSP CPU load) | §2.7-adjacent, §2.11 | Met for pitch detection alone |
| NFR-AUDIO-003 (formant preservation) | — | Not yet tested at all |

---

# 5. Status

**Draft v1.** This SRS should be revisited as further Feasibility Study work (real melodic singing tests, formant-aware pitch-shift testing, the FR-008a fallback fix) produces new real evidence — several NFRs above are explicitly marked "not yet measured" rather than assumed, and should be updated only when real data exists, not by inference.
