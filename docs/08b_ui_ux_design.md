# AcapellaStudio — UI/UX Design

**Document:** 08b_ui_ux_design.md (roadmap Phase 9 — no exact slot in the charter's file
skeleton, placed adjacent to Technology Selection per this project's established
practice of adding lettered files for roadmap-only content, e.g. 00b, 02b)
**Status:** Draft v1
**Predecessor documents:** 05_system_analysis.md (Use Cases), 07_detailed_design.md (modules), Project Vision §10 (original Harmony Wizard concept)

> **Standing rule for this document:** the roadmap's own Harmony Wizard specification (used as the base for §3 below) describes the full long-term vision — up to 6 voices, "Advanced AI" complexity — which is explicitly **not** the MVP per the frozen Problem Statement. Every step below is tagged MVP or DEFERRED so this document doesn't quietly imply more is built than actually exists.

---

# 1. Core Layout Wireframe

Per the roadmap's specified layout, elaborated:

```text
┌──────────────────────────────────────────────────────────────────┐
│ Menu / Transport                                                  │
│ [Record] [Play] [Stop]              Project: <name>   [Export]    │
├──────────────────────────────────────────────────────────────────┤
│                                                                    │
│  Timeline / Tracks                                                │
│  ┌────────────┬─────────────────────────────────────────────┐    │
│  │ Lead Vocal │ ▓▓▓▓▓▓▓▓▓░░░░▓▓▓▓▓▓▓▓▓▓▓▓░░░░░▓▓▓▓▓▓▓▓▓▓▓▓   │    │
│  ├────────────┼─────────────────────────────────────────────┤    │
│  │ Harmony 1  │ (empty until generated)                     │    │
│  ├────────────┼─────────────────────────────────────────────┤    │
│  │ Vocal Perc │ ▓▓░░▓▓░░▓▓░░▓▓░░                             │    │
│  └────────────┴─────────────────────────────────────────────┘    │
│                                                                    │
├──────────────────────────────────────────────────────────────────┤
│ Inspector / Mixer / Pitch Editor (context-dependent panel)        │
│  [Selected: Lead Vocal]  Vol: ──●──  Pan: ──●──  [Harmonize...]   │
└──────────────────────────────────────────────────────────────────┘
```

**MVP scope note:** only two track rows are guaranteed to exist for the MVP — Lead Vocal and (at most) one Harmony Voice, plus optionally one Vocal Percussion track (Problem Statement MVP feature list). The wireframe shows one harmony row deliberately, not several, to avoid visually implying multi-voice support that isn't built.

**Real basis:** none — no GUI framework has been selected (Technology Selection §4) and no UI code exists. This is a design artifact, not a screenshot of anything real.

---

# 2. UX Flow — Primary Recording/Correction Workflow

Grounded in Amendment 2 (post-recording correction, Problem Statement §33):

```text
1. User opens/creates project
        │
        ▼
2. User selects/creates Lead Vocal track, arms for recording
        │
        ▼
3. User records — hears RAW, UNCORRECTED monitoring only (Amendment 2)
        │
        ▼
4. User stops recording — AudioClip created (UC-001)
        │
        ▼
5. [Post-recording] User applies automatic pitch correction (NOT YET BUILT)
        │            or manually edits pitch (NOT YET BUILT)
        ▼
6. User opens Harmony Wizard on the corrected/edited lead vocal (see §3)
        │
        ▼
7. Harmony Voice generated as a separate track (UC-005, partially real —
   §5.2/§5.3 of the Feasibility Study)
        │
        ▼
8. User mixes tracks (UC-007, NOT YET BUILT) and exports (UC-008, NOT YET BUILT)
```

**Honest status:** steps 1-4 (record raw audio) rest on a real, validated audio engine (Feasibility Study §1). Step 5 (correction) has no UI or "nearest scale tone" logic yet (Detailed Design §2.4, §5.1 gap). Step 6-7 (harmony) has real symbolic generation logic behind it, with a known bug, but no UI and no audio-rendering integration (Architecture §5.2 gap). Step 8 has nothing built. **This flow is a target, not a description of working software.**

---

# 3. Harmony Wizard Specification

Following the roadmap's full 9-step specification, with MVP/DEFERRED tags per step.

## Step 1 — Source

```text
Selected Track:
Lead Vocal
```
**Status: MVP.** Track selection is a basic prerequisite; no special design needed beyond what `track/` (Detailed Design §2.6) already specifies.

## Step 2 — Harmony Type

```text
Simple        <- MVP: this is effectively the only real option (diatonic-third,
Tight             Feasibility Study §5) - the others below have no generation
Wide              logic behind them at all
Double
Choir
Vocal Section
Custom
Advanced AI
```
**Status: DEFERRED, except "Simple."** The Problem Statement's MVP is a single diatonic-triadic voice — there is no real distinction implemented yet even between "Tight" and "Wide," let alone "Choir" or "Advanced AI" (the latter requires ML harmony generation, explicitly Phase 3+, Problem Statement). **For the MVP, this step should not be shown as a meaningful choice at all** — presenting 8 options when only 1 has any real behavior behind it would mislead the user. Recommend: omit this step for MVP, or show it disabled/grayed with a "more styles coming" note.

## Step 3 — Voice Configuration

