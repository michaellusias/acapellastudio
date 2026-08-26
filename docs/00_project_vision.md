# AcapellaStudio

## A Real-Time Vocal Production Environment for A Capella Musicians

**Document Type:** Project Vision (long-term direction — companion to the frozen Problem Statement)
**Document Status:** Version 1.0 — Initial Baseline
**Working Name:** AcapellaStudio
**Development Approach:** Software Engineering SDLC
**Project Type:** Native, Offline Desktop Application

> **Relationship to the Problem Statement:** This document is the project's long-term vision and direction. It does **not** redefine the MVP. The MVP boundary remains exactly as frozen in `01-problem-statement.md` (v6) — one rule-based harmony voice, diatonic triadic, vocal-range aware. Everything here beyond that MVP (multi-voice arrangement, the Harmony Wizard's full option set, subharmonic generation, AI-assisted arrangement) is explicitly Phase 2 and beyond, per §23. When this document and the Problem Statement appear to disagree on scope, the Problem Statement's MVP definition governs what gets built first.

---

# 1. Project Overview

AcapellaStudio is a native, offline desktop application designed specifically for a capella musicians and vocal-only music production.

The application is intended to provide an integrated environment for recording, editing, tuning, harmonizing, arranging, and mixing vocal performances without requiring instrumental accompaniment or multiple disconnected software tools.

The long-term vision of AcapellaStudio extends beyond conventional pitch correction or simple harmony generation. The system aims to investigate intelligent multi-voice vocal arrangement, in which a single recorded vocal performance can be analyzed and transformed into a complex, musically coherent arrangement containing multiple independent vocal parts.

The project will initially prioritize a realistic and demonstrable MVP while maintaining a clearly defined long-term research direction toward advanced AI-assisted vocal arrangement.

---

# 2. Background

A capella music is a form of musical performance in which musical material is produced primarily or entirely using the human voice.

Typical a capella arrangements may contain:

- Lead vocals
- Harmony vocals
- Bass vocals
- Vocal doubling
- Background vocals
- Call-and-response parts
- Vocal percussion / beatboxing
- Extended multi-voice arrangements

Producing a polished a capella recording can therefore require a performer to act simultaneously as singer, vocal arranger, recording engineer, pitch editor, harmony arranger, and mixer.

A solo performer who wants to create a complete vocal arrangement may need to record several performances, manually tune them, create harmony parts, synchronize them, edit individual recordings, and mix the resulting layers.

Existing Digital Audio Workstations provide powerful recording, editing, and mixing functionality, but they are designed primarily as general-purpose music-production environments.

Dedicated vocal-processing tools provide sophisticated pitch editing and correction, but they are generally focused on individual vocal processing rather than complete a capella arrangement.

Harmony-generation systems also exist in various forms, but sophisticated multi-voice vocal arrangement remains a difficult computational music problem.

AcapellaStudio therefore investigates whether these tasks can be integrated into one coherent, vocal-first desktop workflow.

---

# 3. Problem Statement (Summary)

> How can a standalone desktop application provide a low-latency vocal recording and pitch-processing environment, while integrating manual editing, automated harmony generation, multi-track vocal production, and eventually intelligent multi-voice vocal arrangement into a unified workflow specifically designed for a capella musicians?

The full research-question breakdown for the MVP lives in the frozen Problem Statement (§6 of `01-problem-statement.md`). This section is retained here only as context for the vision that follows.

---

# 4. Target Users

**Solo A Capella Artists** — perform multiple vocal parts themselves, need harmony/arrangement assistance.
**Small Vocal Groups** — need a lightweight recording/arranging/editing/mixing environment.
**Vocal Percussionists / Beatboxers** — record and arrange vocal percussion alongside melodic parts.
**Music Students and Hobbyists** — experiment with vocal harmony/arrangement without advanced DAW knowledge.

---

# 5. Project Objective

    Record → Detect Pitch → Correct / Edit → Harmonize → Arrange → Mix → Export

The application shall initially focus on a practical MVP while establishing an architecture and research direction capable of eventually supporting sophisticated AI-assisted multi-voice vocal arrangement.

---

# 6. Core Functional Vision

**Vocal Recording / Multitrack Recording** — record and manage independent vocal tracks.

**Real-Time Pitch Detection** — YIN initially; alternatives investigated if insufficient.

**Automatic Pitch Correction** — moves detected notes toward appropriate pitches, with user control over correction strength/speed/key/scale/target behavior. Primarily for small corrective movements.

