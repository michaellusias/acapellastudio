# AcapellaStudio

## A Real-Time Vocal Production Environment for A Capella Musicians

**Document Type:** Problem Statement
**Document Status:** FROZEN — v6, approved as final Problem Statement
**Working Name:** AcapellaStudio


> NOTE ON DOCUMENT SCOPE
>
> This document defines the problem being solved, the research questions,
> project objectives, users, scope, MVP, risks, and success direction.
>
> Detailed architecture, Software Requirements Specification (SRS),
> requirements traceability, system design, implementation details,
> testing procedures, and performance-test specifications shall be
> maintained as separate documents according to the project's
> Software Development Life Cycle (SDLC).
>
> This separation is intentional. The Problem Statement establishes
> WHAT problem is being solved and WHY. Later SDLC documents will
> establish HOW the system will be designed, implemented, tested,
> and evaluated.


# 1. Problem Statement

## 1.1 Title

**AcapellaStudio — A Real-Time Vocal Production Environment for A Capella Musicians**


## 1.2 Background

A capella music is a form of musical performance in which musical material is produced primarily or entirely using the human voice. Typical arrangements may contain a lead vocal, one or more harmony parts, bass vocals, and vocal percussion.

Producing a polished a capella recording presents several technical challenges. A solo performer who wishes to create a layered arrangement may need to perform, record, tune, edit, arrange, and mix several independent vocal layers.

Existing digital audio workstations (DAWs) provide powerful recording, editing, mixing, and processing capabilities. However, they are designed as general-purpose music-production environments rather than workflows specifically centered around vocal-only production.

A solo a capella musician may therefore need to combine multiple functions and tools to:

1. Record multiple vocal performances
2. Detect and correct pitch
3. Manually edit individual notes
4. Create harmonically appropriate additional vocal parts
5. Arrange multiple vocal layers
6. Record and loop vocal percussion
7. Mix the resulting vocal arrangement

AcapellaStudio investigates whether these activities can be integrated into a single lightweight, offline, vocal-focused desktop environment.


# 2. Problem Definition

## 2.1 Central Problem

The central problem addressed by AcapellaStudio is:

> How can a standalone desktop application provide a low-latency vocal recording, editing, pitch-processing, and multitrack production environment while integrating automatic harmony generation into a unified workflow specifically designed for a capella musicians?

The project focuses on the intersection of:

- Digital signal processing
- Real-time audio systems
- Pitch detection
- Pitch correction
- Manual pitch editing
- Music theory and computational harmony
- Multitrack audio processing
- Offline machine learning
- Human-computer interaction
- Software engineering


# 3. Motivation and Justification

## 3.1 Problem in Existing Workflows

General-purpose DAWs are capable of recording and mixing vocals, but their workflows are not specifically optimized for a capella production.

A solo a capella musician who wants to create a complete arrangement may have to manually perform multiple harmony parts, use separate pitch-correction tools, arrange the resulting tracks, and use additional tools or manual techniques for harmony generation.

This creates several difficulties:

- Increased workflow complexity
- Repetition of manual tasks
- Increased technical requirements for inexperienced users
- Lack of integrated harmony-generation assistance
- Lack of vocal-specific production workflows

AcapellaStudio aims to investigate whether these problems can be reduced through an integrated vocal-first application.


## 3.2 Technical Justification

The project is not intended to be merely a graphical wrapper around existing audio software.

The system contains several genuine technical problems, including:

- Real-time fundamental-frequency estimation
- Low-latency audio processing
- Pitch-shifting and formant preservation
- Manual pitch manipulation
- Music-theory-based harmony generation
- Vocal-range-aware voice generation
- Multitrack audio scheduling
- Real-time CPU and thread management
- Offline ML inference

These problems provide a substantial Software Engineering, audio-processing, and computational-musicology research component.


# 4. Users and Stakeholders

## 4.1 Primary Users

### Solo A Capella Artists

Musicians who perform multiple vocal parts themselves and require assistance recording, tuning, arranging, and generating harmony parts.

### Small A Capella Groups

