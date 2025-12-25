//! Schroeder reverb implementation

/// Comb filter for reverb
struct CombFilter {
    buffer: Vec<f32>,
    index: usize,
    feedback: f32,
}

impl CombFilter {
    fn new(delay_samples: usize, feedback: f32) -> Self {
        Self {
            buffer: vec![0.0; delay_samples.max(1)],
            index: 0,
            feedback,
        }
    }

    fn process(&mut self, input: f32) -> f32 {
        let output = self.buffer[self.index];
        self.buffer[self.index] = input + output * self.feedback;
        self.index = (self.index + 1) % self.buffer.len();
        output
    }
}

/// All-pass filter for reverb
struct AllpassFilter {
    buffer: Vec<f32>,
    index: usize,
    feedback: f32,
}

impl AllpassFilter {
    fn new(delay_samples: usize, feedback: f32) -> Self {
        Self {
            buffer: vec![0.0; delay_samples.max(1)],
            index: 0,
            feedback,
        }
    }

    fn process(&mut self, input: f32) -> f32 {
        let delayed = self.buffer[self.index];
        let output = -input + delayed;
        self.buffer[self.index] = input + delayed * self.feedback;
        self.index = (self.index + 1) % self.buffer.len();
        output
    }
}

/// Simple Schroeder reverb
pub struct Reverb {
    /// Comb filters
    comb_filters: Vec<CombFilter>,
    /// All-pass filters
    allpass_filters: Vec<AllpassFilter>,
    /// Mix level (0 = dry, 1 = wet)
    pub mix: f32,
}

impl Reverb {
    pub fn new(sample_rate: f32) -> Self {
        // Comb filter delays (in samples) - prime-ish numbers for richness
        let comb_delays = [1557, 1617, 1491, 1422, 1277, 1356, 1188, 1116];
        let comb_feedback = 0.84;

        // All-pass delays
        let allpass_delays = [225, 556, 441, 341];
        let allpass_feedback = 0.5;

        // Scale delays for sample rate (base is 44100)
        let scale = sample_rate / 44100.0;

        let comb_filters = comb_delays
            .iter()
            .map(|&d| CombFilter::new((d as f32 * scale) as usize, comb_feedback))
            .collect();

        let allpass_filters = allpass_delays
            .iter()
            .map(|&d| AllpassFilter::new((d as f32 * scale) as usize, allpass_feedback))
            .collect();

        Self {
            comb_filters,
            allpass_filters,
            mix: 0.3,
        }
    }

    pub fn process(&mut self, input: f32) -> f32 {
        // Sum comb filter outputs
        let mut wet: f32 = self.comb_filters.iter_mut().map(|f| f.process(input)).sum();
        wet /= self.comb_filters.len() as f32;

        // Chain through all-pass filters
        for filter in &mut self.allpass_filters {
            wet = filter.process(wet);
        }

        // Mix dry and wet
        input * (1.0 - self.mix) + wet * self.mix
    }
}

impl Default for Reverb {
    fn default() -> Self {
        Self::new(44100.0)
    }
}