**Manual Pitch Editing** — visual pitch-editing environment: view contour, identify notes/regions, move pitch, adjust boundaries, override automatic correction. Non-destructive.

---

# 7. Non-Destructive Audio Editing

Trim, split/snip, delete, move, fade in/out, gain adjustment, undo/redo. The original recorded audio remains recoverable — editing operations modify the project's representation, not the source recording.

---

# 8. Key and Scale Detection

A monophonic vocal melody does not always provide sufficient information to determine a unique musical key. Key/scale detection is treated as a distinct research problem: analyze the melody, estimate candidate keys, present the most likely result, indicate uncertainty, and allow user override. The confirmed key/scale feeds the harmony-generation system.

---

# 9. Harmony Generation

Not limited to fixed-interval transposition. The harmony engine should consider melody, key, scale, chord context, interval relationships, voice range, tessitura, voice-leading, harmonic function, rhythm, phrase boundaries, cadences, and user-selected harmony style.

---

# 10. Harmony Wizard *(concrete, implementable UX concept — worth carrying into UI Design / SRS)*

After recording, the user selects a track and chooses "Harmonize," opening a guided wizard rather than an immediately-fixed result.

**10.1 Harmony Type** — single, double, multiple, choir-style, background arrangement, call-and-response, adaptive/AI-selected.

**10.2 Vocal Role / Range** — soprano, mezzo-soprano, alto, tenor, baritone, bass, or custom range — acts as a generation constraint.

**10.3 Harmony Position** — above / below / mixed / adaptive.

**10.4 Harmony Density** — tight / moderate / open / wide / adaptive.

**10.5 Musical Character** — simple, supportive, rich, choir-like, bright, dark, dense, sparse.

**10.6 Voice Independence** — controls how closely the harmony follows the source melody, from tightly parallel to more independently moving, while staying within musical constraints.

---

# 11. Harmony Preview and Regeneration

Preview, regenerate, compare alternatives, modify parameters, cancel, or accept. Generated harmonies are independent tracks; the original vocal is never altered.

---

# 12. Harmony Independence

A core design requirement: generated harmony should not sound like the melody with pitch shifted by a fixed interval. It should be **musically related, but independently meaningful** — actual notes shaped by chord, key, scale, voice range, voice-leading, harmonic function, and phrase, not a rigid parallel transposition.

---

# 13. Controlled Voice Humanization

An optional future control affecting timing, note duration, articulation, pitch expression, vibrato, melodic movement, dynamics — without intentionally introducing musical errors. Preserves pitch correctness, harmonic correctness, rhythm, and voice-leading while allowing natural-feeling variation between voices.

---

# 14. Generated Harmony Tracks

Every generated harmony voice becomes an independent track (edit, tune, mute, solo, pan, mix, delete, regenerate independently) — never forced into one rendered/merged audio file.

---

# 15. Multi-Voice Harmony Vision *(Phase 2+, not MVP)*

Long-term: multiple independent vocal parts from one source (soprano, mezzo, alto, tenor 1/2, baritone, bass, subharmonic). The number and type of voices generated should depend on musical context, arrangement density, user preference, vocal range, harmonic complexity, and available compute — not generated simply because they're technically available.

---

# 16. Intelligent Voice Allocation *(Phase 4/5, research-grade — see RQ9)*

Eventually: how many voices are appropriate, which ranges to occupy, which voices double the melody vs. carry chord tones, when voices enter/leave, when unison or octave doubling is appropriate, how voices distribute across the register. This is the step that would turn the system from a "harmony generator" into an "intelligent vocal arrangement system" — and is genuinely research-grade automated-arranging work, not a scoped engineering task.

---

# 17. Musical Synchronization

All generated voices stay synchronized to the project timeline (tempo, beat, onset, duration, chord changes, phrase boundaries, cadences) — but synchronization doesn't mean identical rhythm across voices. Advanced arrangements may have different entrances, sustained notes, anticipations, delayed entrances, subdivisions, call-and-response, independent movement. The system distinguishes **temporal synchronization** from **musical independence**.

---

# 18. Advanced Harmony Vocabulary *(deferred)*

MVP focuses on diatonic harmony. Long-term research directions: passing chords, suspensions, passing/neighbor tones, secondary dominants, chord extensions, altered/chromatic chords, modal interchange, reharmonization, advanced voice-leading, upper-structure harmony, controlled dissonance/resolution.

---

# 19. Subharmonic and Bass Generation *(deferred, flagged as genuinely hard)*

