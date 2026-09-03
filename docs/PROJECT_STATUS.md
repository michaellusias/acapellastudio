# AcapellaStudio — Project Status

**Current phase:** Phase 7 — Technology Selection (drafted). A significant cross-cutting
architectural amendment (pitch correction moved out of the real-time path) was made during
this phase and propagated across 4 documents.
**Next phase:** Phase 8 — Detailed Software Design (per master roadmap)

## MAJOR ARCHITECTURAL AMENDMENT (Problem Statement §33, Amendment 2)

**Automatic pitch correction (and harmony audio rendering) moved from the real-time
monitoring path to a post-recording processing step.** The singer records with raw,
uncorrected real-time monitoring only (no noticeable delay); correction/harmony are
applied afterward on the captured recording, with no hard deadline.

**Real evidence driving this change:**
- Feasibility Study's real round-trip latency (~15.76-16.00ms, §1.9) already exceeded the
  original <=10ms corrected-monitoring target, and couldn't be cleanly separated from
  transducer response
- PSOLA had never been tested inside a live real-time callback at all - unknown feasibility,
  not just imperfect performance
- Post-recording processing removes both problems and allows higher-quality, look-ahead
  correction algorithms a real-time deadline would have prevented

**Real-time correction is NOT abandoned** - deferred as a legitimate later-phase feature
(live monitoring feedback is a real thing performers want), once post-recording correction
is proven, per the project's established pattern (ship simple/solid first).

**What stayed real-time:** raw monitoring (must have no noticeable delay), and optionally
pitch detection for live visual feedback (still confirmed real-time-safe per §2.11).

**Documents updated to reflect this (all committed):**
- 01_problem_statement.md — §6.3, §6.12.B (struck through), R-009 (struck through), MVP
  feature #5, new §33 (full amendment rationale)
- 04_requirements.md — FR-005 amended, NFR-RT-002 note added (now the only real-time
  latency target, not one of two)
- 05_system_analysis.md — Activity Diagram 4.2 (Pitch Correction) now shows post-recording flow
- 06_architecture.md — real-time/non-real-time split diagram redrawn, Gap §5.3 marked RESOLVED
  (by design, not by testing), §5.1 amended

## Frozen / complete
- 00_project_vision.md, 00b_project_roadmap.md, 01_problem_statement.md (frozen v6 + Amendments 1-2)
- 02_literature_review.md, 02b_competitive_analysis.md — COMPLETE
- 03_feasibility_study.md — all 6 sections, real hardware measurements, real bugs found
- 04_requirements.md — Draft v1 SRS, amended per above
- 05_system_analysis.md — Draft v1, amended per above
- 06_architecture.md — Draft v1, amended per above
- 08_technology_selection.md — Draft v1 — only Language + Audio backend fully "Selected"

## Real, open architecture/technology gaps (explicit, tracked, cumulative)
- Automatic pitch correction ("nearest scale tone" logic) still has no design or prototype
  (gap unchanged by the real-time move - only the deadline pressure relaxed)
- Harmony Rule Engine output never wired to the Pitch Shifter as one pipeline
- ~~PSOLA never tested inside a live real-time audio callback~~ RESOLVED BY DESIGN - no
  longer needs to run there at all
- Real-time-safe concurrency (lock-free ring buffer) not yet selected - prototypes use Mutex
  (still relevant - raw monitoring + pitch detection remain real-time)
- Formant preservation: zero testing exists anywhere in the project
- GUI framework: zero prototyping exists, decision is desk-research-only

## Real defects tracked (cumulative)
- FR-008a — harmony range-fallback failure (confirmed real defect)
- PSOLA octave-up failure (no detectable output pitch) - still real, now tested offline only
- NFR-RT-002 (latency) — genuinely open, now the ONLY real-time latency target (simplified)
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
