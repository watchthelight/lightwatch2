//! Oscillator waveform generators

use std::f32::consts::TAU;

/// Oscillator waveform types
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Waveform {
    Sine,
    Saw,
    Triangle,
    Square,
    Noise,
}

/// Basic oscillator
pub struct Oscillator {
    pub waveform: Waveform,
    pub frequency: f32,
    pub amplitude: f32,
    pub phase: f32,
    noise_state: u32,
}

impl Oscillator {
    pub fn new(waveform: Waveform, frequency: f32) -> Self {
        Self {
            waveform,
            frequency,
            amplitude: 1.0,
            phase: 0.0,
            noise_state: 0xDEADBEEF,
        }
    }

    /// Generate next sample
    pub fn sample(&mut self, sample_rate: f32) -> f32 {
        let value = match self.waveform {
            Waveform::Sine => (self.phase * TAU).sin(),
            Waveform::Saw => 2.0 * self.phase - 1.0,
            Waveform::Triangle => 4.0 * (self.phase - 0.5).abs() - 1.0,
            Waveform::Square => {
                if self.phase < 0.5 {
                    1.0
                } else {
                    -1.0
                }
            }
            Waveform::Noise => self.white_noise(),
        };

        // Advance phase
        self.phase += self.frequency / sample_rate;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        value * self.amplitude
    }

    /// Simple white noise (xorshift)
    fn white_noise(&mut self) -> f32 {
        self.noise_state ^= self.noise_state << 13;
        self.noise_state ^= self.noise_state >> 17;
        self.noise_state ^= self.noise_state << 5;
        (self.noise_state as f32 / u32::MAX as f32) * 2.0 - 1.0
    }

    /// Set frequency
    pub fn set_frequency(&mut self, freq: f32) {
        self.frequency = freq;
    }
}