Vocal groups requiring a lightweight environment for recording and arranging their vocal performances.

### Vocal Percussionists / Beatboxers

Performers who create rhythmic material using their voices and require recording and looping functionality.

### Music Students and Hobbyists

Users interested in vocal arrangement and experimentation without requiring extensive professional DAW knowledge.


## 4.2 Secondary Stakeholders

- Developers and maintainers
- Academic supervisors
- Audio/DSP researchers
- Machine-learning researchers
- Software testers
- Music educators


# 5. Project Objective

The primary objective of AcapellaStudio is to design and develop a standalone, offline desktop application that provides an integrated environment for vocal-only music production.

The system will investigate and prototype:

1. Vocal recording
2. Multitrack vocal playback
3. Real-time pitch detection
4. Automatic pitch correction
5. Manual pitch editing
6. Non-destructive audio clip editing
7. Key and scale identification
8. Rule-based harmony generation
9. Independent harmony-track creation
10. Vocal-percussion recording and looping
11. Basic vocal mixing
12. Offline operation

Machine-learning-based harmony generation will be investigated as a research extension after the rule-based harmony system has been demonstrated successfully.


# 6. Research Questions

## 6.1 Pitch Detection

How accurately can a lightweight pitch-detection algorithm such as YIN estimate the fundamental frequency of a singing voice under realistic conditions, including vibrato, breathiness, noise, and unvoiced segments?


## 6.2 Key and Scale Detection

How reliably can the system infer the key or scale of a monophonic vocal melody when no accompanying chordal information is available?

How should the system respond when the estimated key or scale is ambiguous?

The system shall therefore allow user confirmation or manual selection rather than assuming that automatic key detection is always correct.


## 6.3 Automatic Pitch Correction

**Amended post-freeze (§33, Amendment 2): pitch correction is performed post-recording, not in the real-time monitoring path.** "Processing latency" below now refers to responsiveness of the offline correction step, not a hard real-time deadline.

How can detected vocal pitch be shifted toward a desired musical pitch while maintaining acceptable audio quality and minimizing audible artifacts?

The investigation shall consider:

- Pitch accuracy
- Processing latency (offline responsiveness, not real-time deadline — see amendment above)
- Formant preservation
- CPU consumption
- Audio artifacts
- Correction strength
- Correction speed


## 6.4 Manual Pitch Editing

How can users visually inspect and manually modify the pitch of recorded vocal notes?

The system shall investigate an interface that allows the user to:

- View the detected pitch contour
- Identify individual notes or note regions
- Move a note upward or downward in pitch
- Adjust note timing where appropriate
- Override automatic pitch correction
- Undo or modify previous edits


## 6.5 Non-Destructive Audio Editing

How can standard audio editing operations be implemented without permanently modifying the original recording?

The system shall investigate non-destructive:

- Trim
- Split/Snip
- Delete
- Move
- Fade in
- Fade out
- Gain adjustment
- Undo/Redo

operations.


## 6.6 Rule-Based Harmony Generation

How effectively can music-theory-based rules generate a musically appropriate harmony voice from a single recorded vocal melody?

The initial harmony system shall consider:

- Key
- Scale
- Chord
- Diatonic relationships
- Intervals
- Vocal range
- Tessitura
- Voice-leading
- Avoidance of unreasonable melodic movement

The MVP shall initially focus on generating **one additional harmony voice**.


## 6.7 Independent Harmony Voice Generation

A generated harmony shall not merely exist as an abstract sequence of notes.

The system shall investigate how the generated harmony can be converted into an independent vocal track that can subsequently be:

- Edited
- Pitch-corrected
- Moved
- Muted
- Soloed
- Mixed
- Exported

The long-term architecture shall allow additional harmony voices to be generated independently.

For example:

Lead Vocal
    |
    +---- Harmony Voice 1
    |
    +---- Harmony Voice 2
    |
    +---- Harmony Voice 3
    |
    +---- Harmony Voice 4

Each generated voice shall ultimately be represented as a separate editable track.


## 6.8 Harmony-to-Audio Generation

