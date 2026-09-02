# AcapellaStudio — Project Status

**Current phase:** Phase 6 — System Architecture (drafted)
**Next phase:** Phase 7 — Technology Selection (per master roadmap)

## Frozen / complete
- 00_project_vision.md — long-term vision, Phase 1-5 roadmap, Harmony Wizard concept
- 00b_project_roadmap.md — master SDLC execution plan (authoritative phase sequence)
- 01_problem_statement.md — frozen v6, MVP definition, research questions, risk register (R-001–R-014)
- 02_literature_review.md — COMPLETE — all 8 roadmap research areas
- 02b_competitive_analysis.md — COMPLETE — DAWs, pitch tools, harmony tools, AI vocal tools
- 03_feasibility_study.md — all 6 sections have real content, real hardware measurements,
  real bugs found (PSOLA octave-up failure, harmony range-fallback failure)
- 04_requirements.md — Draft v1 SRS, FR-001 to FR-012 (+FR-008a for the real defect),
  every measurable NFR grounded in real Feasibility Study data
- 05_system_analysis.md — Draft v1 — Use Case Model, Harmony Wizard flow, Activity
  Diagrams, Domain Model, every element tagged with real validation status
- 06_architecture.md — Draft v1 — component diagram, real-time/non-real-time split,
  component definitions, interface definitions (PitchDetector/PitchShifter/
  HarmonyGenerator traits, HarmonyError::NoValidHarmonyForRange mapped directly to the
  real FR-008a defect). Explicitly flags that prototypes use std::sync::Mutex in the
  real-time path (violates NFR-RT-005) and that this must NOT carry into production
  architecture as-is — a genuine self-critical finding, not glossed over.

## Real, open architecture gaps (explicit, tracked)
- Automatic pitch correction ("nearest scale tone" logic) has no design or prototype at all
- Harmony Rule Engine output has never been wired to the Pitch Shifter as one pipeline
- PSOLA has never been tested inside a live real-time audio callback - unknown if fast enough
- Real-time-safe concurrency (lock-free ring buffer) not yet selected - Mutex-based
  prototype approach is a known, acknowledged shortcut, not a production design

## Real defects tracked (carried through SRS, System Analysis, now Architecture)
- FR-008a — harmony range-fallback failure (confirmed real defect)
- PSOLA octave-up failure (no detectable output pitch)
- NFR-RT-002 (latency) — genuinely open, not met or failed
- NFR-AUDIO-003 (formant preservation) — not tested at all

## Not yet started
- Docs 07 through 18 are placeholders pending their respective phases.
- Phase 7 (Technology Selection) not yet started.

## Operating rules in effect
- One step at a time for any command-line/setup instructions
- No fabricated papers, datasets, benchmarks, or measurements — ever; every claim sourced,
  actually run, or flagged as unverified/not-yet-measured
- Real academic/primary sources required for technical claims
- Real-time audio callback isolation rules apply from Phase 1 onward
- Git commit convention: feat/fix/test/docs/refactor, descriptive messages
- Every significant requirement traceable: Requirement → Design → Component → Implementation → Test → Result
