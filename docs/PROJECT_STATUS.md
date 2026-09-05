# AcapellaStudio — Project Status

**Current phase:** Phase 10 — Development Environment (real workspace built and partially
verified)
**Next phase:** Phase 11 — Audio Engine Prototype (per master roadmap — largely already
covered by Feasibility Study work; remaining: wire recording/playback/multitracking into
the new real project structure)

## Real project structure now exists: acapellastudio/

Consolidated from 5 scattered prototype folders into one real Cargo project matching
07_detailed_design.md's module layout exactly:

```
acapellastudio/
├── Cargo.toml
├── src/
│   ├── lib.rs, main.rs
│   ├── audio/          — REAL, ported from audio-io-prototype
│   ├── dsp/pitch.rs     — REAL, ported from yin-fft-prototype, planner-caching fix applied
│   ├── dsp/pitch_shift.rs — REAL, ported from psola-prototype, octave-up bug now Result::Err
│   ├── harmony/key.rs   — REAL, ported from key-detection-prototype
│   ├── harmony/rules.rs — REAL, ported from harmony-prototype, range-fallback bug now Result::Err
│   ├── pitch_edit/, clip/, track/, mixer/, project/, export/ — honest stubs, no real logic
│   └── (no gui/ module - not declared, since no framework selected yet)
```

## Real verification status
- **VERIFIED in sandbox:** harmony::key and harmony::rules (zero external deps) — isolated
  and tested separately since this sandbox's Rust 1.75 can't resolve cpal/rustfft's current
  dependency tree (needs edition2024, same issue hit earlier in the project). All 4 tests
  pass, INCLUDING the critical regression test confirming FR-008a now returns
  Err(NoValidHarmonyForRange) instead of the old silent unison fallback.
- **NOT YET VERIFIED on Michael's machine:** the full workspace including audio/ and
  dsp/pitch.rs (cpal + rustfft dependent) — needs a real `cargo build` + `cargo test` on
  the reference hardware, which is where the authoritative verification belongs anyway
  (per the project's established pattern throughout Phase 3).
- Real unit tests added as regression guards for: 440Hz detection accuracy (<5 cents),
  octave-up rejection, correction-scale shift success, C major key detection, E minor
  narrow-margin key detection, diatonic harmony generation, range-fallback Err behavior.

## CI added
- .github/workflows/ci.yml — real GitHub Actions workflow, installs the EXACT system
  dependencies discovered necessary during Feasibility Study §1.1 (libasound2-dev,
  libpipewire-0.3-dev, clang+libclang-dev), then cargo build + cargo test.
- Audio engine's own test is designed to not panic even without real audio hardware
  (CI runners won't have a real mic) - this was a deliberate design choice, not an
  oversight, so CI can pass honestly without pretending a virtual runner has audio.

## Real, open architecture/technology gaps (explicit, tracked, cumulative)
- Automatic pitch correction ("nearest scale tone" logic) still has no design or prototype
- Harmony Rule Engine output never wired to the Pitch Shifter as one pipeline
- Real-time-safe concurrency (lock-free ring buffer) not yet selected — AudioSink trait
  added specifically to make this boundary explicit, but no lock-free implementation exists
- Formant preservation: zero testing exists anywhere in the project
- GUI framework: zero prototyping exists, no gui/ module declared at all yet
- mixer/, project/, export/, clip/, track/, pitch_edit/: stub-only, no real logic

## Frozen / complete (docs)
- 00_project_vision.md, 00b_project_roadmap.md, 01_problem_statement.md (frozen v6 + Amendments 1-2)
- 02_literature_review.md, 02b_competitive_analysis.md — COMPLETE
- 03_feasibility_study.md, 04_requirements.md, 05_system_analysis.md, 06_architecture.md,
  07_detailed_design.md, 08_technology_selection.md, 08b_ui_ux_design.md — all Draft v1+

## Repo status
- Pushed to GitHub: https://github.com/michaellusias/acapellastudio (via SSH)
- Working copy: ~/Documents/acapella-daw on Michael's machine

## Operating rules in effect
- One step at a time for any command-line/setup instructions
- No fabricated papers, datasets, benchmarks, or measurements — ever; every claim sourced,
  actually run, or flagged as unverified/not-yet-measured/open
- Real academic/primary sources required for technical claims
- Real-time audio callback isolation rules apply from Phase 1 onward
- Git commit convention: feat/fix/test/docs/refactor, descriptive messages
- Every significant requirement traceable: Requirement → Design → Component → Implementation → Test → Result
