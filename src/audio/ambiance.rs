//! Cosmic background ambiance generator

use bevy::prelude::*;

use super::{BiquadFilter, FilterType, Oscillator, Waveform};

/// Cosmic background ambiance generator
#[derive(Resource)]
pub struct CosmicAmbiance {
    /// Low rumble oscillator
    rumble: Oscillator,
    /// High shimmer oscillator
    shimmer: Oscillator,
    /// Noise for texture
    noise: Oscillator,
    /// Low-pass filter for rumble
    rumble_filter: BiquadFilter,
    /// High-pass filter for shimmer
    shimmer_filter: BiquadFilter,
    /// Band-pass for noise
    noise_filter: BiquadFilter,
    /// Master volume
    #[allow(dead_code)]
    pub volume: f32,
    /// Active
    #[allow(dead_code)]
    pub active: bool,
}

impl CosmicAmbiance {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            rumble: Oscillator::new(Waveform::Sine, 30.0),
            shimmer: Oscillator::new(Waveform::Sine, 800.0),
            noise: Oscillator::new(Waveform::Noise, 0.0),
            rumble_filter: BiquadFilter::new(FilterType::LowPass, 60.0, 0.7, sample_rate),
            shimmer_filter: BiquadFilter::new(FilterType::HighPass, 2000.0, 2.0, sample_rate),
            noise_filter: BiquadFilter::new(FilterType::BandPass, 400.0, 0.5, sample_rate),
            volume: 0.15,
            active: true,
        }
    }

    /// Generate ambiance sample
    #[allow(dead_code)]
    pub fn sample(&mut self, sample_rate: f32) -> f32 {
        if !self.active {
            return 0.0;
        }

        // Slowly modulating rumble
        let rumble = self.rumble.sample(sample_rate);
        let rumble_filtered = self.rumble_filter.process(rumble) * 0.5;

        // Subtle shimmer
        let shimmer = self.shimmer.sample(sample_rate);
        let shimmer_filtered = self.shimmer_filter.process(shimmer) * 0.1;

        // Noise texture
        let noise = self.noise.sample(sample_rate);
        let noise_filtered = self.noise_filter.process(noise) * 0.05;

        (rumble_filtered + shimmer_filtered + noise_filtered) * self.volume
    }
}

impl Default for CosmicAmbiance {
    fn default() -> Self {
        Self::new(44100.0)
    }
}
