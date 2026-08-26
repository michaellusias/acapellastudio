# AcapellaStudio — Complete Project Roadmap

**Document Type:** Master Project Roadmap / SDLC Execution Plan  
**Project:** AcapellaStudio — A Real-Time Vocal Production Environment for A Capella Musicians  
**Status:** Planning Baseline  
**Purpose:** This document defines the project from initial research through implementation, testing, evaluation, documentation, and final delivery.

---

# 1. Project Vision

AcapellaStudio is a standalone, offline desktop application designed specifically for vocal-only music production.

The core workflow is:

```text
Record Vocal
    ↓
Edit / Clean / Correct
    ↓
Select Track
    ↓
Open "Harmonize" Wizard
    ↓
Choose Harmony Style / Voice Configuration
    ↓
Analyze Melody + Key + Rhythm + Vocal Range
    ↓
Generate Harmony Arrangement
    ↓
Generate Separate Vocal Tracks
    ↓
Review / Edit / Regenerate
    ↓
Mix
    ↓
Export
```

The long-term vision is a highly intelligent vocal-arrangement system capable of producing sophisticated multi-voice harmony inspired by advanced contemporary vocal arranging, including dense and extended harmony, altered harmony, passing movement, voice-leading, vocal-range awareness, and deliberate variation between voices.

The project must **not** attempt the ultimate system immediately. Development proceeds from a small, testable MVP toward progressively more sophisticated harmony generation.

---

# 2. Core Product Concept

After a user records a vocal track, they can select that track and choose:

```text
Harmonize
```

A guided wizard appears.

Example:

```text
HARMONIZE VOCAL

Source:
[ Lead Vocal ]

What do you want?

○ Simple Harmony
○ Tight Harmony
○ Double
○ Choir
○ Vocal Section
○ Custom Arrangement
○ Advanced / AI Arrangement
```

The wizard then asks appropriate questions depending on the selected mode.

For example:

```text
Harmony Style

○ Tight
○ Wide
○ Open
○ Parallel
○ Independent
○ Choir-like
○ Bass-supported
○ Custom
```

Voice configuration may include:

```text
Soprano
Mezzo-Soprano
Alto
Tenor
Baritone
Bass
Sub-Bass / Low Layer
```

The system then generates each voice as a **separate editable track**.

Example:

```text
Lead
Soprano
Mezzo
Alto
Tenor
Baritone
Bass
Sub-Bass
```

The number of generated tracks depends on the selected arrangement.

---

# 3. Important Harmony Principle

Generated harmonies must not simply duplicate the melody.

The system should attempt to create musical independence through:

- Different intervals
- Different octave placement
- Different rhythmic behavior where appropriate
- Controlled melodic movement
- Voice-leading
- Vocal-range constraints
- Avoidance of excessive parallelism where inappropriate
- Different note durations where musically appropriate
- Passing tones where supported
- Suspensions and resolutions where appropriate
- Chord extensions in advanced modes
- Deliberate variation between voices

The lead melody remains the primary musical reference, but generated voices should sound like **different singers performing complementary musical parts**, not copies of the lead at another pitch.

This becomes a major evaluation criterion.

---

# 4. Ultimate Harmony Vision

The ultimate research direction is an advanced AI-assisted harmony arranger capable of generating highly sophisticated multi-voice vocal arrangements.

The system should eventually be capable of:

1. Understanding the lead melody
2. Detecting or receiving key/scale information
3. Estimating chords and harmonic context
4. Understanding rhythmic structure
5. Understanding phrase boundaries
6. Estimating vocal range
7. Selecting suitable voice types
8. Generating multiple independent vocal lines
9. Maintaining voice-leading
10. Creating chord extensions
11. Creating passing harmony
12. Creating suspensions
13. Creating altered harmony
14. Creating chromatic movement where musically justified
15. Creating dense vocal clusters where appropriate
16. Creating bass movement
17. Creating low/subharmonic layers where technically safe
18. Synchronizing all generated tracks
19. Avoiding identical copies of the melody
20. Producing separate editable tracks
21. Allowing the user to regenerate individual voices
22. Allowing the user to regenerate the entire arrangement

This is the **ultimate target**, not the MVP.

---

# 5. Subharmonic / Low-Frequency Concept

The system may eventually provide a specialized low-frequency vocal layer.

Possible modes:

```text
Bass
Deep Bass
Sub-Bass
Subharmonic
Vocal Bass Enhancement
```

This feature must be treated carefully because artificially generated subharmonics can introduce:

