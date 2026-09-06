# AcapellaStudio — Project Status

**Current phase:** Phase 11 — Audio Engine Prototype (Steps 8-9 implemented: real recording
and playback; Step 10, basic multitracking, is the next open item)
**Next phase:** Continue Phase 11 (Step 10 - multitracking) or Phase 12 (per master roadmap)

## MAJOR RESOLUTION: Mutex-in-real-time-path gap finally fixed

The gap flagged repeatedly since Feasibility Study §1.11 (prototypes used std::sync::Mutex
in the real-time callback, violating NFR-RT-005 despite testing stably) is now genuinely
resolved, not just documented as a target:

- Selected `rtrb` (wait-free SPSC ring buffer, purpose-built for real-time audio) after
  verifying its own documentation and design intent via real research, not assumed
- `AudioEngine::start_recording()` now uses `rtrb::Producer::push()` inside the real-time
  callback - genuinely never blocks, never allocates
- Verified in isolation (sandbox): the exact push/pop/full-buffer-rejection pattern used in
  the real code works correctly (100 samples pushed and drained correctly; third push to a
  2-capacity buffer correctly rejected with Err(Full))
- Updated across 3 documents to reflect this real resolution (not just a design intent):
  Architecture §4.3, Detailed Design §2.1 module maturity table, this status file

## Real, new code this session
- `AudioEngine::start_recording(capacity)` — real, Step 8 of roadmap Phase 11 ("Add
  recording"), returns a `RecordingHandle` with `drain_available()` for pulling captured
  audio on a normal thread
- `AudioEngine::start_playback(samples)` — real, Step 9 ("Add playback"), simple fixed-buffer
  playback (no streaming/seeking yet)
- Added `rtrb = "0.3"` dependency, with real justification recorded in Cargo.toml comments
- New test: `recording_and_playback_do_not_panic_if_devices_exist` — honestly designed to
  not assert success (CI/sandboxed environments may lack real audio devices), same pattern
  as the existing `engine_construction_does_not_panic` test

## NOT yet verified on real hardware (open action item)
- The full `cpal`+`rtrb` integrated recording/playback code has NOT been build-tested on
  Michael's machine yet (sandbox toolchain can't build cpal's current dependency tree, same
  known limitation as before) - only the `rtrb` logic itself was isolated and verified
- A real endurance/CPU test (Feasibility Study §1.11-style) with the new rtrb-based
  recording path has not been run - "the design is now sound" is not the same claim as
  "it's been measured stable," and both should be stated, not conflated

## Real, open gaps still remaining (updated cumulative list)
- Roadmap Phase 11 Step 10: basic multitracking - not yet implemented
- Automatic pitch correction ("nearest scale tone" logic) still has no design or prototype
- Harmony Rule Engine output never wired to the Pitch Shifter as one pipeline
- Formant preservation: zero testing exists anywhere in the project
- GUI framework: zero prototyping exists
- mixer/, project/, export/, clip/, track/, pitch_edit/: stub-only, no real logic
- Sample rate testing: only 48kHz has ever been used in any real test; 44.1kHz (also named
  in Problem Statement NFR-RT-003) has never been tested

## Frozen / complete (docs)
- 00_project_vision.md, 00b_project_roadmap.md, 01_problem_statement.md (frozen v6 + Amendments 1-2)
- 02_literature_review.md, 02b_competitive_analysis.md — COMPLETE
- 03_feasibility_study.md, 04_requirements.md, 05_system_analysis.md, 06_architecture.md,
  07_detailed_design.md, 08_technology_selection.md, 08b_ui_ux_design.md — all Draft v1+,
  06 and 07 updated this session to reflect the real Mutex→rtrb resolution

## Repo status
- Pushed to GitHub: https://github.com/michaellusias/acapellastudio (via SSH)
- Working copy: ~/Documents/acapella-daw/acapellastudio on Michael's machine
- Phase 10 (Development Environment) fully verified on real hardware: cargo build + cargo
  test both succeeded, 8/8 tests passing, before this session's new recording/playback code

## Operating rules in effect
- One step at a time for any command-line/setup instructions
- No fabricated papers, datasets, benchmarks, or measurements — ever; every claim sourced,
  actually run, or flagged as unverified/not-yet-measured/open
- Real academic/primary sources required for technical claims
- Real-time audio callback isolation rules apply from Phase 1 onward
- Git commit convention: feat/fix/test/docs/refactor, descriptive messages
- Every significant requirement traceable: Requirement → Design → Component → Implementation → Test → Result
