# AcapellaStudio — System Analysis

**Document:** 05_system_analysis.md
**Status:** Draft v1 — Phase 5
**Predecessor documents:** 01_problem_statement.md, 03_feasibility_study.md, 04_requirements.md

> **How to read the "Validation status" field below:** each use case notes whether its underlying mechanism has real evidence behind it (a working, tested prototype from the Feasibility Study) or is purely theoretical at this stage (no prototype yet). This is a direct continuation of the project's evidence-first approach — a use case being *specified* here does not mean its feasibility has been *demonstrated*.

---

# 1. Actors

**Singer/User** — the primary actor: a solo a capella musician recording, editing, and arranging their own vocals.

**System** — AcapellaStudio itself, specifically its DSP/harmony/mixing subsystems, treated as a secondary actor where relevant (e.g. the system autonomously suggests a key).

---

# 2. Use Case Model

## UC-001 — Record Vocal

**Actor:** Singer/User
**Preconditions:** a project is open; an input device is selected.
**Main flow:**
1. User selects a track (or creates a new one) and arms it for recording.
2. User starts recording; system captures audio from the selected input device in real time.
3. User stops recording.
4. System creates a new AudioClip on the track containing the captured audio.

**Alternate flows:** user cancels mid-recording (no clip created); input device becomes unavailable mid-recording (system must surface an error, not silently produce a corrupt/truncated clip — no defined behavior for this yet, flagged as an open gap).

**Postconditions:** a new AudioClip exists on the track.

**Validation status:** **Mechanism validated.** Real microphone capture via cpal/PipeWire was demonstrated and stress-tested (Feasibility Study §1.2–§1.11) — the underlying audio-capture pipeline is real, not theoretical. The *use case as a whole* (track arming, clip creation, UI) is not yet implemented.

---

## UC-002 — Edit Vocal (Clip Editing)

**Actor:** Singer/User
**Preconditions:** at least one AudioClip exists on a track.
**Main flow:**
1. User selects a clip or a region within a clip.
2. User performs an operation: trim, split, delete, move, fade, or gain adjustment.
3. System applies the edit non-destructively, preserving the ability to recover the original.