- Excessive low-frequency energy
- Distortion
- Phase problems
- Speaker/headphone limitations
- Unnatural timbre
- Aliasing or processing artifacts

Therefore, the feature must first be researched and prototyped independently.

It should never be assumed that simply shifting a vocal downward produces a convincing subharmonic.

---

# 6. Development Philosophy

The project follows:

```text
Research
→ Requirements
→ Feasibility
→ Architecture
→ Design
→ Implementation
→ Testing
→ Evaluation
→ Refinement
→ Documentation
→ Final Release
```

Every major feature should eventually have:

```text
Requirement
→ Design
→ Implementation
→ Test
→ Measurement
→ Result
```

Do not build major features merely because they sound interesting.

Each feature must have a reason, implementation plan, and evaluation method.

---

# 7. PHASE 0 — Project Definition

## Goal

Freeze the initial problem, vision, scope, and research direction.

## Tasks

### 0.1 Finalize Problem Statement

Document:

- The problem
- Target users
- Existing workflow limitations
- Why a vocal-first application is useful
- Project objective
- Scope boundaries

### 0.2 Define MVP

The initial MVP should include:

1. Audio input
2. Recording
3. Playback
4. Multitrack
5. YIN pitch detection
6. Automatic pitch correction
7. Manual pitch editing
8. Non-destructive clip editing
9. Key/scale handling
10. One rule-based harmony voice
11. Separate harmony track
12. Basic mixing
13. Vocal percussion recording/looping
14. Export
15. Offline operation

ML harmony remains a later phase.

### 0.3 Define Long-Term Vision

Document:

- Multi-voice harmony
- Harmony wizard
- Multiple voice types
- Choir mode
- Advanced AI arranger
- Sophisticated harmonic vocabulary
- Bass/subharmonic generation
- Independent voice generation
- Regeneration controls

### Deliverable

```text
Problem Statement
Vision
MVP Scope
Scope Boundaries
Research Questions
```

---

# 8. PHASE 1 — Literature Review

## Goal

Understand the existing scientific and technical work before designing algorithms.

This phase is extremely important.

Do not begin by assuming that YIN, PSOLA, phase vocoder, or a particular AI model is the best solution.

## 1.1 Research Areas

Search academic literature for:

### Pitch Detection

- YIN
- pYIN
- SWIPE
- CREPE
- monophonic singing pitch detection
- vocal fundamental frequency estimation

### Pitch Correction

- pitch shifting
- time stretching
- PSOLA
- phase vocoder
- formant preservation
- singing voice transformation
- real-time pitch correction

### Harmony Generation

- automatic harmony generation
- vocal harmony generation
- automatic accompaniment
- music harmonization
- symbolic harmony generation
- computational music theory

### Voice Separation / Vocal Synthesis

- singing voice synthesis
- voice conversion
- vocal timbre transfer
- multi-track vocal generation

### Music Information Retrieval

- key detection
- scale detection
- chord recognition
- melody extraction
- beat tracking
- phrase segmentation

### ML Harmony

- neural harmonization
- symbolic music generation
- sequence-to-sequence music generation
- Transformer music generation
- harmony generation datasets

### Subharmonics

- vocal subharmonic generation
- subharmonic synthesis
- nonlinear vocal acoustics
- low-frequency vocal synthesis

### Real-Time Audio

- real-time audio scheduling
- low latency audio
- audio buffer deadlines
- PipeWire
- JACK
- ASIO
- Core Audio
- WASAPI

## 1.2 Build Literature Matrix

For every important paper record:

```text
Paper
Authors
Year
Problem
Method
Dataset
Advantages
Limitations
Metrics
Relevance to AcapellaStudio
```

## Deliverable

```text
Literature Review
Research Matrix
Identified Research Gap
Candidate Algorithms
```

---

# 9. PHASE 2 — Competitive / Existing-System Analysis

## Goal

Determine what existing software already provides.

Investigate:

- Reaper
- Ableton Live
- FL Studio
- Logic Pro
- Melodyne
- Auto-Tune
- Vocaloid-style systems
- Harmony-generation tools
- Vocal arrangement software
- Existing AI music tools

For each system document:

```text
Feature
Available?
How implemented?
Strength
Weakness
AcapellaStudio opportunity
```

Do not claim that something does not exist without checking current evidence.

## Deliverable

```text
Existing Systems Analysis
Feature Comparison
Gap Analysis
```

---

# 10. PHASE 3 — Feasibility Study

