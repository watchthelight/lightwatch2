//! Phase transition sounds

use super::{Envelope, Oscillator, Waveform};
use crate::core::Phase;

/// Phase transition sound
pub struct TransitionSound {
    /// Ascending tone
    tone: Oscillator,
    /// Envelope
    envelope: Envelope,
    /// Active
    pub active: bool,
}

impl TransitionSound {
    pub fn new() -> Self {
        Self {
            tone: Oscillator::new(Waveform::Sine, 440.0),
            envelope: Envelope::new(0.1, 0.2, 0.3, 0.5),
            active: false,
        }
    }

    /// Trigger transition sound for phase
    pub fn trigger_for_phase(&mut self, phase: Phase) {
        let frequency = match phase {
            Phase::Signal => 293.66,     // D4
            Phase::Bang => 220.0,        // A3 (dramatic)
            Phase::Awakening => 329.63,  // E4
            Phase::Discovery => 392.0,   // G4
            Phase::Connection => 440.0,  // A4
            Phase::Acceptance => 493.88, // B4
            Phase::Ended => 587.33,      // D5 (final resolution)
        };

        self.tone.frequency = frequency;
        self.envelope.trigger();
        self.active = true;
    }

    /// Generate sample
    #[allow(dead_code)]
    pub fn sample(&mut self, sample_rate: f32, delta_time: f32) -> f32 {
        if !self.active {
            return 0.0;
        }

        let sample = self.tone.sample(sample_rate);
        let amp = self.envelope.process(delta_time);

        if !self.envelope.is_active() {
            self.active = false;
        }

        sample * amp * 0.2
    }
}

impl Default for TransitionSound {
    fn default() -> Self {
        Self::new()
    }
}