Dedicated low-frequency vocal generation for bass vocals, low-octave effects, subharmonic effects, deep arrangements. **Not** assumed to be ordinary downward pitch shifting — candidate techniques include subharmonic synthesis, frequency division, nonlinear processing, low-octave generation, spectral processing, formant-preserving processing. Evaluated for pitch accuracy, naturalness, formant behavior, low-frequency stability, CPU, latency, phase interaction.

**Caution carried over from review:** convincing vocal subharmonics are notoriously hard even for mature commercial tools, because real vocal fry/subharmonic technique is something a singer performs, not purely a DSP effect on a normal take. This should stay firmly in "open research" status and not get quietly promoted to a planned feature without a dedicated feasibility check first.

---

# 20. AI-Assisted Harmony Generation *(long-term)*

    Original Vocal → Audio Analysis → Pitch/Melody → Rhythm Analysis → Key/Scale
    → Chord/Harmonic Context → Phrase Structure → Vocal Range Constraints
    → User Preferences → AI Arrangement Model → Multi-Voice Arrangement
    → Voice Allocation → Voice Rendering → Independent Vocal Tracks

---

# 21. Musical Representation for AI

Where practical, the AI generates an intermediate musical representation (pitch, onset, duration, voice identity, range, harmonic function, chord relationship, articulation, expression, optional timing variation) rather than raw finished audio — for better editability, explainability, user control, debuggability, and constraint handling than a black-box audio generator.

---

# 22. Advanced AI Vocal Arrangement Vision

Long-term research target: complex harmony, independent voice-leading, extended harmony, rich voicing, controlled dissonance, chromatic movement, reharmonization, rhythmic independence, dynamic voice allocation, bass movement, doubling, octave relationships, call-and-response, expressive variation, large-scale development.

The musical sophistication reference point includes advanced contemporary vocal arranging and the harmonic complexity associated with artists such as Jacob Collier — **a sophistication target, not an objective to reproduce or imitate a specific artist's recordings or proprietary creative process.** The research objective is identifying the underlying musical concepts and computational techniques capable of producing sophisticated arrangements.

---

# 23. Progressive Multi-Voice Development

**Phase 1** — One source vocal → one rule-based harmony *(this is the current MVP — see frozen Problem Statement)*
**Phase 2** — One source vocal → multiple rule-based harmonies
**Phase 3** — One source vocal → ML-assisted harmony
**Phase 4** — One source vocal → ML-assisted multi-voice arrangement
**Phase 5** — One source vocal → advanced intelligent vocal orchestration

Phase 5 represents the ultimate project vision and is not guaranteed to be completed within the initial project timeline.

---

# 24. MVP Definition (Reference Only — Authoritative Copy Lives in the Problem Statement)

This MVP list matches `01-problem-statement.md` §7 exactly and is repeated here only for continuity of this document. **If the two ever diverge due to a future edit, the Problem Statement governs.**

1. Vocal recording · 2. Multitrack recording · 3. Vocal playback · 4. Real-time pitch detection · 5. Automatic pitch correction · 6. Manual pitch editing · 7. Non-destructive clip editing (trim, split/snip, delete, move, fade, gain, session undo/redo) · 8. Key/scale detection with user override · 9. One rule-based harmony voice (diatonic triadic, vocal-range aware, basic voice-leading) · 10. Harmony as an independent editable track · 11. Vocal percussion recording and looping · 12. Basic mixing (volume, mute, solo, pan) · 13. Audio export · 14. Offline operation.

---

# 25. Deferred Features

ML harmony generation; multiple AI-generated harmony voices; full SATB arrangement generation; advanced multi-voice AI arrangement; Jacob-Collier-level harmonic complexity; advanced reharmonization; modal interchange; advanced chromatic harmony; complex chord extensions; intelligent dynamic voice allocation; advanced subharmonic generation; multi-take comping; cross-session editing history; cross-platform validation beyond the primary development OS; professional mastering functionality.

---

# 26. Scope Philosophy

    Working Core → Reliable DSP → Rule-Based Harmony → Multi-Voice Expansion → ML Assistance → Advanced AI Arrangement

A successful MVP with one reliable harmony voice is preferable to an incomplete system attempting six or more voices without sufficient musical or technical quality.

---

# 27. Audio Performance Vision

**Uncorrected monitoring**: target ≤10ms end-to-end, subject to experimental validation.
**Corrected monitoring**: may be higher due to algorithmic delay from pitch-shifting; the acceptable target is established experimentally after the pitch-processing approach is selected. The two are not treated as identical (see Problem Statement §2.10 / §6.12).

---

# 28. Offline and Privacy Requirements