## Goal

Determine what is technically achievable.

This phase prevents the project from becoming impossible.

## 3.1 Audio Feasibility

Test:

- Microphone input
- Sample rates
- Buffer sizes
- Audio backend
- Recording stability
- Playback

Measure:

- Latency
- CPU usage
- Dropouts
- Buffer underruns

## 3.2 Pitch Detection Feasibility

Implement a small prototype.

Test YIN against recorded singing.

Measure:

- Pitch error
- Detection failures
- Vibrato handling
- Breathiness
- Noise sensitivity
- Processing time

Compare with alternatives if necessary.

## 3.3 Pitch-Shifting Feasibility

Prototype candidate methods.

Test:

- Small correction shifts
- Third
- Fifth
- Octave
- Downward shifts

Evaluate:

- Audio quality
- Formant preservation
- Artifacts
- Latency
- CPU usage

This is critical because **pitch correction and harmony generation may require different quality/latency trade-offs**.

## 3.4 Key Detection Feasibility

Test:

```text
Recorded melody
→ pitch extraction
→ note sequence
→ candidate keys
→ confidence
```

Determine when the system should:

```text
Automatically choose key
```

versus:

```text
Ask the user to choose/confirm key
```

## 3.5 Harmony Feasibility

Prototype one harmony voice.

Input:

```text
Melody + Key + Range
```

Output:

```text
Harmony notes
```

Test:

- Thirds
- Fifths
- Diatonic movement
- Voice range
- Voice-leading
- Avoiding identical melodic contours

## 3.6 ML Feasibility

Do not train a large model yet.

First determine:

- Available datasets
- Dataset licensing
- Lead/harmony alignment
- Dataset size
- Computational requirements
- Existing models
- Model formats
- Offline inference options

## Deliverable

```text
Feasibility Study
Algorithm Decision Matrix
Prototype Results
Technical Risks
Recommended Technologies
```

---

# 11. PHASE 4 — Requirements Engineering

## Goal

Convert the research into formal Software Requirements.

Create the SRS.

## Functional Requirements

Examples:

```text
FR-001 Record audio
FR-002 Playback audio
FR-003 Create tracks
FR-004 Detect pitch
FR-005 Correct pitch
FR-006 Manually edit pitch
FR-007 Edit clips
FR-008 Generate harmony
FR-009 Create separate harmony track
FR-010 Mix tracks
FR-011 Export audio
```

Later:

```text
FR-HARM-001 Open harmony wizard
FR-HARM-002 Select harmony style
FR-HARM-003 Select voice types
FR-HARM-004 Select number of voices
FR-HARM-005 Generate separate tracks
FR-HARM-006 Regenerate individual voice
FR-HARM-007 Regenerate entire arrangement
```

## Non-Functional Requirements

Define measurable targets for:

- Latency
- CPU
- Memory
- Reliability
- Audio quality
- Offline operation
- Security/privacy
- Usability

Do not write vague requirements such as:

```text
System shall be fast.
```

Instead define measurable acceptance criteria.

## Deliverable

```text
Software Requirements Specification
```

---

# 12. PHASE 5 — System Analysis

## Goal

Understand how the system behaves before designing implementation.

Create:

### Use Cases

Examples:

```text
Record Vocal
Edit Vocal
Correct Pitch
Manually Edit Pitch
Harmonize Vocal
Edit Harmony
Mix Project
Export Project
```

### Harmony Wizard Use Case

```text
Select Track
→ Open Harmonize
→ Choose Style
→ Choose Voices
→ Set Key
→ Set Vocal Ranges
→ Preview
→ Generate
→ Review
→ Regenerate/Edit
→ Accept
```

### Activity Diagrams

Create diagrams for:

- Recording
- Pitch correction
- Manual editing
- Harmony generation
- Export

### Domain Model

Identify entities:

```text
Project
Track
AudioClip
Take
PitchContour
Note
HarmonyArrangement
HarmonyVoice
MixerChannel
Effect
```

## Deliverable

```text
System Analysis Document
Use Case Model
Activity Diagrams
Domain Model
```

---

# 13. PHASE 6 — System Architecture

## Goal

Design the major software components.

A likely architecture:

```text
                  AcapellaStudio
                       │
              Application Layer
                       │
       ┌───────────────┼────────────────┐
       │               │                │
       ▼               ▼                ▼
 Audio Engine     Harmony Engine    Project Engine
       │               │                │
       ▼               ▼                ▼
 Audio I/O       Rule Engine        Track Manager
 DSP Engine      ML Engine          Clip Manager
 Mixer           Voice Manager       Project Storage
```

