//! AcapellaStudio — binary entry point.
//!
//! Phase 10 deliverable: "Compiling Empty Application." This is genuinely
//! that, and no more — it constructs the real, tested AudioEngine and
//! prints its host/config, but does not yet record, play back, or run any
//! DSP. The real logic (pitch detection, pitch shifting, harmony
//! generation, key detection) is fully implemented and unit-tested in
//! their respective library modules (see src/dsp/, src/harmony/) and
//! exercised by `cargo test`, not from this binary yet.

use acapellastudio::audio::AudioEngine;

fn main() {
    println!("AcapellaStudio — development skeleton (Phase 10)");

    match AudioEngine::new() {
        Ok(engine) => {
            println!("Audio engine initialized.");
            println!("Host: {:?}", engine.host_id());
            println!("Sample rate: {} Hz", engine.sample_rate());
        }
        Err(e) => {
            println!("Audio engine could not initialize: {:?}", e);
            println!("(Expected in environments without real audio devices, e.g. CI.)");
        }
    }

    println!("\nReal, tested modules available: dsp::pitch (YinFftDetector), ");
    println!("dsp::pitch_shift (PsolaShifter), harmony::key (KeyDetector), ");
    println!("harmony::rules (DiatonicThirdHarmonizer). Run `cargo test` to see");
    println!("their real regression tests, ported from the Feasibility Study.");
}