A major research question is how a generated harmony representation should become usable audio.

The project shall investigate whether harmony generation should produce:

1. MIDI/note-level information
2. Pitch-shifted audio derived from the original vocal
3. Synthesized vocal-like audio
4. Another intermediate representation

For the MVP, the preferred approach shall be evaluated based on:

- Audio quality
- Computational complexity
- Implementation feasibility
- Voice individuality
- Pitch accuracy
- Formant preservation
- Offline execution

The selected approach shall be justified experimentally rather than assumed in advance.

> **Carried forward to Feasibility Study:** this question is directly linked to R-012 below. Whether the harmony voice is produced by pitch-shifting the same recorded take, versus a more elaborate resynthesis/variation approach, is an open architectural fork this document intentionally does not resolve. It should be one of the first questions the Feasibility Study answers, since it affects which pitch-shifting approach is worth prototyping first.


## 6.9 Large-Interval Pitch Shifting

Pitch correction and harmony generation have different pitch-shifting requirements.

Automatic correction may require relatively small pitch changes, while generating a harmony may require movement by a third, fifth, sixth, or octave.

Large pitch shifts can introduce:

- Formant distortion
- Robotic vocal characteristics
- Phase artifacts
- Transient degradation
- Unnatural vocal timbre

Therefore, correction-scale pitch shifting and harmony-scale pitch shifting shall be evaluated as separate technical problems.

> **Carried forward to SRS:** the specific harmony interval(s) the MVP targets first (e.g. a third above, a third below, user-selectable) is a design decision, not a Problem Statement decision — to be settled when the SRS is written.


## 6.10 Machine Learning Harmony

Can a machine-learning model generate a natural-sounding harmony voice from a single vocal melody while remaining practical for offline desktop inference?

The investigation shall consider:

- Training-data availability
- Representation of melody and harmony
- Model architecture
- Training requirements
- Inference speed
- CPU/RAM requirements
- Model size
- Harmonic accuracy
- Perceptual quality


## 6.11 ML Dataset Feasibility

Before committing to ML implementation, the project shall investigate whether suitable datasets containing aligned lead and harmony vocal material are available.

If suitable datasets are unavailable, the project shall evaluate the feasibility of constructing a small custom dataset.

ML harmony generation shall not become a mandatory dependency for the core application if data or implementation constraints make it impractical.


## 6.12 Real-Time Performance

Can the audio engine process incoming and playback audio within the required buffer deadlines without producing audible interruptions?

The investigation shall separately evaluate:

### A. Uncorrected Monitoring

Raw microphone monitoring without pitch correction.

This path shall target a low end-to-end monitoring latency appropriate for live singing, with an initial target of approximately 10 ms or lower under the defined reference configuration.

### B. Corrected Monitoring — REMOVED post-freeze, see §33

~~Monitoring in which the vocal signal passes through real-time pitch detection and pitch correction.~~

**Superseded (see §33, Post-Freeze Amendment 2): automatic pitch correction is no longer performed in the real-time monitoring path. The singer monitors raw, uncorrected audio while recording; pitch correction is applied afterward, on the captured recording, with no real-time deadline.** This subsection is retained, struck through, for change-history purposes rather than deleted outright.


## 6.13 Resource Efficiency

Can the application perform its core audio-processing functions using reasonable CPU and memory resources on a defined reference system?

Performance shall be evaluated using measurable workloads rather than subjective claims such as "fast" or "lightweight."


# 7. Corrected MVP Definition

The following features constitute the official MVP.

Anything not explicitly listed here is outside the MVP unless later approved through requirements change management.


## 7.1 MVP Features

### 1. Vocal Recording

The user can record vocals from a supported microphone or audio input device.


### 2. Multitrack Vocal Recording

Multiple vocal recordings can coexist as independent tracks.


### 3. Playback

The system can play recorded vocal tracks simultaneously while maintaining synchronization.


### 4. Real-Time Pitch Detection

The system detects the fundamental frequency of incoming vocals during recording/monitoring.

Initial algorithm:

**YIN**