The real-time audio path must remain separate from slow operations.

```text
REAL-TIME

Audio Input
→ Audio Callback
→ DSP
→ Mixer
→ Output
```

versus:

```text
NON-REAL-TIME

Analysis
Harmony Generation
ML
Editing
Saving
Export
```

## Deliverable

```text
System Architecture Document
Architecture Diagrams
Component Definitions
Interface Definitions
```

---

# 14. PHASE 7 — Technology Selection

Make final technology decisions based on evidence.

## Language

Rust.

## Audio

Investigate and select:

- CPAL
- PipeWire
- JACK
- ALSA
- WASAPI
- ASIO
- Core Audio

## DSP

Select:

- Pitch detection
- Pitch shifting
- Time stretching
- Formant preservation approach

## GUI

Investigate:

- egui
- iced
- other suitable Rust GUI frameworks

## ML

Later evaluate:

- ONNX Runtime
- Candle
- Burn
- other suitable local inference frameworks

## Storage

Determine project format.

Potential structure:

```text
Project
├── project metadata
├── track definitions
├── source recordings
├── edits
├── pitch data
└── generated audio
```

## Deliverable

```text
Technology Selection Document
Technology Decision Matrix
```

---

# 15. PHASE 8 — Detailed Software Design

## Goal

Convert architecture into implementable modules.

Define modules such as:

```text
audio/
dsp/
pitch/
pitch_edit/
clip/
track/
mixer/
harmony/
project/
export/
gui/
```

Define interfaces.

For example:

```text
PitchDetector
    detect(audio_buffer)

PitchCorrector
    process(audio_buffer, target_pitch)

HarmonyGenerator
    generate(melody, harmony_config)

TrackManager
    create_track()
    delete_track()
    move_track()

ProjectManager
    save()
    load()
```

Define data structures.

## Deliverable

```text
Detailed Design Document
Module Specifications
Data Models
API/Interface Definitions
```

---

# 16. PHASE 9 — UI/UX Design

## Goal

Design the workflow before implementing the GUI.

Core layout:

```text
┌─────────────────────────────────────────────┐
│ Menu / Transport                           │
├─────────────────────────────────────────────┤
│                                             │
│              Timeline / Tracks              │
│                                             │
├─────────────────────────────────────────────┤
│ Inspector / Mixer / Pitch Editor            │
└─────────────────────────────────────────────┘
```

The most important special interface is the Harmony Wizard.

## Harmony Wizard

### Step 1 — Source

```text
Selected Track:
Lead Vocal
```

### Step 2 — Harmony Type

```text
Simple
Tight
Wide
Double
Choir
Vocal Section
Custom
Advanced AI
```

### Step 3 — Voice Configuration

```text
Soprano
Mezzo
Alto
Tenor
Baritone
Bass
Sub-Bass
```

### Step 4 — Number of Voices

```text
1
2
3
4
5
6
Custom
```

### Step 5 — Harmonic Complexity

```text
Basic Diatonic
Extended
Advanced
AI Arrangement
```

### Step 6 — Musical Controls

```text
Key
Scale
Chord Mode
Voice Range
Voicing
Density
Independence
```

### Step 7 — Preview

The user can hear the result before accepting.

### Step 8 — Generate

The system creates:

```text
Harmony 1
Harmony 2
Harmony 3
...
```

as separate tracks.

### Step 9 — Edit

Each generated voice can be:

- muted
- soloed
- pitch edited
- moved
- regenerated
- deleted
- mixed independently

## Deliverable

```text
UI Wireframes
UX Flow
Harmony Wizard Specification
```

---

# 17. PHASE 10 — Development Environment

Set up:

```text
Rust
Cargo
Git
Git repository
IDE
Audio development dependencies
Testing framework
CI if appropriate
```

Create the project structure.

Example:

```text
acapellastudio/
├── Cargo.toml
├── src/
│   ├── audio/
│   ├── dsp/
│   ├── pitch/
│   ├── harmony/
│   ├── tracks/
│   ├── project/
│   ├── mixer/
│   └── gui/
├── tests/
├── docs/
└── assets/
```

Establish coding standards and Git workflow.

## Deliverable

```text
Compiling Empty Application
Version-Controlled Repository
Development Environment
```

---

# 18. PHASE 11 — Audio Engine Prototype

Build the audio engine before the GUI becomes complicated.

