//! Main audio synthesis engine

use bevy::prelude::*;

use super::Voice;

/// Main audio synthesis engine
#[derive(Resource)]
pub struct AudioEngine {
    pub sample_rate: f32,
    pub master_volume: f32,
    voices: Vec<Voice>,
    max_voices: usize,
}

impl AudioEngine {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            sample_rate,
            master_volume: 0.5,
            voices: Vec::new(),
            max_voices: 16,
        }
    }

    /// Play a note
    #[allow(dead_code)]
    pub fn play_note(&mut self, frequency: f32) {
        // Find inactive voice or steal oldest
        let voice_idx = self
            .voices
            .iter()
            .position(|v| !v.is_active())
            .unwrap_or_else(|| {
                if self.voices.len() < self.max_voices {
                    self.voices.push(Voice::new(self.sample_rate));
                    self.voices.len() - 1
                } else {
                    0 // Steal first voice
                }
            });

        if voice_idx < self.voices.len() {
            self.voices[voice_idx].trigger(frequency);
        }
    }

    /// Release all notes at frequency
    #[allow(dead_code)]
    pub fn release_note(&mut self, frequency: f32) {
        for voice in &mut self.voices {
            if (voice.oscillator.frequency - frequency).abs() < 0.1 {
                voice.release();
            }
        }
    }

    /// Generate audio buffer
    #[allow(dead_code)]
    pub fn fill_buffer(&mut self, buffer: &mut [f32]) {
        let delta_time = 1.0 / self.sample_rate;

        for sample in buffer.iter_mut() {
            let mut sum = 0.0;
            for voice in &mut self.voices {
                sum += voice.process(delta_time);
            }
            *sample = sum * self.master_volume;
        }
    }
}

impl Default for AudioEngine {
    fn default() -> Self {
        Self::new(44100.0)
    }
}