Alternative algorithms may be investigated if YIN does not provide sufficient accuracy.


### 5. Automatic Pitch Correction

**Amended post-freeze (§33, Amendment 2): applied post-recording, not during real-time monitoring.**

The system can automatically correct detected vocal pitch toward a selected key/scale or target note.

Initial correction is intended for relatively small pitch deviations.


### 6. Manual Pitch Editing

The user can visually inspect recorded vocal pitch and manually:

- Select notes or pitch regions
- Move pitch upward/downward
- Adjust note timing where supported
- Override automatic correction
- Undo/redo edits


### 7. Non-Destructive Clip Editing

The system supports:

- Trim
- Split/Snip
- Delete
- Move
- Fade in
- Fade out
- Gain adjustment
- Undo
- Redo

The original recording shall remain recoverable.


### 8. Key/Scale Detection

The system shall attempt to identify the key/scale of a recorded monophonic vocal melody.

If the confidence of the estimate is insufficient, the user shall be able to select or confirm the key manually.


### 9. One Rule-Based Harmony Voice

The system shall generate one additional harmony voice using music-theory-based rules.

The MVP harmony system shall initially focus on:

- Diatonic harmony
- Triadic harmony
- Key/scale awareness
- Vocal-range awareness
- Basic voice-leading


### 10. Independent Harmony Track

The generated harmony shall become a separate track rather than being permanently merged with the lead vocal.

For example:

Lead Vocal
Harmony 1

The harmony track shall be independently editable and mixable.


### 11. Harmony Track Processing

The generated harmony track shall support applicable editing and processing operations, including:

- Pitch editing
- Clip editing
- Volume adjustment
- Mute
- Solo
- Pan
- Basic mixing


### 12. Vocal Percussion

The system shall support recording vocal percussion/beatboxing as independent tracks.


### 13. Vocal Percussion Looping

Selected vocal-percussion regions can be repeated to create rhythmic layers.


### 14. Basic Mixing

The system shall provide:

- Volume
- Mute
- Solo
- Pan
- Basic gain control


### 15. Audio Export

The completed arrangement can be exported to a supported audio format.


### 16. Offline Operation

Core functionality shall operate without requiring Internet connectivity.


# 8. Explicitly Deferred Features

The following are not part of the MVP.

## 8.1 Machine-Learning Harmony

ML harmony generation is a research extension.

It shall be attempted only after the rule-based harmony system functions successfully.


## 8.2 Multiple Automatically Generated Harmony Voices

The MVP generates one harmony voice.

Future versions may generate additional independent voices such as:

- Harmony 1
- Harmony 2
- Harmony 3
- Bass
- Additional upper/lower voices

Each voice should eventually become an independent editable track.


## 8.3 Full SATB Arrangement

Full automatic:

Soprano
Alto
Tenor
Bass

or larger arrangements are deferred until a reliable single-harmony system exists.


## 8.4 Advanced Harmonic Vocabulary

The following are future research directions:

- Passing chords
- Chromatic harmony
- Modal interchange
- Secondary dominants
- Reharmonization
- Extended chords
- Upper structures
- Complex jazz harmony

These shall not be required for the MVP.


## 8.5 Multi-Take Comping

Advanced take-management and automatic comping are deferred.


## 8.6 Persistent Cross-Session Undo History

Basic in-session undo/redo is included in the MVP.

Persistent historical editing across sessions is deferred.


## 8.7 Cross-Platform Validation

The initial implementation shall target one explicitly selected operating system.

Additional operating systems shall not be claimed as supported until experimentally validated.


# 9. Harmony Architecture Direction

Although multi-voice harmony generation is outside the MVP, the system shall be designed so that the MVP does not prevent future expansion.

The conceptual long-term workflow is:

Lead Vocal
      |
      v
Melody / Pitch Analysis
      |
      v
Key / Scale / Chord Analysis
      |
      v
Harmony Generation
      |
      +----------------+
      |                |
      v                v
Harmony Voice 1   Harmony Voice 2
      |                |
      v                v
