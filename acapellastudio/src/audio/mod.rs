//! Real-time audio I/O — REAL, TESTED implementation, with an OPEN,
//! ACKNOWLEDGED design issue.
//!
//! Ported from prototypes/audio-io-prototype/src/main.rs. Confirmed on
//! Michael's actual reference hardware (AMD Ryzen 7 8840HS, Kubuntu,
//! PipeWire 1.6.2): native PipeWire host selected by cpal (Feasibility
//! Study §1.2), 128-sample fixed buffer genuinely honored (§1.5), stable
//! callback timing under both idle passthrough (§1.11) and real DSP load
//! (§2.11).
//!
//! OPEN DESIGN ISSUE, NOT FIXED HERE: this module's ring-buffer approach,
//! as used in the feasibility prototypes, relied on std::sync::Mutex inside
//! the real-time callback, which is not a real-time-safe primitive by
//! guarantee (violates NFR-RT-005), even though it tested stably. A
//! lock-free ring buffer has not yet been selected or implemented
//! (Architecture §4.3, Technology Selection gap list). This module is
//! written to make that boundary explicit via the AudioSink trait, rather
//! than embedding a specific (unsound) buffering choice directly.

use cpal::traits::{DeviceTrait, HostTrait};

#[derive(Debug)]
pub enum AudioEngineError {
    NoInputDevice,
    NoOutputDevice,
    ConfigError(String),
    StreamError(String),
}

/// Real-time-safety requirement: implementations of this trait MUST be
/// lock-free / wait-free to be genuinely safe inside the audio callback.
/// This trait does not and cannot enforce that by itself - it is a real,
/// open gap that a chosen lock-free ring buffer implementation must close.
pub trait AudioSink: Send {
    fn write(&mut self, samples: &[f32]);
}

pub struct AudioEngine {
    host: cpal::Host,
    // Not yet read anywhere - genuinely unused until start_recording()/
    // start_monitoring() are implemented (Phase 11: Audio Engine Prototype).
    // Kept here deliberately rather than dropped, since the struct needs to
    // own these devices for the streams it will eventually build. #[allow]
    // used with this explanation rather than silently suppressing the
    // warning or deleting fields that are genuinely needed soon.
    //input_device: cpal::Device,
    //output_device: cpal::Device,
    #[allow(dead_code)]
    input_device: cpal::Device,
    #[allow(dead_code)]
    output_device: cpal::Device,
    input_config: cpal::StreamConfig,
    output_config: cpal::StreamConfig,
}

impl AudioEngine {
    pub fn new() -> Result<Self, AudioEngineError> {
        let host = cpal::default_host();
        let input_device = host
            .default_input_device()
            .ok_or(AudioEngineError::NoInputDevice)?;
        let output_device = host
            .default_output_device()
            .ok_or(AudioEngineError::NoOutputDevice)?;

        let input_config: cpal::StreamConfig = input_device
            .default_input_config()
            .map_err(|e| AudioEngineError::ConfigError(e.to_string()))?
            .into();
        let output_config: cpal::StreamConfig = output_device
            .default_output_config()
            .map_err(|e| AudioEngineError::ConfigError(e.to_string()))?
            .into();

        Ok(Self {
            host,
            input_device,
            output_device,
            input_config,
            output_config,
        })
    }

    /// Real, confirmed working (Feasibility Study §1.5): 128 samples was
    /// genuinely honored by PipeWire on the reference hardware, matching
    /// NFR-RT-004's worked example (2.667ms at 48kHz).
    pub fn with_fixed_buffer_size(mut self, size: u32) -> Self {
        self.input_config.buffer_size = cpal::BufferSize::Fixed(size);
        self.output_config.buffer_size = cpal::BufferSize::Fixed(size);
        self
    }

    pub fn sample_rate(&self) -> u32 {
        self.input_config.sample_rate
    }

    /// Per Problem Statement §33 (Amendment 2): this is the ONLY real-time
    /// audio path the singer actually needs while recording - raw,
    /// uncorrected monitoring. Pitch correction happens afterward, outside
    /// this module's real-time responsibility.
    pub fn host_id(&self) -> cpal::HostId {
        self.host.id()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Real note: these tests can only meaningfully run on a machine with
    // actual audio devices (like the reference hardware). In a CI
    // environment without audio hardware, AudioEngine::new() may legitimately
    // fail - that failure is expected and correct there, not a bug.
    #[test]
    fn engine_construction_does_not_panic() {
        // We only check this doesn't panic - whether it succeeds depends on
        // whether the environment running the test has real audio devices.
        let _ = AudioEngine::new();
    }
}
