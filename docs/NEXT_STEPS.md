# AcapellaStudio — Next Steps

**Last updated:** after real controlled reference-pitch validation (Feasibility Study §2.7)
**Purpose:** a short, action-oriented pointer to what's queued next. For the full cumulative history and findings, see `PROJECT_STATUS.md`. For the detailed real results behind each item, see `03_feasibility_study.md`.

---

## Immediate next actions (in rough priority order)

1. **Investigate optimizing the YIN implementation.** The naive prototype costs ~50% of a core for pitch detection alone (§2.7) — a real, evidence-based concern, not hypothetical. Options to investigate: incremental/windowed autocorrelation updates instead of full recomputation every callback, or switching to an FFT-based autocorrelation approach. This is now the top priority since it constrains how much CPU budget is left for everything else (pitch shifting, eventual harmony rendering) sharing the same real-time thread.

2. **Compare YIN against pYIN and SwiftF0** (Literature Review §1.2/§1.4) on real hardware, using the same reference-tone methodology from §2.7 plus real singing-voice tests. SwiftF0 in particular is worth prioritizing given its claimed speed advantage — directly relevant to the CPU concern in item 1.

3. **Test against real melodic singing** — moving pitch, vibrato, natural breathiness — not just a clean sustained reference tone. §2.7 validated detection accuracy on one steady note; real vocal performance is a distinct, not-yet-completed test.

4. **Isolate transducer response from software-path latency** in the ~15.88ms round-trip figure (§1.9) — either via an electrical loopback cable if one becomes available, or by researching typical transducer response times to estimate a plausible split.

5. **Verify the R-014 patent connection** — whether Antares is actually the assignee of US Patent 8,168,877/8,618,402 (Competitive Analysis §3.1) — a direct patent-assignee record check, not further architectural inference.

6. **Verify YIN's actual IP/patent status directly** (Literature Review §1.1) — one uncorroborated source claims a patent exists; needs a direct check (IRCAM licensing pages or a patent database), not reliance on either the claim or its absence elsewhere.

## Not yet started (deferred until the above settles)

- §3 Pitch-shifting feasibility (needs real audio + a PSOLA/phase-vocoder prototype)
- §4 Key-detection feasibility (needs real melody recordings — a genuinely novel test since no published benchmark exists for sparse monophonic vocal input)
- §5 Harmony-generation feasibility (depends on §2/§3/§4 being further along)
- ML dataset licensing verification (JaCappella, JSB Chorales — Feasibility Study §6.4)

## Documents still at placeholder status

`04_requirements.md` through `18_changelog.md` — not started, waiting on the Feasibility Study to reach a natural stopping point before Requirements Specification begins.

---

*This file is a pointer, not a full record — update it whenever priorities shift, but treat `PROJECT_STATUS.md` and `03_feasibility_study.md` as the sources of truth for what's actually been done.*