Independent Track Independent Track
      |                |
      +--------+-------+
               |
               v
            Mixer
               |
               v
             Export

Future versions may extend this to:

Lead
Soprano
Alto 1
Alto 2
Tenor 1
Tenor 2
Bass
Vocal Percussion

with every generated voice represented as an independent track.

This is a future architectural direction and is not an MVP requirement.


# 10. Audio Processing Considerations

The project shall distinguish between different categories of pitch manipulation.

## 10.1 Corrective Pitch Shifting

Used to move a sung note toward its intended pitch.

Typical shift:

Small deviation from target pitch.


## 10.2 Harmony Pitch Shifting

Used to transform an existing vocal performance into another musical voice.

Potential shift:

Third
Fourth
Fifth
Sixth
Octave
or other intervals.

The project shall not assume that an algorithm producing acceptable corrective pitch correction will automatically produce acceptable harmony vocals.


## 10.3 Formant Preservation

Both automatic correction and harmony generation shall investigate formant preservation.

Poor formant handling may produce:

- "Chipmunk" effects
- Artificial/robotic vocals
- Unnatural vocal character

The selected approach shall therefore be evaluated using objective measurements where possible and human perceptual evaluation where appropriate.


# 11. Real-Time System Direction

The real-time audio system shall be designed around the principle that deadline-critical processing must remain isolated from non-real-time operations.

The real-time path is conceptually:

Microphone
    |
    v
Audio Backend
    |
    v
Input Buffer
    |
    v
Real-Time Audio Processing
    |
    +--> Pitch Detection
    |
    +--> Pitch Correction
    |
    +--> Mixing
    |
    v
Output Buffer
    |
    v
Audio Backend
    |
    v
Headphones / Speakers

Operations such as:

- ML inference
- Project saving
- File I/O
- GUI operations
- Complex harmony generation
- Audio export

shall not block the real-time audio callback.


# 12. Software Engineering Context

AcapellaStudio shall be developed using a structured Software Engineering process.

The project shall progress through major stages such as:

1. Problem Definition
2. Requirements Engineering
3. Feasibility Study
4. System Analysis
5. System Architecture
6. Detailed Design
7. Implementation
8. Unit Testing
9. Integration Testing
10. System Testing
11. Performance Evaluation
12. User Evaluation
13. Refinement
14. Final Documentation

Requirements shall eventually be traceable through:

Requirement
    |
    v
Design Component
    |
    v
Implementation
    |
    v
Test Case
    |
    v
Test Result
    |
    v
Acceptance Decision


# 13. Major Technical Risks

## R-001 — Excessive Audio Latency

Pitch-processing algorithms may introduce unacceptable monitoring latency.

**Mitigation:**

Evaluate audio buffers, audio backends, and pitch-processing algorithms early.


## R-002 — Audio Dropouts

DSP processing may exceed the available audio-buffer deadline.

**Mitigation:**

Use a dedicated real-time audio architecture and experimentally evaluate processing deadlines.


## R-003 — ML Inference Too Slow

ML harmony generation may require excessive CPU or memory.

**Mitigation:**

Keep ML inference outside the real-time audio path and investigate lightweight models and optimized inference runtimes.


## R-004 — Poor Pitch Detection

YIN may perform poorly on:

- Breathiness
- Noise
- Vibrato
- Sibilance
- Unvoiced consonants
- Polyphonic/overlapping vocal material

**Mitigation:**

Benchmark YIN and investigate alternative algorithms if necessary.


## R-005 — Unnatural Pitch Correction

Pitch shifting may produce robotic or otherwise unnatural audio.

**Mitigation:**

Evaluate multiple pitch-shifting approaches and investigate formant preservation.


## R-006 — Excessive Project Scope

The project combines:

- DAW functionality
- DSP
- Manual pitch editing
- Harmony generation
- Real-time systems
- ML
- GUI development

Attempting to implement all advanced features simultaneously may make the project impractical.

**Mitigation:**

Maintain the defined MVP and treat advanced functionality as future work.


## R-007 — Manual/Automatic Editing Conflicts

Automatic correction may overwrite manual edits.

