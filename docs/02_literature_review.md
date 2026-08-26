# AcapellaStudio — Literature Review

**Document:** 02_literature_review.md
**Status:** Phase 1 complete — all 8 research areas from the master roadmap covered
**Scope:** Pitch detection, pitch shifting/correction, real-time audio latency, rule-based harmony generation, key/scale detection, voice separation/vocal synthesis, ML harmony generation, subharmonic/bass generation.

> **Standing rule (from the operating charter):** every claim below is either sourced (with a link) or explicitly marked as unverified/needs-follow-up. Nothing here is invented.

---

# 1. Pitch Detection

## 1.1 YIN

**Source:** de Cheveigné, A., & Kawahara, H. (2002). "YIN, a fundamental frequency estimator for speech and music." *Journal of the Acoustical Society of America*, 111(4), 1917–1930. https://doi.org/10.1121/1.1458024

- Time-domain method based on autocorrelation, with a Cumulative Mean Normalized Difference Function (CMNDF) added to suppress the "subharmonic errors" that plague naive autocorrelation (picking half or double the true frequency).
- The original paper reports error rates roughly 3x lower than the best competing methods it was benchmarked against, evaluated on a speech + laryngograph database — **note this is a speech benchmark, not a singing-voice benchmark**, which matters for our use case and is a gap worth flagging.
- No upper limit on the search frequency range, so it's usable for high-pitched voices.
- Described in the paper itself as "relatively simple" to implement "efficiently and with low latency" — good fit for a real-time callback.

