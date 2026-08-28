# AcapellaStudio — Project Status

**Current phase:** Phase 3 — Feasibility Study (IN PROGRESS)
**Next phase:** Controlled reference-pitch test for §2 accuracy, or continue toward §3 (pitch shifting) once §2 is more settled

## Frozen / complete
- 00_project_vision.md — long-term vision, Phase 1-5 roadmap, Harmony Wizard concept
- 00b_project_roadmap.md — master SDLC execution plan (50 phases)
- 01_problem_statement.md — frozen v6, MVP definition, research questions, risk register (R-001–R-014)
- 02_literature_review.md — COMPLETE — all 8 roadmap research areas
- 02b_competitive_analysis.md — COMPLETE — DAWs, pitch tools, harmony tools, AI vocal tools

## 03_feasibility_study.md — real, ongoing progress on Michael's actual hardware

**§1 Audio I/O — substantial real progress:**
- Toolchain working: cpal + pipewire + realtime features, all system deps resolved on real machine
- Native PipeWire host confirmed active (not ALSA fallback)
- 128-sample buffer confirmed genuinely honored by the backend (2.667ms/buffer)
- Real round-trip acoustic loopback latency: ~15.88ms (mean of 2 runs) — exceeds NFR-RT-002's ≤10ms target, but figure includes unseparated speaker/mic transducer response, not pure software latency — status genuinely open, not claimed as pass or fail
- Real 30s idle-passthrough endurance test: ~2% CPU, ~9.3MB RAM, 0 large timing gaps/11237 callbacks — audio I/O layer itself is stable and lightweight
- Real 20s endurance test WITH actual YIN DSP load running every callback: timing stability held (0 large gaps/7469 callbacks, stats nearly identical to idle test) — real evidence the DSP workload doesn't destabilize the real-time thread

**§2 Pitch Detection — strong validated result, one new concern:**
- Real synthetic-signal test: 2.06 cents mean error (clean), 2.28 cents (mild noise) — A5 anomaly (-7.15 cents) flagged, unexplained
- Real controlled reference-tone test (440Hz through speaker, detected via real mic, full acoustic chain): **99.4% detection rate, 0.99 cents mean absolute error, 100% within 50 cents** — genuinely strong, validated accuracy result
- This resolved the earlier open question (0.4% detection on freeform humming was correct silence-handling, not a flaw)
- **New honest concern:** ~50% CPU for naive YIN alone — real optimization target flagged for before this scales into a full production pipeline
- Not yet tested: real melodic singing (moving pitch, vibrato) — only a clean sustained tone validated so far

**§6 ML Dataset Feasibility — COMPLETE:**
- Real datasets found: JaCappella (35 songs, 6 aligned vocal stems), Dagstuhl ChoirSet (peer-reviewed, SATB), Vocal92 (146.73hrs solo a cappella audio), JSB Chorales/Bach Choral Harmony (symbolic only)
- Honest conclusion: R-008 partially resolved — real datasets exist but none match our target genre/scale; licensing unconfirmed for all

**§3, §4, §5 (pitch shifting, key detection, harmony) — not yet started**, depend on §2 being more settled first.

## Prototypes (on Michael's machine, synced into this repo)
- `prototypes/yin-prototype/` — original synthetic-signal-only YIN test
- `prototypes/audio-io-prototype/` — evolved through device enumeration → live callback timing → buffer size testing → round-trip loopback latency → idle endurance test → real-time YIN-on-live-voice combined test (current state)

## Git
- Repository initialized. Multiple real commits with descriptive feat:/docs: messages tracking each real milestone.

## Key findings (cumulative, most important first)
- **Real, validated pitch-detection accuracy is excellent**: 0.99 cents mean absolute error through the full real acoustic chain — the strongest positive result so far.
- **Real-time DSP load does not destabilize the audio thread** at 128-sample buffer on this hardware — confirmed under actual YIN processing, not just idle passthrough.
- **⚠️ New: naive YIN implementation costs ~50% CPU alone** — real optimization needed before adding more DSP to the same real-time budget.
- **⚠️ R-014 (IP risk):** US Patent 8,168,877/8,618,402, pitch-shift-based harmony generation. Antares (Auto-Tune/Harmony Engine maker) is a strong candidate to be connected to it — unconfirmed.
- **⚠️ Round-trip latency (~15.88ms) exceeds the ≤10ms target**, but can't yet be cleanly attributed to software vs. transducer response.
- Real user feedback confirms "mechanical-sounding harmony" (R-009/R-012) is a genuine, documented weakness even in the market-leading commercial tool.
- Real a capella multitrack datasets exist but don't match our target genre/scale.

## Operating rules in effect
- One step at a time for any command-line/setup instructions
- No fabricated papers, datasets, benchmarks, or measurements — ever; every claim sourced, actually run, or flagged as not yet tested/unconfirmed
- Real academic/primary sources required for technical claims
- Real-time audio callback isolation rules apply from Phase 1 onward
- Git commit convention: feat/fix/test/docs/refactor, descriptive messages
- Every significant requirement traceable: Requirement → Design → Component → Implementation → Test → Result