## Step 1

Open audio input.

## Step 2

Receive microphone buffers.

## Step 3

Send audio to output.

## Step 4

Measure latency.

## Step 5

Test buffer sizes.

## Step 6

Test sample rates.

## Step 7

Detect underruns.

## Step 8

Add recording.

## Step 9

Add playback.

## Step 10

Add basic multitracking.

## Acceptance

The engine must record and play reliably before advanced DSP is added.

---

# 19. PHASE 12 — Pitch Detection

Implement YIN.

Pipeline:

```text
Audio
→ Preprocessing
→ YIN
→ Fundamental Frequency
→ Frequency → MIDI Note
→ Confidence
```

Test:

- Different singers
- Different registers
- Vibrato
- Quiet singing
- Loud singing
- Breathiness
- Background noise

Record measurable results.

If YIN is inadequate, evaluate alternatives.

## Deliverable

```text
Pitch Detection Module
Benchmark Results
Algorithm Decision
```

---

# 20. PHASE 13 — Automatic Pitch Correction

Implement:

```text
Detected Pitch
→ Target Pitch
→ Pitch Difference
→ Pitch Shift
→ Corrected Audio
```

Add controls:

```text
Key
Scale
Correction Strength
Correction Speed
```

Test:

- Small corrections
- Large corrections
- Vibrato
- Fast passages
- Sustained notes

Measure:

- Pitch error
- Latency
- CPU
- Audio artifacts
- Formant preservation

---

# 21. PHASE 14 — Manual Pitch Editor

Build the pitch visualization.

Possible display:

```text
Pitch
 ↑
 │       ______
 │   ___/      \____
 │__/               \__
 └────────────────────────→ Time
```

Allow:

- Select note
- Drag pitch
- Adjust timing
- Undo
- Redo
- Compare original
- Preview result

Manual changes must remain distinguishable from automatic correction.

---

# 22. PHASE 15 — Non-Destructive Clip Editing

Implement:

- Trim
- Split
- Move
- Delete
- Fade
- Gain
- Undo
- Redo

Original recording remains intact.

The system should store edits as project state rather than permanently modifying the source.

---

# 23. PHASE 16 — Key and Scale Detection

Pipeline:

```text
Vocal
→ Pitch Extraction
→ Note Sequence
→ Pitch-Class Distribution
→ Candidate Keys
→ Confidence
```

If confidence is low:

```text
Possible Key:
C Major
A Minor
G Major

Please select:
[ C Major ]
[ A Minor ]
[ G Major ]
```

The user must always be able to override the detected key.

---

# 24. PHASE 17 — Rule-Based Harmony Engine

Start with exactly one generated harmony voice.

Input:

```text
Melody
Key
Scale
Vocal Range
Harmony Style
```

Process:

```text
Melody
→ Notes
→ Chord/Harmonic Context
→ Candidate Harmony Notes
→ Range Filtering
→ Voice-Leading Rules
→ Melodic Independence Rules
→ Best Candidate
→ Harmony Track
```

The system must not blindly transpose every melody note by the same interval.

The harmony should respond to the underlying scale/chord context.

---

# 25. PHASE 18 — Harmony Wizard MVP

Implement:

```text
Select Track
→ Harmonize
→ Wizard
→ Select Style
→ Select Voice
→ Set Range
→ Choose Key
→ Preview
→ Generate
```

For MVP, the wizard should primarily support:

```text
Simple
Tight
Wide
Double
```

and one generated harmony voice.

The architecture must allow future multi-voice modes.

---

# 26. PHASE 19 — Separate Harmony Tracks

Every generated voice must become an independent track.

Example:

```text
Track 1 — Lead
Track 2 — Harmony
```

Later:

```text
Lead
Soprano
Mezzo
Alto
Tenor
Baritone
Bass
Sub-Bass
```

Each track must support:

- Mute
- Solo
- Volume
- Pan
- Pitch editing
- Clip editing
- Regeneration

---

# 27. PHASE 20 — Vocal Percussion

Implement:

```text
Record Beatbox
→ Create Percussion Track
→ Select Region
→ Loop
→ Arrange
```

The percussion track remains separate from melodic vocal tracks.

---

# 28. PHASE 21 — Mixer

Implement:

```text
Volume
Mute
Solo
Pan
Master Output
```

Later additions may include:

- EQ
- Compression
- Reverb
- Delay
- Vocal effects

Do not allow effects to derail the core project.

---

# 29. PHASE 22 — Export and Project Persistence

