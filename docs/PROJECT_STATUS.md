# AcapellaStudio — Project Status

**Current phase:** Phase 9 — UI/UX Design (drafted)
**Next phase:** Phase 10 — Development Environment setup (per master roadmap)

## Frozen / complete
- 00_project_vision.md, 00b_project_roadmap.md, 01_problem_statement.md (frozen v6 + Amendments 1-2)
- 02_literature_review.md, 02b_competitive_analysis.md — COMPLETE
- 03_feasibility_study.md — all 6 sections, real hardware measurements, real bugs found
- 04_requirements.md — Draft v1 SRS, amended for post-recording correction
- 05_system_analysis.md — Draft v1, amended for post-recording correction
- 06_architecture.md — Draft v1, amended for post-recording correction
- 07_detailed_design.md — Draft v1 — 12 module specs, real bugs drive interface design
- 08_technology_selection.md — Draft v1 — only Language + Audio backend fully "Selected"
- 08b_ui_ux_design.md — Draft v1 — wireframe, UX flow, full 9-step Harmony Wizard spec
  from the roadmap with MVP/DEFERRED tags per step. Real finding: only a minority of the
  roadmap's 9 wizard steps have any real logic behind them. Simplified 4-6 step MVP flow
  proposed instead of the full 9-step version. Flagged Step 7 (Preview) as a genuine
  blocker - harmony-to-audio rendering doesn't exist yet, so the wizard can't preview
  its own output even for the MVP.

## Repo status
- Pushed to GitHub: https://github.com/michaellusias/acapellastudio (via SSH)
- Working copy: ~/Documents/acapella-daw on Michael's machine

## Real, open architecture/technology gaps (explicit, tracked, cumulative)
- Automatic pitch correction ("nearest scale tone" logic) still has no design or prototype
- Harmony Rule Engine output never wired to the Pitch Shifter — this is also why the
  Harmony Wizard's Preview step (Step 7) can't work yet, a concrete UX consequence of
  an architecture gap, not just an abstract one
- Real-time-safe concurrency (lock-free ring buffer) not yet selected
- Formant preservation: zero testing exists anywhere in the project
- GUI framework: zero prototyping exists, gui/ module and all UI work blocked on this
- mixer/ module: stub only — also blocks most of Harmony Wizard Step 9 (Edit)

## Real defects tracked (cumulative)
- FR-008a — harmony range-fallback failure → HarmonyError::NoValidHarmonyForRange in design
- PSOLA octave-up failure → PitchShiftError::UnsupportedRatio in design
- NFR-RT-002 (latency) — genuinely open, now the ONLY real-time latency target
- NFR-AUDIO-003 (formant preservation) — not tested at all

## Not yet started
- Docs 09-18 (charter skeleton: dsp_design, harmony_engine, ml_research, testing_strategy,
  performance_evaluation, audio_quality_evaluation, user_evaluation, risk_register,
  traceability_matrix, changelog) are placeholders.
- Phase 10 (Development Environment) not yet started — though in practice, Rust/Cargo/Git
  are already real and working per the Feasibility Study prototypes; this phase is mostly
  about formalizing project structure, not starting from zero.

## Operating rules in effect
- One step at a time for any command-line/setup instructions
- No fabricated papers, datasets, benchmarks, or measurements — ever; every claim sourced,
  actually run, or flagged as unverified/not-yet-measured/open
- Real academic/primary sources required for technical claims
- Real-time audio callback isolation rules apply from Phase 1 onward
- Git commit convention: feat/fix/test/docs/refactor, descriptive messages
- Every significant requirement traceable: Requirement → Design → Component → Implementation → Test → Result