**Mitigation:**

Explicitly represent manual overrides and ensure that automatic processing does not silently destroy them.


## R-008 — ML Training Data Scarcity

Suitable datasets containing aligned lead and harmony vocals may be unavailable.

**Mitigation:**

Investigate dataset availability during the feasibility stage. If necessary, construct a small controlled dataset or defer ML harmony generation.


## R-009 — Latency Budget Optimism

~~Pitch-corrected monitoring may not achieve the same latency as uncorrected monitoring.~~

**Resolved by architectural change, post-freeze (§33, Amendment 2): since pitch correction no longer runs in the real-time monitoring path at all, this specific risk no longer applies as originally framed.** Retained, struck through, for change history. The underlying real evidence that motivated this risk (Feasibility Study §1.9: ~15.88ms measured round-trip, not cleanly separable from transducer response) remains relevant context for *why* keeping correction out of the real-time path was the right call, even though the risk itself is now moot.

**Mitigation (historical):** Measure algorithmic latency separately and establish independent acceptance targets.


## R-010 — Large-Interval Pitch-Shift Quality

Harmony generation may require larger pitch shifts than automatic correction.

**Mitigation:**

Evaluate pitch-shifting quality separately for corrective and harmony-scale intervals.


## R-011 — Key/Scale Detection Errors

A monophonic vocal melody may not contain enough information to determine the key reliably.

Incorrect key detection could result in incorrect harmony generation.

**Mitigation:**

Provide confidence estimation and user confirmation/override.


## R-012 — Generated Harmony Sounds Like the Original Voice

Simply pitch-shifting the lead vocal may produce a technically correct but perceptually unnatural harmony because the generated voice retains excessive similarity to the source performance.

**Mitigation:**

Investigate formant manipulation, timing variation, dynamics variation, alternative synthesis methods, or separately recorded/generated vocal material.


## R-013 — Harmony Generation Produces Musically Valid but Unpleasant Results

A harmony may technically satisfy interval and scale rules while still sounding musically poor.

**Mitigation:**

Evaluate voice-leading, melodic motion, vocal range, dissonance, and perceptual quality rather than relying solely on theoretical correctness.


## R-014 — IP/Patent Risk in Pitch-Shift-Based Harmony Generation *(added post-freeze, Phase 1 Literature Review)*

A US patent (8,168,877 / 8,618,402, "Musical harmony generation from polyphonic audio signals") describes a system architecturally close to our planned rule-based harmony engine: analyzing a melody, computing pitch-shift amounts, and applying PSOLA-based pitch shifting to generate harmony audio.

**Mitigation:**

Investigate the patent's actual claims and current legal status during the Feasibility Study, ideally with real legal guidance rather than our own reading of the patent text, before committing implementation effort to this architecture. See `02_literature_review.md` §2.4 for the source and full context.


# 14. Scope Prioritization

The project shall follow the following development priority.

## Priority 1 — Core Audio

- Audio input/output
- Recording
- Playback
- Basic multitracking


## Priority 2 — DSP and Editing

- Pitch detection
- Automatic pitch correction
- Manual pitch editing
- Clip editing
- Low-latency monitoring


## Priority 3 — Musical Analysis

- Key detection
- Scale identification
- User key/scale confirmation


## Priority 4 — Rule-Based Harmony

- One harmony voice
- Diatonic harmony
- Vocal-range awareness
- Basic voice-leading
- Independent harmony track


## Priority 5 — Production

- Vocal percussion
- Looping
- Mixing
- Export


## Priority 6 — Research Extension

- ML dataset investigation
- ML harmony prototype
- Comparison between rule-based and ML approaches


## Priority 7 — Future Expansion

- Multiple harmony voices
- SATB arrangement
- Advanced harmonic vocabulary
- Advanced vocal synthesis
- Multi-take comping
- Cross-platform support


# 15. Success Criteria

The MVP shall be considered successful if it demonstrates:

