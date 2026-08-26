# AcapellaStudio — Project Status

**Current phase:** Phase 3 — Feasibility Study — IN PROGRESS (not complete — see honest breakdown below)
**Next decision point:** continue sandbox-only prototyping (key detection, pitch-shifting on synthetic signals) OR move to hands-on hardware setup on your machine for real audio/mic testing

## Frozen / complete
- 00_project_vision.md — long-term vision, Phase 1-5 roadmap, Harmony Wizard concept
- 00b_project_roadmap.md — master SDLC execution plan (50 phases)
- 01_problem_statement.md — frozen v6, MVP definition, research questions, risk register (R-001–R-014)
- 02_literature_review.md — COMPLETE — all 8 roadmap research areas
- 02b_competitive_analysis.md — COMPLETE — DAWs, pitch tools, harmony tools, AI vocal tools

## In progress — Phase 3 Feasibility Study (honest breakdown)
- **§3.1 Audio Feasibility — PENDING.** Requires real hardware/microphone. Not started.
- **§3.2 Pitch Detection — PARTIALLY COMPLETE.** Real YIN implementation built in Rust (`prototypes/yin-prototype/`), tested on synthetic sine waves: ~2 cents mean error (clean), <2% CPU/buffer-duration cost. **Found a genuine unexplained anomaly** — A5 (880Hz) shows ~7x higher error than neighboring notes — flagged as an open item, not resolved. Real singing-voice testing still needed (requires mic).
- **§3.3 Pitch-Shifting — PENDING.** Not yet prototyped. Could be tested on synthetic signals without a mic — legitimate next sandbox task.
- **§3.4 Key Detection — PENDING.** Not yet prototyped. Could be tested on synthetic note sequences without a mic — legitimate next sandbox task.
- **§3.5 Harmony — PENDING.** Blocked on §3.4 producing a usable note-sequence representation first.
- **§3.6 ML Feasibility — PARTIALLY COMPLETE.** JSB Chorales dataset verified in depth: 382 Bach chorales, public domain, ~215KB, symbolic/MIDI only (not audio). A capella-specific dataset and audio-paired dataset search still needed.

## Key findings this phase
- Real YIN prototype (Rust, from-scratch implementation) works correctly on synthetic signals with good accuracy and low CPU cost — but this is NOT validation on real singing voice, which is a materially harder test.
- The A5 frequency anomaly needs investigation before treating YIN as validated even on synthetic signals.
- JSB Chorales confirmed as a real, free, public-domain symbolic dataset — but audio-domain and a capella-specific data remain unconfirmed.

## Not yet started
- Docs 04 through 18 are placeholders pending their respective phases.
- Real hardware/microphone testing (needs your machine, one step at a time).

## Operating rules in effect
- One step at a time for any command-line/setup instructions
- No fabricated papers, datasets, benchmarks, or measurements — ever; every claim sourced, tested-and-shown, or flagged as unverified/pending
- Real academic/primary sources required for technical claims
- Real-time audio callback isolation rules apply from Phase 1 onward
- Git commit convention: feat/fix/test/docs/refactor, descriptive messages
- Every significant requirement traceable: Requirement → Design → Component → Implementation → Test → Result