Implement:

```text
Save Project
Load Project
Export Audio
```

Ensure that reopening a project preserves:

- Tracks
- Clips
- Positions
- Pitch edits
- Harmony configuration
- Mixer settings

---

# 30. PHASE 23 — ML Harmony Research

Only begin this after the rule-based harmony system works.

## Step 1

Investigate datasets.

## Step 2

Determine whether matched vocal harmony data exists.

## Step 3

Determine licensing.

## Step 4

Choose representation.

Possible representations:

```text
Melody → Harmony MIDI/notes
```

or:

```text
Melody audio → Harmony notes
```

or eventually:

```text
Melody audio → Generated vocal audio
```

Start with symbolic/note-level generation if appropriate because it is substantially easier to control and evaluate.

## Step 5

Create baseline model.

## Step 6

Train.

## Step 7

Evaluate.

## Step 8

Optimize for offline inference.

## Step 9

Integrate only if technically justified.

---

# 31. PHASE 24 — Advanced Multi-Voice Harmony

After one harmony voice works:

```text
1 Voice
→ 2 Voices
→ 3 Voices
→ 4 Voices
→ 5 Voices
→ 6+ Voices
```

Do not jump directly to six voices.

Each voice needs:

```text
Range
Tessitura
Independence
Voice-leading
Collision avoidance
Musical role
```

The arranger should understand that the voices form a **system**, not independent harmonizers operating separately.

---

# 32. PHASE 25 — Advanced Harmony / Ultimate AI Arranger

This phase represents the long-term research goal.

The AI arranger may receive:

```text
Lead Vocal
Key
Scale
Chord Progression
Tempo
Rhythm
Phrase Structure
Voice Ranges
Desired Number of Voices
Harmony Style
```

It should produce:

```text
Soprano
Mezzo
Alto
Tenor
Baritone
Bass
Sub-Bass
```

or another appropriate configuration.

Advanced modes may explore:

- Passing chords
- Secondary dominants
- Modal interchange
- Chromatic movement
- Suspensions
- Extensions
- Altered harmony
- Clusters
- Open voicing
- Close voicing
- Dense arrangements
- Bass motion
- Countermelody
- Rhythmic independence

The project should avoid claiming to reproduce a particular artist exactly. The research objective should instead be defined in terms of measurable musical capabilities and characteristics.

---

# 33. PHASE 26 — Subharmonic Research

Treat this as an independent DSP research branch.

Investigate:

- Natural vocal subharmonics
- Nonlinear synthesis
- Frequency division
- Spectral methods
- Pitch shifting
- Formant/timbre preservation

Test:

```text
Normal Vocal
→ Low Vocal
→ Subharmonic Candidate
```

Evaluate:

- Perceived realism
- Frequency response
- Distortion
- Phase
- CPU
- Stability

Only integrate if quality is acceptable.

---

# 34. PHASE 27 — Testing

Testing must happen continuously, not only at the end.

## Unit Tests

Test:

- Pitch calculations
- MIDI conversion
- Note segmentation
- Harmony rules
- Range filtering
- Voice-leading
- Clip operations
- Track operations
- Project serialization

## Integration Tests

Test:

```text
Recording
→ Pitch Detection
→ Correction
→ Editing
→ Harmony
→ Track Creation
→ Mixing
→ Export
```

## System Tests

Test complete user workflows.

## Regression Tests

Every bug that is fixed should receive a regression test when practical.

---

# 35. PHASE 28 — Real-Time Performance Testing

Measure:

- End-to-end latency
- Audio callback execution time
- CPU utilization
- Memory usage
- Buffer underruns
- Dropouts
- Stability

Test multiple:

```text
44.1 kHz
48 kHz

64 samples
128 samples
256 samples
512 samples
```

Do not assume the application is real-time merely because it runs.

The audio callback must consistently complete before the buffer deadline.

---

# 36. PHASE 29 — Audio Quality Evaluation

Evaluate:

### Pitch Correction

- Pitch accuracy
- Artifacts
- Formant preservation
- Naturalness

### Harmony

- Harmonic correctness
- Voice-leading
- Vocal range
- Melody independence
- Dissonance
- Naturalness

### Multi-Voice

- Voice separation
- Chord quality
- Range conflicts
- Clashes
- Perceptual realism

### Subharmonics

- Low-frequency realism
- Distortion
- Phase behavior
- Speaker safety

---

# 37. PHASE 30 — User Evaluation

Recruit suitable testers if permitted.