**⚠️ Open item — IP status unverified.** One source (a personal/hobbyist implementation page, http://mroy.chez-alice.fr/yin/index.html) claims "a patent exists on the algorithm and thus you cannot use it in a commercial software without authorization from the patent holder(s)." No other source found in this pass (the original JASA paper, multiple GitHub implementations, academic citations) mentions or corroborates a patent. **This must be independently verified before final commitment to YIN** — check IRCAM's technology-licensing pages directly and/or a patent database search, rather than relying on either the single claim or its absence in other sources. Filed as a Feasibility Study action item, not resolved here.

## 1.2 pYIN (Probabilistic YIN)

**Source:** Mauch, M., & Dixon, S. — pYIN plugin documentation, Queen Mary University of London, Centre for Digital Music. https://code.soundsoftware.ac.uk/projects/pyin

- Modifies YIN to output multiple pitch-candidate values with associated probabilities per frame instead of one hard estimate, then uses a Hidden Markov Model (Viterbi-decoded) to find a smooth path through candidates over time.
- Per a comparative paper found in this pass (arXiv:2206.14357, "Comparing Conventional Pitch Detection Algorithms with a Neural Network Approach"), pYIN is reported to outperform plain YIN with fewer pitch-doubling errors and better voicing detection.
- Relevant to us: better robustness could reduce the correction-artifact risk (R-004 in the frozen Problem Statement) at the cost of added complexity (HMM decoding) versus plain YIN.

## 1.3 CREPE (neural, for contrast — MVP will not use this, noted for completeness)

Referenced in the same comparative paper (arXiv:2206.14357) alongside pYIN and YAAPT as a neural-network pitch-detection baseline. Not investigated in depth this pass since it's outside MVP scope (adds a model-inference dependency to a path that should stay lightweight and deterministic for the MVP) — worth a closer look only if YIN/pYIN prove insufficient in Feasibility Study testing.

## 1.4 SwiftF0 (found in Pass 2, noted here for completeness)

**Source:** Nieradzik, L. (2025). "SwiftF0: Fast and Accurate Monophonic Pitch Detection." https://arxiv.org/pdf/2508.18440

- A lightweight neural model (95,842 parameters — small) specifically targeting real-time, resource-constrained monophonic pitch estimation — directly relevant to our use case (single vocal line, real-time, needs to run on a student's laptop).
- Reported by the authors: 91.80% harmonic-mean accuracy at 10dB SNR, outperforming CREPE by over 12 percentage points at that noise level, and running roughly 42x faster than CREPE on CPU.
- **These are the authors' own self-reported benchmark numbers from their paper — not independently verified by us.** Worth including as a candidate for the Feasibility Study's own comparative testing (alongside YIN/pYIN) precisely because it's small enough to be practical, not because its claimed numbers should be taken as settled.
- Since it's neural, it's outside our "keep the MVP deterministic and simple" preference (§1.3 above) but small enough that it's not obviously in the same category as CREPE — worth a real comparison rather than dismissing it outright.

---

# 2. Pitch Shifting / Correction

## 2.1 PSOLA (Pitch Synchronous Overlap-Add)

**Sources:**
- Lent, K. (1989). "An efficient method for pitch shifting digitally sampled sounds." *Computer Music Journal*, 13, 65–71. (cited via USPTO document 8168877, "Musical harmony generation from polyphonic audio signals")
- Schnell, N. et al., IRCAM — "Synthesizing a choir in real-time using Pitch Synchronous Overlap Add (PSOLA)." http://articles.ircam.fr/textes/Schnell00a/index.pdf

- Time-domain technique: preserves the spectral envelope (formants) during pitch shifting because it manipulates timing/windowing of pitch-synchronous grains rather than resampling the spectrum wholesale.
- Low computational cost — favorable for a real-time budget.
- **Constraint noted directly in the IRCAM source:** PSOLA requires the signal to be reasonably harmonic and well-suited to windowed decomposition into pitch-synchronous grains — i.e. it assumes a clean, mostly-periodic monophonic voice signal. Breathy or noisy vocal passages may degrade its pitch-mark detection.
- The IRCAM paper is directly relevant beyond the algorithm itself: it's specifically about using PSOLA to synthesize a *choir* (multiple voices) in real time from source material — closely adjacent to our harmony-generation use case.

## 2.2 Phase Vocoder

**Source:** Lenarczyk, et al. (2017). "Real time pitch shifting with formant structure preservation." Interspeech 2017. https://www.isca-archive.org/interspeech_2017/lenarczyk17_interspeech.pdf

- Frequency-domain method: interpolates amplitude/phase spectra (via STFT) to change duration without altering frequency content, then rescales to shift pitch. Can be implemented in real time via FFT.
- General finding across sources in this pass: phase vocoder methods preserve broad spectral shape well but are prone to "phasiness" and transient smearing at large pitch-shift ratios or rapid modulation — directly relevant to our concern (Problem Statement §2.7 / R-010) that harmony-scale shifts (a third, fifth, octave) are a harder problem than small corrective shifts.

## 2.3 Formant Preservation — the "chipmunk effect" problem

**Source:** "A Detailed Analysis of a Time-Domain Formant-Corrected Pitch-Shifting Algorithm" (ResearchGate, cites TD-PSOLA/SOLA/WSOLA family). https://www.researchgate.net/publication/255966071

- Confirms directly what our Problem Statement already anticipated: naive pitch shifting without a dedicated formant-preservation step produces the "chipmunk effect" — this is a named, well-studied failure mode, not a hypothetical risk.
- The formant-preservation fix generally requires estimating the spectral envelope of both the original and shifted signal separately from the pitch-shift operation itself — i.e. formant preservation is an *additional* processing stage layered on top of whichever pitch-shift algorithm is chosen, not a property that comes for free from picking "the right" algorithm.

## 2.4 ⚠️ IP Risk — Patented Harmony-via-Pitch-Shift System

**Source:** US Patent 8,168,877 / related 8,618,402 — "Musical harmony generation from polyphonic audio signals." Full text via USPTO: https://image-ppubs.uspto.gov/dirsearch-public/print/downloadPdf/8168877

- This patent describes a system with a "Harmony Shift Generator" block that computes pitch-shift amounts and a "Shifter" block that applies PSOLA-based pitch shifting to a monophonic melody signal to generate harmony audio — **this is architecturally very close to what AcapellaStudio's rule-based harmony engine is planned to do** (analyze melody → determine harmony intervals → pitch-shift to produce harmony audio).
- **This is a real risk, not a hypothetical one, and needs dedicated attention — flagged here, not resolved.** I am not a patent attorney and this document does not constitute legal analysis. This should become an explicit action item in the Feasibility Study: identify the patent's actual claims (not just its abstract/description), check current legal status (patents expire; filing/grant dates need checking), and determine whether AcapellaStudio's specific implementation would need to differ in a legally meaningful way, ideally with real legal guidance rather than our own reading of patent text.
- Recommend adding this as a new risk entry in the Problem Statement's risk register (something like R-014 — IP/Patent Risk in Harmony Generation) rather than leaving it only in this literature review.

---

# 3. Real-Time Audio Latency

## 3.1 Practical latency figures by backend (non-academic but concrete, current)

**Sources:** Multiple current (2026) technical guides — linuxdj.com Linux Audio Quality Guide; oneuptime.com PipeWire configuration guide; botmonster.com PipeWire low-latency guide; audiolatencytest.org.

Reported figures (**these are third-party claims from technical blogs, not our own measurements — must be independently verified on our actual reference hardware before being used as requirements, per the "never invent/assume measurements" rule**):
- ALSA direct: claimed under 5ms with optimized buffers
- PipeWire: claimed 10–20ms typical, with sub-10ms achievable at 64–128 sample quantum on suitable hardware
- PulseAudio: claimed 50–100ms (relevant only as a "what to avoid" data point — not a candidate backend for us)
- WASAPI shared mode (Windows): claimed 10–30ms; exclusive mode 2–5ms
- ASIO (Windows): described as bypassing the Windows audio stack for the lowest latency, no specific figure given in these sources

## 3.2 Academic measurement of interface hardware latency

**Source:** "Real-time auralization for performers on virtual stages." https://arxiv.org/pdf/2309.03149

- Directly measured (using the ITA Toolbox) latency of three real audio interfaces at their smallest buffer settings: RME Fireface UC (48 samples → 4ms), Focusrite Scarlett 2i2 (48 samples → 13ms), M-Audio Fast Track Ultra (256 samples → 17ms).
- Useful as a concrete example of **how** latency should be measured and reported (methodology), and as a reminder that buffer size alone doesn't determine latency — different interfaces at similar buffer sizes produced meaningfully different results (4ms vs. 13ms at the same 48-sample buffer). This directly supports NFR-RT-010 in the frozen Problem Statement (reference hardware must be explicitly documented, not assumed generic).

---

# 4. Rule-Based Harmony Generation

## 4.1 Directly relevant prior work — audio-level harmonization system

**Source:** Yogev, N., & Lerch, A. (2008). "A System for Automatic Audio Harmonization." 25th Tonmeistertagung — VDT International Convention. https://musicinformatics.gatech.edu/wp-content_nondefault/uploads/2016/10/Yogev-and-Lerch-2008-A-System-for-Automatic-Audio-Harmonization-Ein-S-1.pdf

**This is the closest prior-art match found in this pass to AcapellaStudio's own MVP harmony engine.** Its pipeline:
1. Segments the melody into phrases
2. Tags melody notes with harmonic functions
3. Establishes a palette of possible chords per note
4. Finds voicings via a Constraint Satisfaction Problem (CSP), with classical voice-leading/counterpoint rules as constraints
5. Adds passing/transition-note embellishments via a secondary stage
6. **Synthesizes actual four-voice audio output using pitch-shifting techniques** — i.e. it doesn't stop at symbolic harmony, it renders to audio, exactly like our MVP needs to.

Directly informs our architecture: this system proves the "symbolic harmony decision → pitch-shift to render" pipeline (Problem Statement §6.8, option 2) is an established, published approach, not a hypothesis unique to us. Worth reading in full before finalizing our harmony engine design.

## 4.2 Neo-Riemannian voice-leading for four-part harmony

**Source:** Greer, T., & Narayanan, S. (2021). "Harmonize This Melody: Automatic Four-Part Harmony Generation Using Neo-Riemannian Voice-Leading." ISMIR 2021. https://sail.usc.edu/publications/html/b2hd-Greer2022.html

- Uses chord choices/progressions as the harmonic representation; given melody note + current chord, outputs a plausible next chord, with a tunable "conservativeness" parameter.
- Explicitly frames harmonization as non-deterministic — "harmonic accompaniment is never strictly wrong or right" — which is a useful framing for our own MVP: the rule-based engine should aim for *musically defensible*, not *the one correct answer*.

## 4.3 Broader taxonomy / context

**Source:** "A Functional Taxonomy of Music Generation Systems." https://arxiv.org/pdf/1812.04186

- Frames automatic harmony generation as a field split roughly between chorale-style harmonization (explicit voice-leading rule adherence — closer to our diatonic-triad MVP target) and popular-music chord-progression generation (pattern-similarity driven — closer to a possible future ML approach). Useful for correctly positioning our MVP within the existing research landscape when we write the Feasibility Study and SRS.

## 4.4 Hybrid rule+search approaches (context for future phases, not MVP)

**Source:** "Designing Maintainable Hybrid Generative Systems: A Quantum-Inspired Approach to Automated Music Harmony Generation." https://arxiv.org/pdf/2607.06296

- Combines a generative candidate-exploration module with a rule-based optimization/scoring layer (voice-leading smoothness, functional harmonic consistency, cadential preference). Not needed for MVP (diatonic triads only) but a legitimate direction for Phase 2 rule-based expansion before jumping straight to ML.

---

# 4A. Key and Scale Detection *(Pass 2)*

## 4A.1 Krumhansl-Schmuckler Algorithm — the standard approach

**Sources:**
- Krumhansl, C. L. (1990). *Cognitive Foundations of Musical Pitch*. Oxford University Press. (Foundational key-profile work, cited across all sources found in this pass.)
- "Understanding the Algorithm Behind Audio Key Detection." https://arxiv.org/pdf/2505.17259
- GitHub — Corentin-Lcs, "music-key-finder: Krumhansl-Schmuckler Key-Finding Algorithm." https://github.com/Corentin-Lcs/music-key-finder

**How it works:** Build a chromagram (12-bin pitch-class energy histogram) from the audio, average it into a single normalized 12-element vector, then compute the Pearson correlation between that vector and 24 pre-defined "key profiles" (one per major/minor key, empirically derived from listener perceptual ratings in Krumhansl's original psychology research). The key profile with the highest correlation is the detected key.

**This is the standard, most-cited approach across every source found in this pass** — it is not one candidate among many so much as the default starting point for this problem.

## 4A.2 Reported Accuracy — genuinely mixed results across sources, must not be averaged into one number

Multiple sources in this pass report meaningfully different accuracy figures for the Krumhansl-Schmuckler (KS) algorithm, which is itself an important finding:

- One academic comparison (Nápoles García et al., "Key-Finding Based on a Hidden Markov Model and Key Profiles," https://napulen.github.io/media/justkeydding/napoles19key.pdf) reports plain KS at only 69.0% (their own test set), while KS combined with Temperley's profiles reaches 96.8% on a different test set — a large spread depending on which key-profile weights are used and which dataset is tested against.
- Another academic source (arXiv:2402.10247, comparing against Music21's KS implementation) reports KS at 75% global key-signature accuracy, against their own proposed method's 93%.
- Non-academic/product sources (removevocals.ai, pitchdetector.net) report 80–92% depending on material density, but these are marketing-adjacent claims from key-detection tools, not peer-reviewed evaluations, and are noted here only as directional, not authoritative.

**Conclusion for our purposes:** there is no single "KS accuracy" figure — accuracy is highly dependent on which key-profile weighting is used (plain Krumhansl-Kessler vs. Temperley vs. Aarden-Essen vs. others) and what material it's tested on. **This directly reinforces why our Problem Statement (§2.3/§6.2) requires user confirmation/override rather than trusting automatic key detection outright** — the literature itself shows this is not a solved, uniformly-accurate problem.

## 4A.3 The Monophonic-Specific Problem — directly relevant to us

**Source:** removevocals.ai key-finder FAQ (non-academic, but directly on-point): "Chromagram analysis works best on full arrangements with chords and bass. On solo vocals or a single instrument the pitch profile is sparser so the confidence will be lower."

This is exactly our situation — AcapellaStudio's key detection runs on a single sung melody line, not a full mix with chords and bass. A sparser pitch-class distribution gives the correlation step in §4A.1 less information to work with, meaning **we should expect meaningfully lower confidence/accuracy than the headline figures reported for full-mix material**, not the same accuracy.

**Source:** "A probability model for key analysis in music." https://www.sciencedirect.com/science/article/abs/pii/S0950705114001993

- References Izmirli's work building key-detection templates specifically from *monophonic note recordings*, and a probabilistic (rather than pure-correlation) approach to key analysis — a genuine alternative direction worth investigating further if plain KS underperforms on our sparse monophonic input during Feasibility Study testing.

## 4A.4 Practical Failure Modes Noted Across Sources

- **Modal ambiguity** — parallel major/minor and modal scales (Dorian, Phrygian, etc.) are a known weak point; cryo-mix.com's guide (non-academic, practical) explicitly calls out that "accuracy drops for songs that modulate, use modal scales, or have sparse harmonic content."
- **Relative major/minor confusion** — a very commonly cited failure mode across sources: C major and A minor share the same pitch classes, so pure chroma-correlation methods can confuse them without additional cues (e.g. which note the melody resolves to).
- **Modulation within a piece** — KS in its basic form assumes one global key; several sources note it produces a single answer even for pieces that change key, which doesn't fit anyway since our input is a single vocal take, not a full multi-section song, but is worth keeping in mind if percussion/looping later introduces longer-form material.

---

# 4B. Voice Separation / Vocal Synthesis / Voice Conversion *(Pass 3)*

## 4B.1 Content/Timbre Disentanglement — directly relevant to R-012

**Sources:**
- Sha, B., Li, X., Wu, Z., Shan, Y., & Meng, H. (2023). "Neural Concatenative Singing Voice Conversion: Rethinking Concatenation-Based Approach for One-Shot Singing Voice Conversion." https://arxiv.org/abs/2312.04919
- "Building Controllable Virtual Singer by Unsupervised Learning from Voice Recordings." https://arxiv.org/pdf/2305.05401

- The dominant modern paradigm in singing voice conversion (SVC) is to **disentangle content (what's being sung) from timbre (who's singing it)**, then recombine content from one source with timbre from another. NeuCoSVC specifically addresses "timbre leakage" (where the converted voice still sounds partly like the original singer) using self-supervised learned features plus a preserved harmonic/pitch signal.
- **This is directly relevant to R-012** (generated harmony sounding like a pitch-shifted copy of the original voice, not a distinct voice): content/timbre disentanglement is exactly the kind of technique that could let a generated harmony voice sound genuinely different in timbre, not just shifted in pitch — a real, published direction for addressing that risk, even though it's a Phase 3+/ML-era technique, well beyond MVP.
- **Caution:** these systems are heavyweight (self-supervised feature extractors, neural vocoders) and trained on large speech/singing corpora — not something to consider for the rule-based MVP. Filed here as a legitimate future direction, not an MVP candidate.

## 4B.2 DDSP — lighter-weight alternative worth noting

**Source:** "Vocal Timbre Effects with Differentiable Digital Signal Processing." https://arxiv.org/pdf/2306.10886

- DDSP (Differentiable Digital Signal Processing) reframes neural audio synthesis as learning the *parameters* of classical synthesizer/filter components rather than generating raw waveforms directly — generally lighter-weight than full neural vocoders.
- Notably, this specific paper reports adapting DDSP to vocals *with intelligible lyrics* as a real-time implementation — a meaningfully lighter-weight candidate than full SVC systems, worth a closer look if timbre-differentiated harmony voices become a real Phase 3+ priority.

---

# 4C. ML Harmony Generation *(Pass 3)*

## 4C.1 DeepBach — the canonical neural chorale harmonizer, with a real available dataset

**Source:** Hadjeres, G., Pachet, F., & Nielsen, F. (2017). "DeepBach: a steerable model for Bach chorales generation." *ICML 2017*. arXiv:1612.01010

- Trained on the **JSB Chorales dataset** (J.S. Bach four-part chorales) — a real, specific, commonly-cited, publicly available symbolic (score-based) dataset, confirmed via multiple independent citations in this pass (Music Transformer, "Bach or Mock?", Symbolic Music Data v1.0). **This is a genuinely useful, concrete answer to part of R-008** (ML dataset scarcity) — at least for symbolic four-part chorale-style harmonization, real training data exists and is well-established in the field, even though it's Bach-chorale style, not a capella pop/vocal style specifically.
- "Steerable" — allows constrained generation (e.g. fixing some voices/notes and letting the model fill in the rest), a useful property if we ever want a hybrid where the user's manual edits constrain what the ML system generates around them.

## 4C.2 Directly matching prior art — vocal-specific neurosymbolic harmonizer

**Source:** "AI Harmonizer: Expanding Vocal Expression with a Generative Neurosymbolic Music AI System." https://www.researchgate.net/publication/392941833

**This is the closest ML-harmony prior art found to date, specifically for our use case.** Key points:
- Explicitly targets solo vocalists, generating "musically coherent four-part harmonies without requiring prior harmonic input from the user" — i.e. no manual key/chord entry needed, closely matching our own Harmony Wizard vision (though our MVP requires user confirmation of detected key per §2.3, this system autonomously infers it).
- Combines generative AI for pitch detection and voice modeling with custom-trained **symbolic** music models — i.e. it follows the "symbolic decision, then render" pipeline direction we already favor (Problem Statement §6.8), not end-to-end raw audio generation.
- Trained on a dataset of 18,005 melody/chord pairs, and benchmarked against a named prior baseline system (MTHarmonizer) on objective chord/melody harmonicity metrics plus a 100+ participant subjective listening test — a real, describable evaluation methodology worth modeling our own future ML evaluation on.
- **This should be a required read before any Phase 3+ ML harmony design work begins.**

## 4C.3 Rule-based comparison directly matching our MVP scope

**Source:** "Automatic Melody Harmonization with Triad Chords: A Comparative Study." https://arxiv.org/pdf/2001.02360

- Specifically studies **triad-chord harmonization** — this is precisely our MVP's stated harmonic vocabulary (Problem Statement: "diatonic triadic harmony" only). A comparative study at exactly our target complexity level, rather than the more advanced extended/jazz harmony most other sources address.
- Contains an extensive bibliography of prior rule-based and statistical harmonization approaches (Ebcioğlu 1988's expert system; Allan & Williams 2005's probabilistic chorale harmonization) — useful additional reading if the Yogev & Lerch (2008) approach (§4.1) needs supplementing during design.

## 4C.4 Broader field survey

**Source:** "A Survey on Deep Learning for Symbolic Music Generation: Representations, Algorithms, Evaluations, and Challenges." *ACM Computing Surveys*. https://dl.acm.org/doi/abs/10.1145/3597493

- Confirms harmonization/accompaniment generation is an established sub-field within symbolic music generation research, with representation choices (piano-roll/grid vs. event-sequence) being a first-order design decision — relevant context for whenever we design our own harmony representation format (Problem Statement §6.8).

---

# 4D. Subharmonic / Bass Vocal Generation *(Pass 3)*

## 4D.1 Commercial DSP technique — zero-crossing counting

**Source:** Bender, F. "Subharmonic Synthesis for Bass Enhancement." https://medium.com/@franz.bender/subharmonic-synthesis-for-bass-enhancement-812b58eca930

- Describes a concrete, implementable technique: count zero-crossings of the input signal to derive a square wave at half the input frequency, volume-match via envelope calculation, then lowpass filter to remove harsh harmonic content from the squared-off waveform, and mix back with the original.
- Explicitly suggests combining with harmonic-percussive source separation (HPS) to control whether subharmonics are generated from sustained (harmonic) content, percussive content, or both — directly relevant if we ever want subharmonics only on sustained vocal notes, not on percussive vocal-percussion tracks.
- **This is a genuinely concrete starting algorithm candidate for a future Phase 2+ subharmonic feature** — but note it's written and demonstrated for general mix bass-enhancement, not vocals specifically.

## 4D.2 ⚠️ Direct evidence that vocals are a special, harder case

**Source:** "Sub Harmonic Synthesizer" — Diamond Cut User Forum (product documentation for a real commercial tool). https://www.diamondcut.com/vforum/forum/general-discussion/general-audio/55968-sub-harmonic-synthesizer

- The product documentation itself includes a **"Male Vocal Discriminator"** checkbox specifically to *reduce* the subharmonic synthesizer's tendency to act on bass male vocals, and warns: **"Sub-harmonic distortion can be introduced by this effect on male vocals, especially when the frequency control is set to values greater than 100 Hz."**
- **This is concrete, sourced evidence — not just our own hypothesis — that generic subharmonic-synthesis techniques behave worse specifically on vocals than on other program material**, directly supporting the caution already written into the Project Vision document (§19) and reinforcing that subharmonic vocal generation deserves its own dedicated feasibility check before being attempted, rather than assuming the general-purpose technique in §4D.1 will simply work.

## 4D.3 Real vocal technique — kargyraa / "Throat Bass"

**Source:** "Subharmonics (Singing method)." NamuWiki (non-academic, but factually describes an established vocal technique with citations to the underlying acoustic mechanism). https://en.namu.wiki/w/서브하모닉스(창법)

- Describes kargyraa (a Tuvan throat-singing technique) and "Throat Bass" (used in beatboxing) as **techniques a singer performs** — asymmetric vocal-fold vibration, or mixing true voice with vocal fry — that produce genuine subharmonics (often an octave down) directly from the human voice, not from DSP processing after the fact.
- **This reinforces the point already made in our Project Vision document (§19):** convincing vocal subharmonics may be more achievable by processing/encouraging recordings of a singer actually using vocal-fry/kargyraa-adjacent technique than by trying to synthesize the effect purely in DSP from a normally-sung take. Worth keeping both directions (technique-assisted recording vs. pure DSP synthesis) open as we approach that phase, rather than committing to DSP-only.

## 4D.4 Pathological/clinical subharmonic research (context, not directly usable)

**Source:** "Towards detecting the pathological subharmonic voicing with fully convolutional neural networks." https://arxiv.org/pdf/2501.09159

- Uses a kinematic vocal-fold model to synthesize subharmonic voicing for clinical/pathological voice research (detecting disordered voicing, not for music production). Not directly applicable to our use case, but confirms subharmonic vocal-fold dynamics are physiologically modelable — useful background if we ever pursue a genuinely physically-modeled (rather than purely spectral) approach to vocal subharmonics far down the roadmap.

---

# 5. Literature Matrix

| Paper | Authors | Year | Problem | Method | Advantages | Limitations | Relevance to AcapellaStudio |
|---|---|---|---|---|---|---|---|
| YIN, a fundamental frequency estimator | de Cheveigné & Kawahara | 2002 | Pitch/F0 estimation | Autocorrelation + CMNDF | Low error rate, real-time capable, few parameters | Benchmarked on speech, not singing; IP status unverified (§1.1) | Candidate MVP pitch detector |
| pYIN plugin | Mauch & Dixon | — | Pitch/F0 estimation, robustness | YIN + HMM-smoothed candidates | Fewer octave errors than YIN per comparative study | Added HMM complexity/latency | Fallback if plain YIN underperforms |
| Real time pitch shifting w/ formant preservation | Lenarczyk et al. | 2017 | Pitch shifting | Phase vocoder (FFT-based) | Real-time capable, formant-preserving variant | Phasiness/smearing at large shifts | Candidate for harmony-scale shifting |
| Synthesizing a choir using PSOLA | Schnell et al. (IRCAM) | 2000 | Multi-voice synthesis from one source | TD-PSOLA | Low CPU cost, formant-preserving, choir-specific precedent | Requires clean harmonic input signal | Directly precedent for our harmony-voice rendering |
| A System for Automatic Audio Harmonization | Yogev & Lerch | 2008 | Melody→harmony, symbolic + audio render | Phrase segmentation + harmonic tagging + CSP voicing + pitch-shift render | End-to-end pipeline matching our exact use case | Older work; needs close reading | **Closest known prior art to our MVP harmony engine** |
| Harmonize This Melody | Greer & Narayanan | 2021 | Four-part harmony generation | Neo-Riemannian voice-leading, next-chord prediction | Tunable conservativeness; strong ISMIR venue | Chord choice only, not full audio rendering | Voice-leading framing useful for our rule engine |
| Real-time auralization for performers | arXiv 2309.03149 | 2023 | Measuring real interface latency | Direct hardware measurement (ITA Toolbox) | Concrete measured numbers, real methodology | Small interface sample | Methodology template for our latency measurement |
| Musical harmony generation from polyphonic audio signals | US Patent 8,168,877 / 8,618,402 | — | Pitch-shift-based harmony generation | Harmony Shift Generator + Shifter (PSOLA-based) | N/A (patent, not a technique to adopt) | **Legal risk, not a resource** | **IP risk — needs Feasibility Study follow-up** |
| SwiftF0: Fast and Accurate Monophonic Pitch Detection | Nieradzik | 2025 | Real-time monophonic F0 estimation | Lightweight neural model (~96K params) | Small, fast (42x faster than CREPE per authors), robust to noise per authors' own benchmark | Self-reported numbers, not independently verified by us | Candidate for Feasibility Study comparison alongside YIN/pYIN |
| Understanding the Algorithm Behind Audio Key Detection | (arXiv 2505.17259) | 2025 | Key detection methodology | Krumhansl-Schmuckler chroma correlation | Clear, current methodological writeup | Doesn't address monophonic-specific accuracy | Standard-approach reference for our key-detection module |
| Key-Finding Based on a Hidden Markov Model and Key Profiles | Nápoles García et al. | 2019 | Key detection accuracy comparison | KS + multiple key-profile variants + meta-classifier | Direct accuracy comparison across profile variants (69–96%) | Wide accuracy spread shows profile choice matters a lot | Evidence that "KS accuracy" isn't one number — supports our user-override requirement |
| A probability model for key analysis in music | (ScienceDirect, cites Izmirli) | 2014 | Key detection, incl. monophonic-derived templates | Probabilistic model, monophonic note-recording templates | Directly addresses monophonic material | Older work, needs closer reading | Alternative direction if plain KS underperforms on sparse vocal input |
| Neural Concatenative Singing Voice Conversion | Sha, Li, Wu, Shan, Meng | 2023 | Singing voice conversion, timbre leakage | SSL features + harmonic signal generator + waveform synthesizer | Resolves timbre-leakage issue vs. prior disentanglement methods | Heavyweight (SSL extractor + neural vocoder), needs large training data | Future direction for R-012 (harmony sounding like a copy of the source voice) |
| DeepBach: a steerable model for Bach chorales generation | Hadjeres, Pachet, Nielsen | 2017 | Neural chorale harmonization | Trained on JSB Chorales dataset, steerable/constrained generation | Real, established, public dataset (JSB Chorales); steerability useful for hybrid manual+ML workflows | Bach-chorale style, not a capella pop/vocal style | Confirms real training data exists for at least symbolic four-part harmonization (partial answer to R-008) |
| AI Harmonizer: Expanding Vocal Expression with a Generative Neurosymbolic Music AI System | (ResearchGate) | 2025 | Vocal-specific automatic 4-part harmony | Generative AI + custom symbolic models, no user harmonic input needed | Directly matches our use case; real benchmark vs. MTHarmonizer + 100+ participant listening test | Full architecture/dataset details behind paywall in this pass | **Closest known ML-harmony prior art to our own vision — required reading before Phase 3+ ML design** |
| Automatic Melody Harmonization with Triad Chords: A Comparative Study | (arXiv 2001.02360) | 2020 | Triad-chord harmonization specifically | Comparative study of multiple approaches | Matches our exact MVP harmonic vocabulary (triads) | Symbolic, not audio-rendering | Directly relevant comparison set for our rule-based MVP engine |
| Subharmonic Synthesis for Bass Enhancement | Bender (Medium) | 2024 | Generic bass/subharmonic DSP technique | Zero-crossing counting → half-frequency square wave → lowpass → mix | Concrete, implementable, combinable with harmonic/percussive separation | Demonstrated on general mixes, not vocals specifically | Candidate starting algorithm for future subharmonic feature |
| Sub Harmonic Synthesizer (product docs) | Diamond Cut | — | Commercial subharmonic synthesis, vocal-specific caveats | Frequency-domain synthesis w/ "Male Vocal Discriminator" control | Real product evidence of a known vocal-specific failure mode | Not academic; single product's documentation | **Concrete evidence vocals are a harder case for subharmonic synthesis — supports existing caution in Project Vision §19** |

---

# 6. Identified Research Gaps

1. **Singing-voice-specific pitch-detection benchmarks.** The core YIN paper's error-rate claims are from a speech database, not singing. No singing-specific benchmark for YIN/pYIN was found in this pass — worth a targeted follow-up search, or planning our own small benchmark during Feasibility Study.
2. **No literature found (in this pass) directly comparing correction-scale vs. harmony-scale pitch-shift quality on the same algorithm.** Our Problem Statement (§2.7) treats these as separate problems on hypothesis; the literature found so far discusses large-shift degradation qualitatively but didn't surface a study quantifying the difference at, say, a minor third vs. an octave specifically for vocal material. Worth another targeted search pass.
3. **No dedicated a capella-specific harmonization literature found.** Yogev & Lerch (2008) is general audio harmonization, not a capella-specific. Given AcapellaStudio's specific niche, this gap is expected but worth one more targeted search before Feasibility Study concludes.
4. **No key-detection study found that specifically benchmarks accuracy on solo monophonic vocal input (as opposed to solo monophonic instrument input, or full-mix audio).** The removevocals.ai source (§4A.3) states directional expectation (lower confidence on sparse material) but is not a rigorous study. This is a genuine gap and directly informs the Feasibility Study's own testing plan — we likely need to run our own small accuracy test on real sung monophonic recordings rather than relying on published figures, which were mostly gathered on full-mix or symbolic (MIDI) data.
5. **No study found quantifying subharmonic-synthesis artifact rates specifically on vocal material** (as opposed to the qualitative product-documentation warning found in §4D.2). If subharmonic generation is pursued in a later phase, this would need our own testing, not an assumption either way.
6. **The AI Harmonizer paper (§4C.2) is the closest match to our own long-term vision found in either pass, but full architecture and dataset details were not accessible in this pass** (ResearchGate preview only) — worth a dedicated follow-up to obtain the full paper before any Phase 3+ ML harmony design work.

---

# 7. Candidate Algorithms Carried Forward to Feasibility Study

- **Pitch detection:** YIN (primary candidate, pending IP verification), pYIN (fallback/robustness upgrade)
- **Pitch shifting:** PSOLA (primary candidate for both correction and harmony-scale shifting, pending formant-preservation-quality testing at larger intervals), phase vocoder (alternative/comparison candidate)
- **Harmony generation architecture:** phrase segmentation → harmonic function tagging → constraint-based voicing → pitch-shift render, following the Yogev & Lerch (2008) shape, adapted down to our MVP's single-voice, diatonic-triad scope
- **Key detection:** Krumhansl-Schmuckler chroma-correlation (primary candidate, standard approach), with explicit expectation of lower accuracy on our sparse monophonic input than published full-mix figures — user confirmation/override (already required by Problem Statement §2.3) is not just good UX, it's a necessary compensation for a known accuracy gap in the literature itself
- **Voice conversion/timbre disentanglement (Phase 3+ only):** SSL-feature-based content/timbre separation (e.g. NeuCoSVC-style) as a candidate direction for addressing R-012 (harmony sounding like a copy of the source voice), once we're past the rule-based MVP
- **ML harmony (Phase 3+ only):** DeepBach-style steerable generation on a symbolic representation, informed by the AI Harmonizer paper's vocal-specific approach and evaluation methodology (chord/melody harmonicity metrics + listening test); JSB Chorales confirmed as one real available dataset, though a capella/pop-specific data still needs its own search
- **Subharmonics (Phase 2+ only, flagged as higher-risk):** zero-crossing/half-frequency DSP technique as a starting candidate, with explicit awareness (per real product documentation) that vocals are a documented harder case than general program material

---

# 8. Action Items for Feasibility Study

1. Verify YIN's actual patent status directly (not via secondary claims either way).
2. Investigate US Patent 8,168,877 / 8,618,402 claims and legal status; consider whether real legal consultation is warranted before implementation begins.
3. Measure real latency on our actual reference hardware — do not carry forward the third-party blog figures in §3.1 as requirements.
4. Run a small YIN vs. pYIN accuracy comparison on actual singing-voice recordings (not speech) before committing.
5. One more targeted literature search specifically for a capella / vocal-ensemble harmonization work, and for correction-scale vs. harmony-scale pitch-shift quality studies.
6. Run our own small key-detection accuracy test on real sung monophonic recordings, since no source found in either pass benchmarks this specific case directly.
7. Compare SwiftF0 against YIN/pYIN on our own singing-voice test material before committing to a pitch-detection algorithm.
8. Obtain the full "AI Harmonizer" paper (§4C.2) before any Phase 3+ ML harmony design work — it's the closest known match to our vision and was only accessible via preview in this pass.
9. Search specifically for a capella/pop-vocal-appropriate symbolic harmony datasets, since JSB Chorales (§4C.1) is a real but stylistically mismatched dataset for our genre.

---

# 9. Status

**Phase 1 Literature Review is complete** across all 8 research areas specified in the master roadmap: pitch detection, pitch correction, harmony generation, voice separation/vocal synthesis, MIR/key detection, ML harmony, subharmonics, and real-time audio. Passes 1–3 are folded into this single document. Ready to proceed to Phase 2 (Competitive/Existing-System Analysis) per the roadmap.
