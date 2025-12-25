//! Bang rumble sound effect

use super::{BiquadFilter, Envelope, FilterType, Oscillator, Waveform};

/// Bang rumble generator
pub struct BangRumble {
    /// Sub-bass oscillator
    sub: Oscillator,
    /// Mid rumble
    mid: Oscillator,
    /// Noise layer
    noise: Oscillator,
    /// Low-pass filter
    filter: BiquadFilter,
    /// Amplitude envelope
    envelope: Envelope,
    /// Time alive
    pub age: f32,
    /// Total duration
    pub duration: f32,
    /// Is active
    pub active: bool,
}

impl BangRumble {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            sub: Oscillator::new(Waveform::Sine, 30.0),
            mid: Oscillator::new(Waveform::Sine, 60.0),
            noise: Oscillator::new(Waveform::Noise, 0.0),
            filter: BiquadFilter::new(FilterType::LowPass, 100.0, 0.7, sample_rate),
            envelope: Envelope::new(0.01, 0.5, 0.6, 4.0),
            age: 0.0,
            duration: 6.0,
            active: false,
        }
    }

    /// Trigger the bang rumble
    pub fn trigger(&mut self) {
        self.active = true;
        self.age = 0.0;
        self.envelope.trigger();
    }

    /// Generate sample
    pub fn sample(&mut self, sample_rate: f32, delta_time: f32) -> f32 {
        if !self.active {
            return 0.0;
        }

        self.age += delta_time;

        // Progress through rumble
        let progress = self.age / self.duration;

        // Pitch drops over time
        let pitch_factor = 1.0 - progress * 0.5;
        self.sub.frequency = 30.0 * pitch_factor;
        self.mid.frequency = 60.0 * pitch_factor;

        // Filter opens then closes
        let filter_freq = if progress < 0.2 {
            100.0 + (progress / 0.2) * 400.0
        } else {
            500.0 - (progress - 0.2) / 0.8 * 450.0
        };
        self.filter.set_cutoff(filter_freq);

        // Generate layers
        let sub_sample = self.sub.sample(sample_rate) * 0.7;
        let mid_sample = self.mid.sample(sample_rate) * 0.4;
        let noise_sample = self.noise.sample(sample_rate) * 0.2;

        // Filter and envelope
        let mixed = sub_sample + mid_sample + noise_sample;
        let filtered = self.filter.process(mixed);
        let amp = self.envelope.process(delta_time);

        // Check completion
        if self.age >= self.duration {
            self.active = false;
        }

        filtered * amp
    }
}

impl Default for BangRumble {
    fn default() -> Self {
        Self::new(44100.0)
    }
}