Potential groups:

- Singers
- A capella performers
- Music students
- Producers

Give them standardized tasks.

Example:

```text
Record melody
→ Correct pitch
→ Generate harmony
→ Modify harmony
→ Mix
→ Export
```

Measure:

- Task completion
- Time
- Errors
- Satisfaction
- Perceived harmony quality
- Ease of use

---

# 38. PHASE 31 — Security and Privacy Validation

Verify:

- No network dependency
- No unexpected audio transmission
- Local model execution
- Local recording storage
- Project privacy

Test the application while disconnected from the Internet.

---

# 39. PHASE 32 — Documentation

Prepare:

## Technical Documentation

- Architecture
- Algorithms
- Data structures
- APIs
- DSP implementation
- ML implementation

## User Documentation

- Installation
- Recording
- Editing
- Pitch correction
- Harmony wizard
- Mixing
- Export

## Research Documentation

- Literature review
- Methodology
- Experiments
- Results
- Limitations
- Future work

---

# 40. PHASE 33 — Requirements Traceability

Create the final matrix:

| Requirement | Design | Module | Test | Result |
|---|---|---|---|---|
| Recording | Audio Architecture | Audio Engine | TC-001 | Pass/Fail |
| Pitch Detection | DSP Design | Pitch Engine | TC-002 | Pass/Fail |
| Pitch Correction | DSP Design | Corrector | TC-003 | Pass/Fail |
| Manual Editing | Editor Design | Pitch Editor | TC-004 | Pass/Fail |
| Harmony | Harmony Design | Harmony Engine | TC-005 | Pass/Fail |
| Multitrack | Track Design | Track Manager | TC-006 | Pass/Fail |
| Export | Storage Design | Exporter | TC-007 | Pass/Fail |

Every significant requirement must be traceable.

---

# 41. PHASE 34 — Performance Results

Create tables containing actual measured values.

Example:

| Buffer | Sample Rate | Tracks | CPU | Callback Time | Latency | XRuns |
|---:|---:|---:|---:|---:|---:|---:|
| 64 | 48 kHz | 1 | TBD | TBD | TBD | TBD |
| 128 | 48 kHz | 1 | TBD | TBD | TBD | TBD |
| 256 | 48 kHz | 4 | TBD | TBD | TBD | TBD |
| 512 | 48 kHz | 8 | TBD | TBD | TBD | TBD |

Never invent these values.

---

# 42. PHASE 35 — Final Evaluation

Compare the finished system against the original objectives.

Answer:

1. Does recording work?
2. Does pitch detection work?
3. Does automatic correction work?
4. Does manual correction work?
5. Is editing non-destructive?
6. Does harmony generation work?
7. Are generated harmonies musically independent?
8. Are harmony voices separate tracks?
9. Does multitracking work?
10. Does percussion looping work?
11. Does mixing work?
12. Does export work?
13. Does the system operate offline?
14. Does it meet its measured latency requirements?
15. Does it operate reliably?
16. What limitations remain?

---

# 43. PHASE 36 — Final Refinement

Use evaluation results to improve:

- DSP
- UI
- Harmony rules
- Performance
- Error handling
- User workflow

Do not introduce major new features at this stage unless required to correct a critical problem.

---

# 44. PHASE 37 — Final Release Candidate

Freeze:

- Requirements
- Code
- Configuration
- Dependencies
- Documentation

Perform:

```text
Clean Build
→ Install
→ Run
→ Record
→ Edit
→ Harmonize
→ Mix
→ Export
```

on the defined reference environment.

---

# 45. PHASE 38 — Final Project Documentation

Final project package should contain:

```text
01 Problem Statement
02 Literature Review
03 Feasibility Study
04 SRS
05 System Analysis
06 System Architecture
07 Detailed Design
08 UI/UX Design
09 Implementation
10 Testing Strategy
11 Test Results
12 Performance Evaluation
13 ML Research
14 User Evaluation
15 Security/Privacy
16 Limitations
17 Future Work
18 Final Report
19 User Manual
20 Developer Documentation
```

---

# 46. PHASE 39 — Final Demonstration

Demonstrate the complete workflow:

```text
Launch AcapellaStudio
        ↓
Create Project
        ↓
Record Lead
        ↓
View Pitch
        ↓
Automatic Correction
        ↓
Manual Pitch Editing
        ↓
Select Lead
        ↓
Harmonize
        ↓
Harmony Wizard
        ↓
Choose Harmony Style
        ↓
Choose Voice
        ↓
Preview
        ↓
Generate
        ↓
Separate Harmony Track
        ↓
Edit Harmony
        ↓
Add Vocal Percussion
        ↓
Mix
        ↓
Export
```

