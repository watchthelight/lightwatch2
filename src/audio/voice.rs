//! Synth voice combining oscillator, filter, and envelope

use super::{BiquadFilter, Envelope, FilterType, Oscillator, Waveform};

/// Synth voice combining oscillator, filter, and envelope
pub struct Voice {
    pub oscillator: Oscillator,
    pub filter: BiquadFilter,
    pub amp_envelope: Envelope,
    pub filter_envelope: Envelope,
    pub filter_env_amount: f32,
    sample_rate: f32,
}

impl Voice {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            oscillator: Oscillator::new(Waveform::Sine, 440.0),
            filter: BiquadFilter::new(FilterType::LowPass, 2000.0, 1.0, sample_rate),
            amp_envelope: Envelope::new(0.01, 0.1, 0.7, 0.3),
            filter_envelope: Envelope::new(0.01, 0.2, 0.3, 0.5),
            filter_env_amount: 1000.0,
            sample_rate,
        }
    }

    /// Trigger voice with frequency
    pub fn trigger(&mut self, frequency: f32) {
        self.oscillator.set_frequency(frequency);
        self.amp_envelope.trigger();
        self.filter_envelope.trigger();
    }

    /// Release voice
    #[allow(dead_code)]
    pub fn release(&mut self) {
        self.amp_envelope.release();
        self.filter_envelope.release();
    }

    /// Process and return sample
    pub fn process(&mut self, delta_time: f32) -> f32 {
        if !self.amp_envelope.is_active() {
            return 0.0;
        }

        // Get oscillator sample
        let osc_sample = self.oscillator.sample(self.sample_rate);

        // Apply filter with envelope modulation
        let filter_env = self.filter_envelope.process(delta_time);
        let base_cutoff = 500.0;
        let cutoff = base_cutoff + filter_env * self.filter_env_amount;
        self.filter.set_cutoff(cutoff);
        let filtered = self.filter.process(osc_sample);

        // Apply amplitude envelope
        let amp = self.amp_envelope.process(delta_time);

        filtered * amp
    }

    pub fn is_active(&self) -> bool {
        self.amp_envelope.is_active()
    }
}
