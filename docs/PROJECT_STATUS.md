# AcapellaStudio — Project Status

**Current phase:** Phase 3 — Feasibility Study (IN PROGRESS — partial, honestly incomplete)
**Next phase:** Continue Phase 3 — either hands-on hardware setup (§1 Audio Feasibility) on Michael's actual machine, or ML dataset desk research (§6) — both can proceed independently

## Frozen / complete
- 00_project_vision.md — long-term vision, Phase 1-5 roadmap, Harmony Wizard concept
- 00b_project_roadmap.md — master SDLC execution plan (50 phases)
- 01_problem_statement.md — frozen v6, MVP definition, research questions, risk register (R-001–R-014)
- 02_literature_review.md — COMPLETE — all 8 roadmap research areas
- 02b_competitive_analysis.md — COMPLETE — DAWs, pitch tools, harmony tools, AI vocal tools

## In progress
- 03_feasibility_study.md — **honestly partial**:
  - §2 Pitch Detection: REAL synthetic-signal prototype built and run (Rust, YIN algorithm). Real results: 2.06 cents mean error (clean), 2.28 cents (mild noise), 0.82ms processing time per 2048-sample buffer on this container's CPU. Unexplained accuracy anomaly at A5 (880Hz, -7.15 cents) flagged for investigation, not dismissed.
  - §1, §3, §4, §5: NOT YET TESTED — require Michael's real hardware, microphone, and recorded singing voice. Explicitly marked as not started rather than filled with placeholder numbers.
  - §6 ML dataset feasibility: queued, not yet started, doable as desk research independent of hardware.

## Prototypes
- `prototypes/yin-prototype/` — working Rust YIN implementation, tested on synthetic sine waves only. Real singing-voice testing is next, once microphone input exists.

## Git
- Repository initialized at project root. First commit made covering all documentation through Competitive Analysis. Commit convention (feat/fix/test/docs/refactor) in use.

## Key findings so far (cumulative)
- **⚠️ Unresolved IP risk (R-014):** US Patent 8,168,877/8,618,402 covers a pitch-shift-based harmony generation architecture close to our planned approach; Antares (maker of Auto-Tune + Harmony Engine) is a strong candidate to be connected to it — not confirmed, needs direct patent-assignee verification.
- **⚠️ Unverified claim:** one source claims YIN is patented; not corroborated elsewhere.
- Real user feedback confirms "mechanical-sounding harmony" (R-009/R-012) is a genuine, already-documented weakness even in the market-leading commercial harmony tool (Antares Harmony Engine).
- No existing tool combines harmony-from-own-voice + fully offline + a capella-specific workflow — confirmed real gap.
- **NEW:** Real YIN prototype shows promising accuracy/speed on synthetic tones, but this must not be mistaken for validation against real singing voice, which has not yet been tested.

## Not yet started
- Docs 04 through 18 are placeholders pending their respective phases.
- Phase 3 Feasibility Study sections requiring hardware/microphone/singing-voice recordings.

## Operating rules in effect
- One step at a time for any command-line/setup instructions
- No fabricated papers, datasets, benchmarks, or measurements — ever; every claim sourced, actually run, or flagged as not yet tested
- Real academic/primary sources required for technical claims
- Real-time audio callback isolation rules apply from Phase 1 onward
- Git commit convention: feat/fix/test/docs/refactor, descriptive messages
- Every significant requirement traceable: Requirement → Design → Component → Implementation → Test → Result
