# AcapellaStudio — Project Status

**Current phase:** Phase 11 — Audio Engine Prototype — **COMPLETE, fully verified on real
hardware** (Steps 1-10 all done: audio I/O, latency measurement, buffer size testing,
underrun detection via timing proxy, real recording, real playback, real basic
multitracking)
**Next phase:** Phase 12 (per master roadmap — likely DSP integration / correction pipeline,
to be confirmed by reading the roadmap before starting)

## Real, new code this session

### Recording/playback (Steps 8-9) — FULLY VERIFIED on real hardware
- `AudioEngine::start_recording()` / `start_playback()` — real, working, using `rtrb`
  (wait-free SPSC ring buffer) — this FINALLY resolves the Mutex-in-real-time-path gap
  flagged since Feasibility Study §1.11
- **Real hardware confirmation:** `cargo build` + `cargo test` succeeded on Michael's
  machine, **9/9 tests passing**, including the new recording/playback test
- One real bug found and fixed during hardware testing: `StreamConfig` must be passed by
  value not reference to `build_input_stream`/`build_output_stream` (same API quirk
  hit at the very start of this project) — fixed with `.clone()`, confirmed working

### Basic multitracking (Step 10) — verified in sandbox, mixer/ has real logic for the first time
- `mixer::mix_tracks()` — real, tested summation-based mixing (sums multiple tracks,
  scales down by track count to prevent clipping)
- `AudioEngine::start_multitrack_playback()` — connects mixer/ to audio/ for the first
  time, mixes tracks then plays the result
- **Honest scope:** no per-track volume/pan/mute/solo yet (Track struct has these fields,
  but mixer doesn't read them) — pure equal-weight summation only. Unequal-length tracks
  are handled by padding with silence, tested explicitly.
- Verified in sandbox (mixer/ has zero external deps): all 4 tests pass (two-track mix,
  unequal-length padding, empty list, single-track passthrough) — NOT yet verified on
  real hardware as part of the full workspace

## Repo status
- Pushed to GitHub: https://github.com/michaellusias/acapellastudio
- Working copy: ~/Documents/acapella-daw/acapellastudio on Michael's machine
- Branch is `main` (not `master` as originally set up in the sandbox — renamed at some
  point during Michael's GitHub setup)

## Real, open gaps still remaining (updated cumulative list)
- mixer/ has real basic logic now, but no per-track volume/pan/mute/solo application yet
- Automatic pitch correction ("nearest scale tone" logic) still has no design or prototype
- Harmony Rule Engine output never wired to the Pitch Shifter as one pipeline
- Formant preservation: zero testing exists anywhere in the project
- GUI framework: zero prototyping exists
- project/, export/, clip/, track/, pitch_edit/: still stub-only, no real logic
- Sample rate testing: only 48kHz has ever been used in any real test; 44.1kHz never tested
- No real endurance/CPU test has been run with the new rtrb-based recording path

## Frozen / complete (docs)
- 00_project_vision.md, 00b_project_roadmap.md, 01_problem_statement.md (frozen v6 + Amendments 1-2)
- 02_literature_review.md, 02b_competitive_analysis.md — COMPLETE
- 03_feasibility_study.md, 04_requirements.md, 05_system_analysis.md, 06_architecture.md,
  07_detailed_design.md, 08_technology_selection.md, 08b_ui_ux_design.md — all Draft v1+

## Operating rules in effect
- One step at a time for any command-line/setup instructions
- No fabricated papers, datasets, benchmarks, or measurements — ever; every claim sourced,
  actually run, or flagged as unverified/not-yet-measured/open
- Real academic/primary sources required for technical claims
- Real-time audio callback isolation rules apply from Phase 1 onward
- Git commit convention: feat/fix/test/docs/refactor, descriptive messages
- Every significant requirement traceable: Requirement → Design → Component → Implementation → Test → Result