If advanced harmony has been implemented:

```text
Select Advanced AI Arrangement
        ↓
Analyze Melody
        ↓
Analyze Key / Chords / Rhythm
        ↓
Determine Voice Configuration
        ↓
Generate Multiple Voices
        ↓
Generate Separate Tracks
        ↓
Review / Regenerate
        ↓
Mix
```

---

# 47. Recommended Development Order

The practical coding order is:

```text
1. Project setup
2. Audio input/output
3. Recording
4. Playback
5. Basic multitrack
6. Pitch detection
7. Pitch visualization
8. Automatic pitch correction
9. Manual pitch editing
10. Non-destructive clip editing
11. Key/scale detection
12. Rule-based harmony
13. Harmony Wizard MVP
14. Separate harmony tracks
15. Vocal percussion
16. Mixer
17. Export
18. Project persistence
19. Performance optimization
20. Testing
21. ML feasibility
22. ML prototype
23. Multi-voice harmony
24. Advanced harmony
25. Subharmonic research
26. Final evaluation
27. Documentation
28. Release
```

---

# 48. Critical Rules for the Project

## Rule 1 — Do Not Build Everything at Once

The project must always have a working baseline.

```text
Record → Playback
```

before:

```text
Record → AI Harmony → Advanced Mixing
```

---

## Rule 2 — Research Before Choosing Algorithms

Never decide:

```text
"YIN is definitely the final algorithm."
```

Instead:

```text
YIN
→ Test
→ Compare
→ Decide
```

The same applies to pitch shifting and ML.

---

## Rule 3 — Real-Time Audio Is Special

Never put expensive operations directly into the real-time audio callback.

Avoid:

- File I/O
- Network access
- Blocking locks
- Unbounded computation
- Large allocations
- ML inference

inside the deadline-critical path.

---

## Rule 4 — Harmony Is Not Just Pitch Shifting

A harmony generator must reason about musical structure.

Bad:

```text
Lead + 4 semitones
```

Better:

```text
Melody
→ Scale
→ Chord
→ Candidate Notes
→ Voice Range
→ Voice Leading
→ Musical Independence
→ Harmony
```

---

## Rule 5 — Generated Voices Must Be Independent

A harmony should not simply sound like the lead copied upward/downward.

Evaluate:

```text
Interval movement
Contour similarity
Rhythmic similarity
Repeated notes
Voice-leading
Harmonic function
```

---

## Rule 6 — Keep Generated Voices Editable

Every generated voice should become a real track.

The user must be able to:

```text
Mute
Solo
Edit
Move
Pitch-correct
Delete
Regenerate
Mix
```

---

## Rule 7 — Separate MVP From Vision

MVP:

```text
One strong harmony voice.
```

Vision:

```text
Many intelligent harmony voices.
```

Do not sacrifice the MVP to achieve the vision.

---

# 49. Final Product Evolution

The project should evolve approximately as follows:

```text
LEVEL 1
Vocal Recorder
        ↓
LEVEL 2
Vocal Editor + Pitch Correction
        ↓
LEVEL 3
Rule-Based Harmony
        ↓
LEVEL 4
Harmony Wizard
        ↓
LEVEL 5
Multiple Harmony Voices
        ↓
LEVEL 6
AI-Assisted Harmony
        ↓
LEVEL 7
Advanced Multi-Voice Arranger
        ↓
LEVEL 8
Advanced Harmonic / Vocal Arrangement System
        ↓
LEVEL 9
Research-Level Intelligent Vocal Arrangement
```

---

# 50. Final Definition of Success

AcapellaStudio succeeds as a project if it demonstrates a technically sound, measurable, and reproducible solution to the core problem:

> **A singer can record a vocal performance, correct and edit it, select it, use a guided harmony workflow to generate a musically appropriate independent harmony, receive that harmony as a separate editable track, combine it with additional vocal layers and percussion, mix the arrangement, and export the result — entirely offline within one application.**

The advanced vision extends this toward:

> **An intelligent vocal-arrangement system that can analyze a lead performance and construct a synchronized, multi-voice vocal arrangement with independent voice-leading, appropriate vocal ranges, harmonic sophistication, deliberate variation from the melody, and separate editable tracks.**

The project must reach the first definition before attempting to claim the second.
