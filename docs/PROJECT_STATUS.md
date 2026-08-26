# AcapellaStudio — Project Status

**Current phase:** Phase 3 — Feasibility Study (IN PROGRESS — §2 partial, §6 complete, §1/§3/§4/§5 pending hardware)
**Next phase:** Hands-on hardware setup on Michael's Kubuntu machine (§1 Audio Feasibility) — one step at a time

## Frozen / complete
- 00_project_vision.md — long-term vision, Phase 1-5 roadmap, Harmony Wizard concept
- 00b_project_roadmap.md — master SDLC execution plan (50 phases)
- 01_problem_statement.md — frozen v6, MVP definition, research questions, risk register (R-001–R-014)
- 02_literature_review.md — COMPLETE — all 8 roadmap research areas (one gap statement corrected post-Feasibility-Study research, see §6.2 note)
- 02b_competitive_analysis.md — COMPLETE — DAWs, pitch tools, harmony tools, AI vocal tools

## In progress — 03_feasibility_study.md
- §2 Pitch Detection: REAL synthetic-signal prototype built and run (Rust, YIN). Results: 2.06 cents mean error (clean), 2.28 cents (mild noise), 0.82ms/buffer processing time (this container's CPU only). Unexplained anomaly at A5 (880Hz) flagged, not dismissed.
- §6 ML Dataset Feasibility: **COMPLETE.** Real datasets found:
  - JaCappella (HuggingFace) — 35 songs, 34 min, 6 aligned vocal stems (lead/soprano/alto/tenor/bass/percussion), Japanese children's songs genre
  - Dagstuhl ChoirSet — 55 min, SATB, Western classical choir, peer-reviewed (TISMIR)
  - Vocal92 — 146.73 hours real solo a cappella singing (not harmony-aligned, but useful for pitch-detection testing against real voices)
  - JSB Chorales / Bach Choral Harmony — symbolic/MIDI only, no audio, licensing unconfirmed
  - **Honest conclusion:** R-008 (dataset scarcity) partially resolved — real aligned audio stems exist, but none match our target genre/scale (contemporary pop a capella). Licensing not confirmed for any — flagged, not assumed.
- §1, §3, §4, §5: NOT YET TESTED — require Michael's real hardware, microphone, recorded singing voice.

## Prototypes
- `prototypes/yin-prototype/` — working Rust YIN implementation, tested on synthetic sine waves only.

## Git
- Repository initialized. 2 commits so far (docs skeleton + freeze; YIN prototype). Commit convention (feat/fix/test/docs/refactor) in use.

## Key findings so far (cumulative)
- **⚠️ R-014 (IP risk):** US Patent 8,168,877/8,618,402 — pitch-shift-based harmony generation, architecturally close to our approach. Antares (Auto-Tune/Harmony Engine maker) is a strong candidate to be connected to it — unconfirmed, needs direct patent-assignee check.
- **⚠️ Unverified claim:** one source claims YIN is patented; not corroborated elsewhere.
- Real user feedback confirms "mechanical-sounding harmony" (R-009/R-012) is a genuine, documented weakness even in the market-leading commercial tool (Antares Harmony Engine).
- No existing tool combines harmony-from-own-voice + fully offline + a capella-specific workflow — confirmed real gap.
- Real YIN prototype: promising synthetic-signal accuracy/speed, NOT yet validated against real singing voice.
- Real a capella multitrack audio datasets exist (JaCappella, Dagstuhl ChoirSet) but don't match our target genre/scale — a capella-specific ML training data remains a partially-open question, not a solved one.

## Not yet started
- Docs 04 through 18 are placeholders pending their respective phases.
- Feasibility Study §1 (audio I/O), §3 (pitch shifting), §4 (key detection), §5 (harmony) — all require hands-on hardware work.

## Operating rules in effect
- One step at a time for any command-line/setup instructions
- No fabricated papers, datasets, benchmarks, or measurements — ever; every claim sourced, actually run, or flagged as not yet tested/unconfirmed
- Real academic/primary sources required for technical claims
- Real-time audio callback isolation rules apply from Phase 1 onward
- Git commit convention: feat/fix/test/docs/refactor, descriptive messages
- Every significant requirement traceable: Requirement → Design → Component → Implementation → Test → Result
