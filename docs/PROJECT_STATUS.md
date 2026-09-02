# AcapellaStudio — Project Status

**Current phase:** Phase 7 — Technology Selection (drafted; roadmap Phase order differs
slightly from the charter's file-numbering skeleton — Tech Selection content lives in
08_technology_selection.md per the charter's naming, even though it's roadmap Phase 7,
before Phase 8/Detailed Design which will use 07_detailed_design.md)
**Next phase:** Phase 8 — Detailed Software Design (per master roadmap)

## Frozen / complete
- 00_project_vision.md, 00b_project_roadmap.md, 01_problem_statement.md (frozen v6)
- 02_literature_review.md, 02b_competitive_analysis.md — COMPLETE
- 03_feasibility_study.md — all 6 sections, real hardware measurements, real bugs found
- 04_requirements.md — Draft v1 SRS, grounded in real Feasibility Study data
- 05_system_analysis.md — Draft v1 — Use Cases, Harmony Wizard flow, Activity Diagrams, Domain Model
- 06_architecture.md — Draft v1 — components, interfaces, real self-critical Mutex/NFR-RT-005 finding
- 08_technology_selection.md — Draft v1 — see below for real decision status

## Technology Selection real status (only 2 of 6 areas fully "Selected")
- **Selected:** Language (Rust) — 5 working prototypes back this
- **Selected:** Audio backend (cpal + native PipeWire, Linux only) — real hardware tests
- **Selected, with open item:** Pitch detection (FFT-YIN) — plan-caching fix unconfirmed on hardware
- **Open:** Pitch shifting (PSOLA leading candidate) — confirmed octave-up bug, zero formant testing
- **Open:** Time stretching — no work done at all
- **Open:** Formant preservation — no work done at all, arguably the most important open DSP question
- **Open:** GUI framework — Iced is leading candidate (real evidence: iced_audio extension exists,
  egui's immediate-mode redraw-every-frame model is a real CPU-budget concern given our DSP already
  uses meaningful CPU) — but this is desk research, not a project-tested decision
- **Deferred:** ML framework — intentionally, not MVP scope
- **Proposed, untested:** Storage format — directory-based project structure, domain-model-grounded
  design, no implementation yet

## Real, open architecture/technology gaps (explicit, tracked, cumulative)
- Automatic pitch correction ("nearest scale tone" logic) has no design or prototype at all
- Harmony Rule Engine output never wired to the Pitch Shifter as one pipeline
- PSOLA never tested inside a live real-time audio callback
- Real-time-safe concurrency (lock-free ring buffer) not yet selected - prototypes use Mutex
- Formant preservation: zero testing exists anywhere in the project
- GUI framework: zero prototyping exists, decision is desk-research-only

## Real defects tracked (cumulative)
- FR-008a — harmony range-fallback failure (confirmed real defect)
- PSOLA octave-up failure (no detectable output pitch)
- NFR-RT-002 (latency) — genuinely open, not met or failed
- NFR-AUDIO-003 (formant preservation) — not tested at all

## Not yet started
- Docs 07 (detailed design, roadmap Phase 8), 09-18 are placeholders.

## Operating rules in effect
- One step at a time for any command-line/setup instructions
- No fabricated papers, datasets, benchmarks, or measurements — ever; every claim sourced,
  actually run, or flagged as unverified/not-yet-measured/open
- Real academic/primary sources required for technical claims
- Real-time audio callback isolation rules apply from Phase 1 onward
- Git commit convention: feat/fix/test/docs/refactor, descriptive messages
- Every significant requirement traceable: Requirement → Design → Component → Implementation → Test → Result