1. Functional native desktop vocal recording
2. Reliable vocal playback
3. Functional multitrack vocal management
4. Measurable pitch-detection performance
5. Functional automatic pitch correction
6. Functional manual pitch editing
7. Non-destructive clip editing
8. Key/scale detection with user override
9. Functional rule-based generation of one harmony voice
10. Conversion of the generated harmony into an independent editable track
11. Basic vocal-percussion recording and looping
12. Basic vocal mixing
13. Audio export
14. Offline core operation
15. Separation between real-time and non-real-time processing
16. Measurable audio latency
17. Measurable CPU and memory usage
18. Testing of real-time audio stability
19. Traceability between requirements, implementation, and testing
20. Documented limitations and experimental results


# 16. Research Evaluation Direction

The project shall avoid unsupported claims such as:

"low latency"
"high quality"
"natural harmony"
"efficient"
"accurate"
"reliable"

unless these claims can be supported by defined measurements or evaluation procedures.

Examples:

"Low latency"
    ->
Measured end-to-end latency

"Accurate pitch detection"
    ->
Measured frequency/cents error

"Efficient"
    ->
Measured CPU and memory usage

"Reliable audio engine"
    ->
Measured buffer deadline violations/XRuns/dropouts

"Natural harmony"
    ->
Musical evaluation + perceptual evaluation

"Non-destructive editing"
    ->
Verified restoration of the original recording


# 17. Project Constraints

1. The application shall operate offline.
2. The application shall be a native desktop application.
3. The project shall prioritize vocal-only production.
4. Real-time audio processing shall use a low-latency architecture.
5. ML inference shall not block the real-time audio path.
6. Initial audio-rate investigation shall prioritize 44.1 kHz and 48 kHz.
7. A reference hardware configuration shall be established before final performance evaluation.
8. The MVP shall generate one automated harmony voice.
9. Multiple harmony voices shall remain future work.
10. Advanced harmonic generation shall remain outside the MVP.
11. Professional-grade performance shall not be claimed without objective evidence.
12. Original recordings shall remain recoverable through non-destructive editing.
13. Cross-platform compatibility shall not be claimed until experimentally validated.


# 18. Long-Term Vision

Although the MVP intentionally focuses on one harmony voice, the long-term vision of AcapellaStudio is a vocal-first production environment capable of assisting a musician in constructing a complete vocal arrangement from a relatively small amount of source material.

A future workflow could conceptually be:

Lead Vocal
    |
    v
Pitch + Melody Analysis
    |
    v
Key / Scale / Chord Analysis
    |
    v
Harmony Arrangement Engine
    |
    +---- Soprano
    |
    +---- Alto 1
    |
    +---- Alto 2
    |
    +---- Tenor 1
    |
    +---- Tenor 2
    |
    +---- Bass
    |
    +---- Vocal Percussion
    |
    v
Independent Vocal Tracks
    |
    v
Editing + Mixing
    |
    v
Final A Capella Production

Each generated vocal part should ultimately exist as an independent track so that the user retains control over:

- Pitch
- Timing
- Volume
- Pan
- Muting
- Soloing
- Editing
- Mixing

The long-term goal is therefore not merely "automatic harmony generation."

It is the development of a vocal-first production environment in which computational tools assist the musician while preserving user control over the final arrangement.


# 19. Current Document Status

**Status:** FROZEN — v6 approved as the final Problem Statement.

The following decisions remain intentionally open and shall be resolved through the subsequent SDLC stages rather than being assumed in this document:

- Primary operating system
- Audio backend
- Reference hardware
- Final audio I/O architecture
- Final pitch-detection algorithm
- Final pitch-shifting algorithm
- Pitch-shifting architecture for real-time correction
- Pitch-shifting architecture for harmony generation
- Key/scale-detection method
- Harmony representation
- Harmony-to-audio generation method (linked to R-012 — see §6.8)
- Specific harmony interval(s) targeted by the MVP (linked to §6.9)
- ML dataset feasibility
- ML model architecture
- Final GUI framework
- Detailed latency measurement methodology
- Quantitative CPU acceptance thresholds
- Detailed usability evaluation methodology