Core functionality requires no cloud processing, internet connectivity, online audio upload, or remote AI services. Recorded vocals stay on the user's machine unless explicitly exported. Future ML models are designed for local/offline inference where feasible.

---

# 29. Software Engineering Principle

Structured SDLC: Problem Definition → Requirements Engineering → Feasibility Study → System Analysis → Architecture & Design → Implementation → Unit Testing → Integration Testing → System Testing → Performance Evaluation → User Evaluation → Refinement. Every major requirement traceable through Requirement → Design → Implementation → Test → Result.

---

# 30. Major Research Questions

**RQ1** Pitch detection accuracy (YIN/alternatives) on realistic singing.
**RQ2** Pitch correction: low latency + naturalness + formant preservation.
**RQ3** Manual pitch editing: non-destructive, acceptable audio quality.
**RQ4** Key/scale inference reliability from a single monophonic melody.
**RQ5** Rule-based harmony: musically appropriate, range/voice-leading aware.
**RQ6** Harmony independence: avoiding simple transposed-copy sound while staying coherent.
**RQ7** Multi-voice arrangement: one melody → multiple independently functioning parts.
**RQ8** AI harmony: ML generating musically meaningful harmony beyond strict rules.
**RQ9** *(research-grade)* AI arrangement: can a system determine voice count, ranges, entrances/exits, independence, voicing/distribution — not just which notes harmonize.
**RQ10** Subharmonics: convincing bass/subharmonic vocal generation with preserved musical/perceptual quality.
**RQ11** Real-time performance: strict deadlines with stable recording/playback.
**RQ12** Offline AI: sophisticated harmony models running locally within acceptable resource limits.

---

# 31. Major Project Risks

R-001 Excessive audio latency · R-002 Audio dropouts · R-003 Poor pitch detection · R-004 Unnatural pitch correction · R-005 Large-interval pitch-shift quality · R-006 ML inference performance · R-007 ML dataset availability · R-008 Excessive project scope · R-009 Harmony independence (sounds like a transposition) · R-010 Poor multi-voice arrangements (technically correct, musically incoherent) · R-011 Subharmonic artifacts · R-012 AI black-box behavior (unpredictable, hard-to-edit results).

*(These map directly onto R-001–R-013 in the frozen Problem Statement; not duplicated in detail here to avoid two diverging risk registers — the Problem Statement's risk list is the one to update going forward.)*

---

# 32. Long-Term Vision

> A local AI-assisted vocal production environment capable of transforming a single recorded vocal performance into a complete, editable, synchronized, multi-voice a capella arrangement.

The system should eventually determine what harmony is appropriate, how many voices are needed, what ranges they occupy, what each voice sings, how each voice moves, when each enters/leaves, arrangement density, whether to add bass/subharmonics, where voices double vs. diverge, and how the arrangement develops dynamically — producing not a single rendered file but an editable arrangement of independent tracks (Lead, Soprano, Mezzo, Alto, Tenor 1/2, Baritone, Bass, Subharmonic, Vocal Percussion). The AI assists with arrangement; the user remains the final decision-maker.

---

# 33. Final Project Vision Statement

The immediate objective is a reliable vocal recording, editing, pitch-processing, and rule-based harmony system.

> The long-term objective: to investigate whether a locally running intelligent system can transform a single human vocal performance into a sophisticated, multi-layered, musically coherent a capella arrangement containing independently functioning vocal voices while preserving user control and editability.

The project combines Software Engineering, Real-Time Systems, Digital Signal Processing, Music Information Retrieval, Computational Music Theory, Machine Learning, Audio Engineering, and Human-Computer Interaction. The final system should attempt to understand the musical role of each generated voice, not merely produce "more vocals."

---

# 34. Document Status and Relationship to Other Documents

**Version:** 1.0 — Initial Baseline (Project Vision)

This document establishes long-term direction, research questions, and phased ambition. It works alongside, and does not replace:

- `01-problem-statement.md` — **frozen (v6)** — the authoritative near-term MVP definition and risk register.

Subsequent SDLC documents (Feasibility Study, Stakeholder Analysis, Requirements Specification, SRS, Use Case Specification, System Analysis, System Architecture, Detailed Design, Database/Project Data Design, Audio Engine Design, Harmony Engine Design, AI/ML Research and Design, UI Design, Testing Strategy, Performance Evaluation Plan, Risk Management, Implementation Plan, Deployment Plan, User Evaluation, Final System Documentation) should treat the Problem Statement as the near-term contract and this Vision document as the direction those decisions should stay compatible with, without pulling Phase 2+ scope into MVP-level documents prematurely.