**Alternate flows:** user undoes the edit (system reverts to the prior state, within the current session per FR-007's scope).

**Postconditions:** the clip's edit state reflects the operation; original recorded audio remains recoverable.

**Validation status:** **Not validated.** No clip-editing prototype exists yet — this use case is specified but entirely unimplemented and untested.

---

## UC-003 — Correct Pitch (Automatic)

**Actor:** Singer/User (initiates), System (performs correction)
**Preconditions:** a clip with detected pitch information exists; a target key/scale is set.
**Main flow:**
1. System continuously detects pitch on the input/clip.
2. System computes the nearest target pitch (per selected scale) for each detected note.
3. System shifts the audio pitch toward the target, within user-configured correction strength/speed.

**Alternate flows:** detected pitch is ambiguous or silence/noise (system should not "correct" toward a spurious target — no defined behavior yet for this case).

**Postconditions:** the clip's audible pitch is corrected; original pitch information is retained for later reference/undo.

**Validation status:** **Partially validated, with a real gap.** Real-time pitch *detection* is validated (Feasibility Study §2.7, 0.99 cents error on reference tone). Real-time pitch *shifting* (PSOLA) is validated in isolation for accuracy (§3.2) but has a confirmed real bug at large upward shifts (§3.2, octave-up failure) and has never been tested for formant preservation at all (§3.3/NFR-AUDIO-003 in the SRS). The two components (detection + shifting) have never been wired together and tested as one pipeline.

---

## UC-004 — Manually Edit Pitch

**Actor:** Singer/User
**Preconditions:** a clip with a visible pitch contour exists.
**Main flow:**
1. User views the detected pitch contour for a clip.
2. User selects a note or region and drags it to a new pitch and/or adjusts timing.
3. System applies the edit and marks the region as manually overridden, per FR-006/Problem Statement §6.4.

**Alternate flows:** automatic correction is later re-run — system must not silently overwrite the manual edit (Problem Statement R-007).

**Postconditions:** the edited region reflects the user's manual pitch, and is flagged as such for future automatic-correction passes.

**Validation status:** **Not validated.** No pitch-editing UI or manual-override data model exists yet.

---

## UC-005 — Harmonize Vocal

**Actor:** Singer/User (initiates), System (generates)
**Preconditions:** a lead vocal clip exists; a key is detected or set.
**Main flow:** see the dedicated **Harmony Wizard use case (§3)** below — this use case is the entry point into that more detailed flow.

**Postconditions:** a new harmony track/clip exists, linked to the source melody it was derived from.

**Validation status:** **Partially validated, with a real gap.** Diatonic-third harmony generation logic is real and tested (Feasibility Study §5.2), including an actual (if limited-value) voice-leading check. A real, confirmed defect exists in the range-fallback path (§5.3, formalized as FR-008a in the SRS) — this use case cannot be considered reliable until that's fixed. Key detection feeding into this use case is algorithm-verified but not tested on real audio (§4.3).

---

## UC-006 — Edit Harmony

**Actor:** Singer/User
**Preconditions:** a generated harmony track/clip exists.
**Main flow:**
1. User selects the harmony track.
2. User applies any of UC-002 (clip editing) or UC-004 (manual pitch editing) to the harmony track, identically to a recorded track.

**Postconditions:** harmony track reflects the user's edits; the track remains editable, not "baked in."

**Validation status:** **Not validated** — depends entirely on UC-002 and UC-004, neither of which is implemented.

---

## UC-007 — Mix Project

**Actor:** Singer/User
**Preconditions:** at least one track exists.
**Main flow:**
1. User adjusts per-track volume, mute, solo, and pan.
2. System combines all tracks into a stereo output in real time for monitoring.

**Postconditions:** the project's mix state reflects the user's control settings.

**Validation status:** **Not validated.** No mixing prototype exists yet. Note: the idle-passthrough and live-DSP endurance tests (Feasibility Study §1.11, §2.11) exercised simultaneous input+output streams, which is architecturally related but is not the same as a real multitrack mixing engine.

---

## UC-008 — Export Project

**Actor:** Singer/User
**Preconditions:** a project with a mix exists.
**Main flow:**
1. User selects an export format/location.
2. System renders the full mix to a file.

**Postconditions:** an audio file exists on disk matching the in-app mix.

**Validation status:** **Not validated.** No export functionality exists yet.

---

# 3. Harmony Wizard Use Case (Detailed)

Per the Project Vision (§10) and the master roadmap's specified flow. This is a detailed expansion of UC-005.

```text
Select Track
    |
    v
Open Harmonize
    |
    v
Choose Style          <- Project Vision §10.1 (single/double/choir/etc.) - NOT YET IMPLEMENTED,
    |                      MVP scope is a single diatonic harmony voice only (Problem Statement)
    v
Choose Voices         <- MVP: fixed at "one voice" - the fuller Voice Configuration
    |                      (Project Vision §10.2) is Phase 2+ scope, not MVP
    v
Set Key               <- REAL, PARTIALLY VALIDATED: Krumhansl-Schmuckler implemented and
    |                      verified on synthetic data (Feasibility Study §4.2); user
    |                      confirmation/override required per Problem Statement §2.3,
    |                      not yet built as a UI step
    v
Set Vocal Ranges      <- REAL, VALIDATED: range-constrained diatonic-third generation
    |                      implemented and tested (Feasibility Study §5.2-§5.3) - INCLUDING
    |                      a confirmed real defect in the fallback path (FR-008a)
    v
Preview               <- NOT YET IMPLEMENTED - no audio rendering of generated harmony
    |                      exists yet, only symbolic note selection (§5 tests symbolic
    |                      output, not rendered audio)
    v
Generate              <- REAL, PARTIALLY VALIDATED: symbolic harmony generation works;
    |                      audio rendering (pitch-shifting the melody to the harmony
    |                      notes) has NOT been wired to this step yet, though the
    |                      underlying PSOLA shifting exists in isolation (§3)
    v
Review
    |
    v
Regenerate/Edit
    |
    v
Accept
```

**Honest summary of this flow's real status:** the two steps most central to the actual harmony logic (**Set Key**, **Set Vocal Ranges**) have real, tested prototypes behind them — including a real known defect that needs fixing before this use case can be considered reliable (FR-008a). Every UI-facing step (Open Harmonize, Choose Style, Preview, Review, Regenerate/Edit, Accept) is currently unimplemented. **Generate** is real only for the symbolic decision-making half of the problem; rendering that decision into actual audio has not yet been demonstrated as one connected pipeline.

---

# 4. Activity Diagrams

## 4.1 Recording

```text
[Start]
   |
   v
Select/Create Track
   |
   v
Arm Track for Recording
   |
   v
User Starts Recording -----> System Opens Input Stream (REAL, validated: Feasibility
   |                          Study §1.2-§1.11)
   v
[Recording Loop: capture buffers, write to clip] (REAL: buffer capture, timing-gap-checked,
   |                                                stable under idle AND real DSP load per
   |                                                §1.11, §2.11)
   v
User Stops Recording
   |
   v
System Finalizes AudioClip
   |
   v
[End]
```

## 4.2 Pitch Correction

**Amended (Problem Statement §33, Amendment 2): this entire flow now runs POST-RECORDING, not in the real-time monitoring path.** The user records with raw monitoring only; the flow below runs afterward, on the captured recording, with no hard deadline.

```text
[Start - triggered after recording is captured, not during live monitoring]
   |
   v
System Detects Pitch (REAL, validated: YIN, 0.99 cents error on reference tone, §2.7 -
   |                    can now run non-causally, with full look-ahead, since there is
   |                    no real-time deadline)
   v
Is confident pitch detected? --No--> [Hold / no correction applied - behavior undefined
   |                                   for ambiguous/silent input, open gap]
  Yes
   |
   v
Compute Nearest Target Scale Tone (NOT YET IMPLEMENTED - no "nearest scale tone" logic
   |                                 exists yet, only raw pitch detection)
   v
Shift Pitch Toward Target (PARTIALLY REAL: PSOLA shifting validated for accuracy at
   |                         correction-scale intervals, §3.2 - but formant preservation
   |                         completely untested, and this has never been driven by a
   |                         "target = nearest scale tone" computation. Removing the
   |                         real-time deadline resolves Architecture §5.3's open question
   |                         about whether PSOLA is fast enough for live use - it no
   |                         longer needs to be.)
   v
Output Corrected Audio
   |
   v
[End]
```

## 4.3 Manual Editing

```text
[Start]
   |
   v
Display Pitch Contour (NOT YET IMPLEMENTED - no UI exists)
   |
   v
User Selects Note/Region
   |
   v
User Drags to New Pitch/Timing
   |
   v
System Applies Edit, Marks Region as Manual Override
   |
   v
[Later] Automatic Correction Re-run? --Yes--> Skip Manually-Overridden Regions
   |                                            (per Problem Statement R-007 - NOT YET
  No                                            IMPLEMENTED as an actual guard)
   |
   v
[End]
```

## 4.4 Harmony Generation

```text
[Start]
   |
   v
Melody Available (from recorded clip)
   |
   v
Detect/Confirm Key (REAL: Krumhansl-Schmuckler, algorithm-verified on synthetic data,
   |                  §4.2 - real audio accuracy still an open question, §4.3)
   v
For Each Melody Note:
   |
   v
   Find Diatonic Third Within Vocal Range (REAL: implemented and tested, §5.2)
   |
   v
   Range Satisfied? --No--> Attempt Fallback (Third Above)
   |                             |
  Yes                            v
   |                         Fallback Satisfied? --No--> CONFIRMED DEFECT: falls back
   |                             |                         to unison (FR-008a) - NOT
   |                            Yes                         A REAL SOLUTION, needs fix
   |                             |
   +-----------------------------+
   |
   v
Symbolic Harmony Note Selected
   |
   v
Render to Audio via Pitch-Shift (NOT YET WIRED UP - PSOLA exists in isolation, §3, but
   |                               has not been connected to this symbolic output)
   v
[End]
```

## 4.5 Export

```text
[Start]
   |
   v
User Selects Format/Location (NOT YET IMPLEMENTED)
   |
   v
System Renders Full Mix (NOT YET IMPLEMENTED - no mixing engine exists)
   |
   v
System Writes Audio File (NOT YET IMPLEMENTED)
   |
   v
[End]
```

---

# 5. Domain Model

## 5.1 Entities

```text
Project
  - contains: Track[]
  - has: sample rate, tempo (if used), key/scale metadata

Track
  - contains: AudioClip[]
  - has: type (Lead Vocal | Harmony Voice | Vocal Percussion), volume, pan, mute, solo

AudioClip
  - belongs to: Track
  - contains: Take[] (if multi-take recording is ever supported - Problem Statement §8.5,
    explicitly deferred/stretch, NOT MVP)
  - has: source audio reference, position, length, fade in/out, gain
  - has: PitchContour (derived, not raw audio)

Take
  - belongs to: AudioClip
  - NOT MVP scope (Problem Statement §8.5) - included here only because the roadmap's
    domain-model list names it; no take-management logic exists or is planned for MVP

PitchContour
  - belongs to: AudioClip
  - contains: Note[] (detected pitch/timing segments) - REAL: this is what our YIN
    prototype actually produces (a sequence of frequency values over time), though no
    formal PitchContour data structure has been built yet in the actual codebase

Note
  - belongs to: PitchContour (as detected) or HarmonyVoice (as generated)
  - has: pitch (frequency or pitch-class+octave), onset, duration, manual-override flag

HarmonyArrangement
  - belongs to: Project (or a specific melody AudioClip)
  - contains: HarmonyVoice[]
  - MVP scope: exactly one HarmonyVoice (Problem Statement MVP definition)

HarmonyVoice
  - belongs to: HarmonyArrangement
  - contains: Note[] (generated) - REAL: this maps directly to the diatonic-third
    harmonizer's actual output (Feasibility Study §5.2), though not yet a formal
    class/struct in any real codebase - currently just a prototype's Vec<Note>

MixerChannel
  - corresponds to: Track (one per track)
  - has: volume, pan, mute, solo state
  - NOT YET IMPLEMENTED - no mixing engine exists

Effect
  - belongs to: MixerChannel or AudioClip
  - NOT MVP scope beyond pitch correction/shifting itself - generic effects chain is not
    part of the MVP feature list
```

## 5.2 Key Relationships

```text
Project 1---* Track
Track 1---* AudioClip
AudioClip 1---0..1 PitchContour
PitchContour 1---* Note
AudioClip (melody) 1---0..1 HarmonyArrangement
HarmonyArrangement 1---1 HarmonyVoice        (MVP: exactly one, not many)
HarmonyVoice 1---* Note
Track 1---1 MixerChannel                      (not yet implemented)
```

## 5.3 Honest note on domain-model maturity

None of the entities above exist as real code structures yet, with one partial exception: the prototypes' internal representations (a `Vec<f64>` of detected frequencies from the YIN prototype; a `Vec<Note>` from the harmony prototype, where `Note` is a real, small `struct { pitch_class, octave }`) are informal precursors to `PitchContour`/`Note`/`HarmonyVoice` above, not the real thing. This domain model is a *design target* for Phase 6 (Architecture) and Phase 7 (Detailed Design), not a description of anything currently implemented as a proper data model.

---

# 6. Status

Draft v1. Use Case Model, Harmony Wizard flow, Activity Diagrams, and Domain Model are all specified. Validation-status annotations throughout distinguish real, tested mechanisms (pitch detection, key detection algorithm, diatonic-third harmony logic, PSOLA accuracy) from entirely unimplemented UI/data-model/integration work — the large majority of this document describes what needs to be built, not what already exists.
