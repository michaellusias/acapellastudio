# AcapellaStudio — Project Status

**Current phase:** Phase 4 — Requirements Engineering (SRS drafted)
**Next phase:** Phase 5 — System Analysis (Use Cases, per master roadmap)

## Frozen / complete
- 00_project_vision.md — long-term vision, Phase 1-5 roadmap, Harmony Wizard concept
- 00b_project_roadmap.md — master SDLC execution plan (authoritative phase sequence)
- 01_problem_statement.md — frozen v6, MVP definition, research questions, risk register (R-001–R-014)
- 02_literature_review.md — COMPLETE — all 8 roadmap research areas
- 02b_competitive_analysis.md — COMPLETE — DAWs, pitch tools, harmony tools, AI vocal tools
- 03_feasibility_study.md — all 6 sections have real content (audio I/O, pitch detection, pitch
  shifting, key detection, harmony, ML datasets) — real hardware measurements throughout,
  real bugs found and documented (PSOLA octave-up failure, harmony range-fallback failure)
- 04_requirements.md — Draft v1 SRS — FR-001 through FR-012 (+FR-008a for the real defect
  found), NFRs explicitly grounded in real Feasibility Study measurements with honest
  met/not-met/not-yet-measured status per requirement, not silently carried forward

## Real defects tracked (from Feasibility Study, now formalized in SRS)
- **FR-008a — harmony range-fallback failure**: confirmed real defect, not a hypothetical risk.
- **PSOLA octave-up failure**: pitch shifting produces no detectable output at +1200 cents.
- **NFR-RT-002 (≤10ms latency)**: genuinely open — real measurement (~15.88ms) can't yet be
  cleanly separated from transducer response, so neither "met" nor "failed" is honest yet.
- **NFR-AUDIO-003 (formant preservation)**: not tested at all — sine-wave test signal used so
  far has no formants to test against.

## Key findings so far (cumulative, unchanged from prior status, still relevant)
- R-014 IP/patent risk (US Patent 8,168,877/8,618,402) — possibly connected to Antares, unconfirmed
- Real user feedback confirms "mechanical harmony" risk is real even in market-leading tools
- No existing tool combines harmony-from-own-voice + offline + a capella workflow — confirmed gap

## Not yet started
- Docs 05 through 18 are placeholders pending their respective phases.
- Phase 5 (System Analysis / Use Cases) not yet started.

## Operating rules in effect
- One step at a time for any command-line/setup instructions
- No fabricated papers, datasets, benchmarks, or measurements — ever; every claim sourced,
  actually run, or flagged as unverified/not-yet-measured
- Real academic/primary sources required for technical claims
- Real-time audio callback isolation rules apply from Phase 1 onward
- Git commit convention: feat/fix/test/docs/refactor, descriptive messages
- Every significant requirement traceable: Requirement → Design → Component → Implementation → Test → Result
