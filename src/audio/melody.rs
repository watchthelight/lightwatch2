//! Procedural melody generation

use super::leitmotif::{Contour, Leitmotif};
use super::scale::Scale;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

/// Generated melody
pub struct Melody {
    /// Frequencies of notes
    pub notes: Vec<f32>,
    /// Durations of notes (in beats)
    pub durations: Vec<f32>,
    /// Is this a rest?
    pub is_rest: Vec<bool>,
}

/// Melody generator
pub struct MelodyGenerator {
    scale: Scale,
    rng: ChaCha8Rng,
}

impl MelodyGenerator {
    pub fn new(seed: u64) -> Self {
        Self {
            scale: Scale::d_pentatonic(3), // D3 as root
            rng: ChaCha8Rng::seed_from_u64(seed),
        }
    }

    /// Generate melody from leitmotif
    pub fn generate(&mut self, leitmotif: &Leitmotif) -> Melody {
        let mut notes = Vec::new();
        let mut durations = Vec::new();
        let mut is_rest = Vec::new();

        let mut current_degree = *leitmotif
            .preferred_degrees
            .choose(&mut self.rng)
            .unwrap_or(&0);
        let mut current_octave = leitmotif.octave_range.0;

        for i in 0..leitmotif.phrase_length {
            // Check if this is a rest
            let rest = leitmotif.rhythm.rests.contains(&i);
            is_rest.push(rest);

            if rest {
                notes.push(0.0);
            } else {
                // Choose next note based on contour and tendency
                let (next_degree, next_octave) = self.choose_next_note(
                    current_degree,
                    current_octave,
                    leitmotif,
                    i as f32 / leitmotif.phrase_length as f32,
                );

                current_degree = next_degree;
                current_octave = next_octave;

                let freq = self.scale.frequency(current_degree, current_octave);
                notes.push(freq);
            }

            // Get duration
            let dur_idx = i % leitmotif.rhythm.durations.len();
            durations.push(leitmotif.rhythm.durations[dur_idx]);
        }

        Melody {
            notes,
            durations,
            is_rest,
        }
    }

    fn choose_next_note(
        &mut self,
        current_degree: usize,
        current_octave: i32,
        leitmotif: &Leitmotif,
        progress: f32,
    ) -> (usize, i32) {
        // Determine direction based on contour
        let direction = match leitmotif.contour {
            Contour::Ascending => 1,
            Contour::Descending => -1,
            Contour::Arch => {
                if progress < 0.5 {
                    1
                } else {
                    -1
                }
            }
            Contour::Valley => {
                if progress < 0.5 {
                    -1
                } else {
                    1
                }
            }
            Contour::Static => 0,
        };

        // Determine interval size
        let large_interval = self.rng.gen::<f32>() < leitmotif.interval_tendency;
        let step = if large_interval { 2 } else { 1 };

        // Calculate new degree
        let mut new_degree = current_degree as i32 + (direction * step);
        let mut new_octave = current_octave;

        // Handle octave wrapping
        if new_degree >= 5 {
            new_degree -= 5;
            new_octave += 1;
        } else if new_degree < 0 {
            new_degree += 5;
            new_octave -= 1;
        }

        // Clamp octave
        new_octave = new_octave.clamp(leitmotif.octave_range.0, leitmotif.octave_range.1);

        // Bias toward preferred degrees
        if self.rng.gen::<f32>() < 0.3 {
            new_degree = *leitmotif
                .preferred_degrees
                .choose(&mut self.rng)
                .unwrap_or(&0) as i32;
        }

        (new_degree as usize, new_octave)
    }
}
