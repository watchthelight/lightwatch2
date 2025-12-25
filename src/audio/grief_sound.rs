//! Grief dissonance sound effect

use super::{Envelope, Oscillator, Waveform};

/// Grief dissonance - triggered when Child fades first
pub struct GriefDissonance {
    /// Dissonant cluster oscillators
    oscillators: [Oscillator; 3],
    /// Amplitude envelopes
    envelopes: [Envelope; 3],
    /// LFO for tremolo
    tremolo: Oscillator,
    /// Time alive
    pub age: f32,
    /// Duration
    pub duration: f32,
    /// Is active
    pub active: bool,
}

impl GriefDissonance {
    pub fn new() -> Self {
        // A3, Bb3, B3 - minor second clusters
        let frequencies = [220.0, 233.0, 247.0];

        Self {
            oscillators: [
                Oscillator::new(Waveform::Triangle, frequencies[0]),
                Oscillator::new(Waveform::Triangle, frequencies[1]),
                Oscillator::new(Waveform::Triangle, frequencies[2]),
            ],
            envelopes: [
                Envelope::new(0.5, 0.5, 0.4, 2.0),
                Envelope::new(0.7, 0.3, 0.5, 2.5),
                Envelope::new(0.3, 0.6, 0.3, 1.5),
            ],
            tremolo: Oscillator::new(Waveform::Sine, 4.0),
            age: 0.0,
            duration: 4.0,
            active: false,
        }
    }

    /// Trigger grief sound
    pub fn trigger(&mut self) {
        self.active = true;
        self.age = 0.0;
        for env in &mut self.envelopes {
            env.trigger();
        }
    }

    /// Generate sample
    pub fn sample(&mut self, sample_rate: f32, delta_time: f32) -> f32 {
        if !self.active {
            return 0.0;
        }

        self.age += delta_time;

        // Tremolo modulation
        let tremolo = 0.7 + 0.3 * self.tremolo.sample(sample_rate);

        // Generate dissonant cluster
        let mut output = 0.0;
        for (osc, env) in self.oscillators.iter_mut().zip(self.envelopes.iter_mut()) {
            let sample = osc.sample(sample_rate);
            let amp = env.process(delta_time);
            output += sample * amp;
        }

        // Check completion
        if self.age >= self.duration {
            self.active = false;
        }

        output * tremolo * 0.4
    }
}

impl Default for GriefDissonance {
    fn default() -> Self {
        Self::new()
    }
}
