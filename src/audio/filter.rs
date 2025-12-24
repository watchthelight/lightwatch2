//! Biquad filter implementation

use std::f32::consts::TAU;

/// Filter types
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FilterType {
    LowPass,
    HighPass,
    BandPass,
}

/// Biquad filter implementation
pub struct BiquadFilter {
    filter_type: FilterType,
    cutoff: f32,
    resonance: f32,
    sample_rate: f32,
    // Coefficients
    a0: f32,
    a1: f32,
    a2: f32,
    b1: f32,
    b2: f32,
    // State
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
}

impl BiquadFilter {
    pub fn new(filter_type: FilterType, cutoff: f32, resonance: f32, sample_rate: f32) -> Self {
        let mut filter = Self {
            filter_type,
            cutoff,
            resonance,
            sample_rate,
            a0: 1.0,
            a1: 0.0,
            a2: 0.0,
            b1: 0.0,
            b2: 0.0,
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
        };
        filter.calculate_coefficients();
        filter
    }

    /// Recalculate filter coefficients
    pub fn calculate_coefficients(&mut self) {
        let omega = TAU * self.cutoff / self.sample_rate;
        let sin_omega = omega.sin();
        let cos_omega = omega.cos();
        let alpha = sin_omega / (2.0 * self.resonance);

        match self.filter_type {
            FilterType::LowPass => {
                let b0 = (1.0 - cos_omega) / 2.0;
                let b1 = 1.0 - cos_omega;
                let b2 = (1.0 - cos_omega) / 2.0;
                let a0 = 1.0 + alpha;
                let a1 = -2.0 * cos_omega;
                let a2 = 1.0 - alpha;

                self.a0 = b0 / a0;
                self.a1 = b1 / a0;
                self.a2 = b2 / a0;
                self.b1 = a1 / a0;
                self.b2 = a2 / a0;
            }
            FilterType::HighPass => {
                let b0 = (1.0 + cos_omega) / 2.0;
                let b1 = -(1.0 + cos_omega);
                let b2 = (1.0 + cos_omega) / 2.0;
                let a0 = 1.0 + alpha;
                let a1 = -2.0 * cos_omega;
                let a2 = 1.0 - alpha;

                self.a0 = b0 / a0;
                self.a1 = b1 / a0;
                self.a2 = b2 / a0;
                self.b1 = a1 / a0;
                self.b2 = a2 / a0;
            }
            FilterType::BandPass => {
                let b0 = alpha;
                let b1 = 0.0;
                let b2 = -alpha;
                let a0 = 1.0 + alpha;
                let a1 = -2.0 * cos_omega;
                let a2 = 1.0 - alpha;

                self.a0 = b0 / a0;
                self.a1 = b1 / a0;
                self.a2 = b2 / a0;
                self.b1 = a1 / a0;
                self.b2 = a2 / a0;
            }
        }
    }

    /// Process single sample
    pub fn process(&mut self, input: f32) -> f32 {
        let output =
            self.a0 * input + self.a1 * self.x1 + self.a2 * self.x2 - self.b1 * self.y1
                - self.b2 * self.y2;

        self.x2 = self.x1;
        self.x1 = input;
        self.y2 = self.y1;
        self.y1 = output;

        output
    }

    /// Update cutoff frequency
    pub fn set_cutoff(&mut self, cutoff: f32) {
        self.cutoff = cutoff.clamp(20.0, self.sample_rate * 0.49);
        self.calculate_coefficients();
    }

    /// Update resonance
    #[allow(dead_code)]
    pub fn set_resonance(&mut self, resonance: f32) {
        self.resonance = resonance.clamp(0.1, 10.0);
        self.calculate_coefficients();
    }
}
