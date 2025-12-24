//! Musical scale system - D pentatonic

#![allow(dead_code)]

/// Musical note frequencies (A4 = 440Hz)
pub const A4: f32 = 440.0;

/// Calculate frequency from MIDI note number
pub fn midi_to_freq(note: i32) -> f32 {
    A4 * 2.0_f32.powf((note - 69) as f32 / 12.0)
}

/// D pentatonic scale intervals from D
pub const D_PENTATONIC: [i32; 5] = [0, 2, 5, 7, 10]; // D, E, G, A, B

/// Scale degrees
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ScaleDegree {
    Root,   // D
    Second, // E
    Third,  // G (minor 3rd from D)
    Fifth,  // A
    Sixth,  // B
}

impl ScaleDegree {
    #[allow(dead_code)]
    pub fn semitones(&self) -> i32 {
        match self {
            ScaleDegree::Root => 0,
            ScaleDegree::Second => 2,
            ScaleDegree::Third => 5,
            ScaleDegree::Fifth => 7,
            ScaleDegree::Sixth => 10,
        }
    }
}

/// Scale with root note
pub struct Scale {
    root_midi: i32,
    intervals: Vec<i32>,
}

impl Scale {
    pub fn d_pentatonic(octave: i32) -> Self {
        // D2 = MIDI 38, D3 = 50, D4 = 62
        let root = 38 + (octave * 12);
        Self {
            root_midi: root,
            intervals: D_PENTATONIC.to_vec(),
        }
    }

    /// Get frequency for scale degree in given octave offset
    pub fn frequency(&self, degree: usize, octave_offset: i32) -> f32 {
        let interval = self.intervals[degree % self.intervals.len()];
        let midi = self.root_midi + interval + (octave_offset * 12);
        midi_to_freq(midi)
    }

    /// Get all frequencies in range
    #[allow(dead_code)]
    pub fn frequencies_in_range(&self, octaves: i32) -> Vec<f32> {
        let mut freqs = Vec::new();
        for oct in 0..octaves {
            for &interval in &self.intervals {
                freqs.push(midi_to_freq(self.root_midi + interval + oct * 12));
            }
        }
        freqs
    }
}
