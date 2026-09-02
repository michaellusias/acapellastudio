# AcapellaStudio — Project Status

**Current phase:** Phase 5 — System Analysis (drafted)
**Next phase:** Phase 6 — System Architecture (per master roadmap)

## Frozen / complete
- 00_project_vision.md — long-term vision, Phase 1-5 roadmap, Harmony Wizard concept
- 00b_project_roadmap.md — master SDLC execution plan (authoritative phase sequence)
- 01_problem_statement.md — frozen v6, MVP definition, research questions, risk register (R-001–R-014)
- 02_literature_review.md — COMPLETE — all 8 roadmap research areas
- 02b_competitive_analysis.md — COMPLETE — DAWs, pitch tools, harmony tools, AI vocal tools
- 03_feasibility_study.md — all 6 sections have real content, real hardware measurements,
  real bugs found (PSOLA octave-up failure, harmony range-fallback failure)
- 04_requirements.md — Draft v1 SRS, FR-001 to FR-012 (+FR-008a for the real defect),
  every measurable NFR grounded in real Feasibility Study data with honest met/not-met/
  not-yet-measured status
- 05_system_analysis.md — Draft v1 — Use Case Model (UC-001 to UC-008), detailed Harmony
  Wizard use case, 5 Activity Diagrams, Domain Model (entities + relationships). Every
  use case/diagram step tagged with real validation status (tested prototype vs.
  entirely unimplemented) rather than treated as uniformly theoretical.

## Real defects tracked (carried through SRS and now System Analysis)
- FR-008a — harmony range-fallback failure (confirmed real defect)
- PSOLA octave-up failure (no detectable output pitch)
- NFR-RT-002 (latency) — genuinely open, not met or failed
- NFR-AUDIO-003 (formant preservation) — not tested at all

## Key findings so far (cumulative, unchanged, still relevant)
- R-014 IP/patent risk (US Patent 8,168,877/8,618,402) — possibly connected to Antares, unconfirmed
- Real user feedback confirms "mechanical harmony" risk is real even in market-leading tools
- No existing tool combines harmony-from-own-voice + offline + a capella workflow — confirmed gap

## Not yet started
- Docs 06 through 18 are placeholders pending their respective phases.
- Phase 6 (System Architecture) not yet started.

## Operating rules in effect
- One step at a time for any command-line/setup instructions
- No fabricated papers, datasets, benchmarks, or measurements — ever; every claim sourced,
  actually run, or flagged as unverified/not-yet-measured
- Real academic/primary sources required for technical claims
- Real-time audio callback isolation rules apply from Phase 1 onward
- Git commit convention: feat/fix/test/docs/refactor, descriptive messages
- Every significant requirement traceable: Requirement → Design → Component → Implementation → Test → Result