These items are the intended starting point for the **Feasibility Study**, the next document in the SDLC sequence.


# 20. Changelog

## v6 — Research-Ready Scope and Conceptual Correction (FROZEN)

- Clarified the distinction between the Problem Statement and later SDLC documents.
- Preserved the MVP boundary around one generated harmony voice.
- Added explicit independent-track handling for generated harmony voices.
- Clarified that future harmony voices should become separate editable tracks.
- Added a dedicated harmony-to-audio research question.
- Distinguished harmonic note generation from actual vocal-audio generation.
- Clarified that pitch-shifted harmony audio may retain excessive similarity to the source voice.
- Added a dedicated risk for unnatural generated-voice similarity.
- Added a dedicated risk for incorrect key/scale estimation.
- Strengthened the distinction between musical correctness and perceptual quality.
- Clarified the separation between corrective pitch shifting and harmony-scale pitch shifting.
- Preserved the distinction between uncorrected and pitch-corrected monitoring latency.
- Maintained ML harmony as a research extension rather than an MVP dependency.
- Maintained future multi-voice/SATB harmony as a long-term architectural direction.
- Strengthened the Software Engineering/SDLC context without turning the Problem Statement into an SRS.
- Defined measurable evaluation direction for claims such as latency, accuracy, efficiency, reliability, and naturalness.
- Kept advanced harmonic vocabulary and multi-take comping outside the MVP.

## Freeze note (added on approval)

- Linked the harmony-to-audio open question (§6.8) explicitly to R-012, and flagged it as the first question the Feasibility Study should answer.
- Linked the harmony-interval-selection open question (§6.9) explicitly to the upcoming SRS.
- No structural or scope changes were made — v6's content is otherwise unchanged from the version submitted for approval.

## Amendment 1 (Phase 1, Literature Review)

- Added R-014 (IP/patent risk in pitch-shift-based harmony generation), based on real evidence found during the Literature Review (a US patent architecturally close to the planned rule-based harmony engine).

# 33. Post-Freeze Amendment 2 — Pitch Correction Moved Out of the Real-Time Path

**Date/phase:** during Phase 7 (Technology Selection), following real evidence from the Feasibility Study.

**Change:** automatic pitch correction (and, by extension, harmony audio rendering) is no longer performed inside the real-time monitoring path. The singer records with raw, uncorrected monitoring only. Pitch correction is applied as a post-recording processing step, with no real-time deadline.

**Rationale — grounded in real project evidence, not a preference reversal:**
1. The real-time round-trip latency measured in the Feasibility Study (§1.9: ~15.76–16.00ms) already exceeded the original ≤10ms corrected-monitoring target, and could not be cleanly separated from transducer response — this was a genuinely unresolved, open problem (SRS NFR-RT-002).
2. PSOLA pitch shifting had never been tested inside a live real-time callback at all (Architecture §5.3) — its real-time feasibility was completely unknown, not just imperfect.
3. Post-recording processing removes both problems at once: raw monitoring only needs to solve the easier, already-partially-validated uncorrected-latency problem (§1.9's uncorrected path, closer to the original target), and correction/harmony rendering gain unlimited processing time, enabling higher-quality, look-ahead-capable algorithms that a hard real-time deadline would have prevented.
4. This does not abandon real-time correction permanently — hearing corrected pitch while performing is a legitimate, real feature some vocalists want (comparable to live Auto-Tune monitoring). It is deferred as a later-phase enhancement once post-recording correction is proven, consistent with this project's established pattern of shipping the simpler, solid version first (e.g. one harmony voice before many).

**Documents affected by this change (updated accordingly):** this document (§6.3, §6.12.B, R-009, MVP feature #5), `04_requirements.md` (FR-005), `05_system_analysis.md` (Activity Diagram 4.2), `06_architecture.md` (real-time/non-real-time split).

**What did NOT change:** raw monitoring remains a real-time requirement (the singer must hear themselves without noticeable delay while recording) — only correction and harmony rendering moved to post-processing. Pitch *detection* may still run in real time if useful for live visual feedback, but is no longer required to drive real-time correction.