```text
Soprano
Mezzo
Alto
Tenor
Baritone
Bass
Sub-Bass
```
**Status: PARTIALLY MVP.** The real harmony prototype (Feasibility Study §5.2-§5.3) takes a **vocal range** (min/max MIDI) as a constraint, not a named voice-type preset — so the underlying capability (range-constrained generation) is real, but these specific named presets (mapping "Alto" to a specific MIDI range, etc.) have not been defined anywhere in the project. **Action needed before this step can ship even in a simplified MVP form:** define the actual MIDI ranges each label corresponds to — not done yet, a real open item, not just a UI polish task.

## Step 4 — Number of Voices

```text
1             <- MVP: the only supported value
2
3
4
5
6
Custom
```
**Status: MVP = "1" only, hard-coded, no picker needed.** The Problem Statement is explicit that multi-voice generation is Phase 2+. For the MVP, this entire step should be omitted from the UI — showing a selector with a fixed, forced value would be confusing rather than honest about current scope.

## Step 5 — Harmonic Complexity

```text
Basic Diatonic   <- MVP: this is the ONLY real, implemented option
Extended
Advanced
AI Arrangement
```
**Status: DEFERRED, except "Basic Diatonic."** Same reasoning as Step 2 — no generation logic exists for anything beyond basic diatonic triads (Problem Statement's explicit MVP scope). Recommend omitting this step entirely for MVP, same as Step 2.

## Step 6 — Musical Controls

```text
Key            <- MVP: real, Krumhansl-Schmuckler-based (Feasibility Study §4),
Scale             with user confirmation/override per Problem Statement §2.3 -
                  REQUIRED, not optional, given the real narrow-margin finding
                  (E minor/E major, §4.2)
Chord Mode     <- DEFERRED - no chord-level control exists, harmonizer works
                  purely from scale degree
Voice Range    <- MVP: real, this is exactly what the harmony prototype's
                  vocal-range parameter already does (Feasibility Study §5.2)
Voicing        <- DEFERRED - "close/open voicing" has no implementation
Density        <- DEFERRED - meaningless with a fixed single voice (Step 4)
Independence   <- DEFERRED - no "voice independence" control exists; the
                  current harmonizer has exactly one behavior
```
**Status: Key + Voice Range = MVP (both real). Everything else = DEFERRED.** This is the step with the most genuine substance for the MVP — it's worth designing properly, unlike Steps 2/4/5 which should mostly be hidden.

## Step 7 — Preview

```text
The user can hear the result before accepting.
```
**Status: NOT YET POSSIBLE, even for MVP.** This requires the harmony decision (real, symbolic, Feasibility Study §5) to be rendered into actual audio via pitch-shifting (Architecture Gap §5.2 — the two have never been wired together). **This is a real blocker for the MVP Harmony Wizard being usable at all**, not a nice-to-have — a wizard that can't preview its own output before the user accepts it is a significantly worse experience, and arguably shouldn't ship without this step working.

## Step 8 — Generate

```text
Harmony 1        <- MVP: exactly one, per the fixed "Number of Voices" above
Harmony 2        <- DEFERRED
Harmony 3        <- DEFERRED
...
```
**Status: MVP = create exactly one "Harmony 1" track.** Real basis: the symbolic generation logic is tested (Feasibility Study §5); track creation itself is not implemented (`track/` module is design-only, Detailed Design §2.6).

## Step 9 — Edit

```text
- muted            <- DEFERRED (mixer/ is a stub, Detailed Design §2.7)
- soloed           <- DEFERRED (same)
- pitch edited     <- DEFERRED (pitch_edit/ has no implementation)
- moved            <- DEFERRED (clip/ has no implementation)
- regenerated      <- MVP-plausible: re-running the harmony generator is cheap
                      once it exists, but nothing currently calls it from a UI
- deleted          <- DEFERRED (track/ deletion exists in design, not implementation)
- mixed independently <- DEFERRED (mixer/ is a stub)
```
**Status: everything here is DEFERRED for now**, since it depends on modules (`mixer/`, `pitch_edit/`, `clip/`, `track/`) that the Detailed Design document (§4) already ranked as the least mature in the whole project.

---

# 4. MVP Harmony Wizard — Simplified Real Flow

Given the step-by-step tagging above, the **actual MVP wizard** should look nothing like the roadmap's full 9-step version. A simplified, honest MVP flow:

```text
1. Select Lead Vocal track
        │
        ▼
2. System detects key (Krumhansl-Schmuckler, real) → shows top candidate(s)
        │
        ▼
3. User confirms or overrides detected key (REQUIRED per Problem Statement §2.3)
        │
        ▼
4. User sets/confirms vocal range for the harmony voice
        │
        ▼
5. [BLOCKING GAP: Preview - not possible until harmony-to-audio rendering exists]
        │
        ▼
6. User accepts → system creates "Harmony 1" track (generation logic real;
   track creation not implemented)
```

**This is a 4-6 step flow, not 9** — Steps 2, 4, 5, and most of 6 from the roadmap's full specification are correctly omitted or hidden for the MVP, per the honest tagging above. The full 9-step wizard remains the right design target for later phases (per Project Vision §10), not something to build prematurely.

---

# 5. Status

Draft v1. Of the roadmap's full Harmony Wizard specification, only a minority of steps (Source, the "Simple"/"Basic Diatonic" implicit defaults, Key, Voice Range, single-voice Generate) have real logic behind them. This document exists to prevent the natural temptation to build all 9 UI steps just because the roadmap lists them — most would be empty chrome around nothing, and the simplified 4-6 step flow in §4 is the actual, honest MVP target.
