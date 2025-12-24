//! Traveler leitmotif definitions

#![allow(dead_code)]

use crate::core::TravelerId;

/// Melodic contour types
#[derive(Clone, Copy, Debug)]
pub enum Contour {
    Ascending,
    Descending,
    Arch,
    Valley,
    Static,
}

/// Rhythmic pattern
#[derive(Clone, Debug)]
pub struct RhythmPattern {
    /// Note durations as fractions of a beat
    pub durations: Vec<f32>,
    /// Rest positions (indices)
    pub rests: Vec<usize>,
}

/// Leitmotif definition for a traveler
#[derive(Clone)]
pub struct Leitmotif {
    #[allow(dead_code)]
    pub traveler: TravelerId,
    /// Preferred scale degrees
    pub preferred_degrees: Vec<usize>,
    /// Typical contour
    pub contour: Contour,
    /// Octave range (low, high)
    pub octave_range: (i32, i32),
    /// Base tempo (BPM)
    pub tempo: f32,
    /// Rhythm pattern
    pub rhythm: RhythmPattern,
    /// Phrase length in notes
    pub phrase_length: usize,
    /// Tendency to use large intervals
    pub interval_tendency: f32,
}

impl Leitmotif {
    /// Create leitmotif for Archivist
    /// Deliberate, measured, uses lower register
    pub fn archivist() -> Self {
        Self {
            traveler: TravelerId::Archivist,
            preferred_degrees: vec![0, 2, 4], // D, G, B
            contour: Contour::Arch,
            octave_range: (0, 1),
            tempo: 60.0,
            rhythm: RhythmPattern {
                durations: vec![1.0, 1.0, 2.0, 1.0, 1.0],
                rests: vec![],
            },
            phrase_length: 5,
            interval_tendency: 0.3, // Prefers small intervals
        }
    }

    /// Create leitmotif for Wanderer
    /// Exploratory, wider range, varied rhythm
    pub fn wanderer() -> Self {
        Self {
            traveler: TravelerId::Wanderer,
            preferred_degrees: vec![1, 2, 3, 4], // E, G, A, B
            contour: Contour::Ascending,
            octave_range: (0, 2),
            tempo: 75.0,
            rhythm: RhythmPattern {
                durations: vec![0.5, 0.5, 1.0, 0.25, 0.25, 0.5, 1.0],
                rests: vec![3],
            },
            phrase_length: 7,
            interval_tendency: 0.6, // Mix of intervals
        }
    }

    /// Create leitmotif for Keeper
    /// Steady, grounded, repetitive
    pub fn keeper() -> Self {
        Self {
            traveler: TravelerId::Keeper,
            preferred_degrees: vec![0, 2, 0, 3], // D, G, D, A - ostinato-like
            contour: Contour::Static,
            octave_range: (0, 1),
            tempo: 55.0,
            rhythm: RhythmPattern {
                durations: vec![1.0, 1.0, 1.0, 1.0],
                rests: vec![],
            },
            phrase_length: 4,
            interval_tendency: 0.2, // Very small intervals
        }
    }

    /// Create leitmotif for Child
    /// Light, high register, playful
    pub fn child() -> Self {
        Self {
            traveler: TravelerId::Child,
            preferred_degrees: vec![2, 3, 4, 3, 2], // G, A, B, A, G
            contour: Contour::Valley,
            octave_range: (1, 2),
            tempo: 90.0,
            rhythm: RhythmPattern {
                durations: vec![0.5, 0.5, 0.5, 0.5, 1.0],
                rests: vec![],
            },
            phrase_length: 5,
            interval_tendency: 0.4,
        }
    }

    /// Create leitmotif for Other
    /// Distant, sparse, unpredictable
    pub fn other() -> Self {
        Self {
            traveler: TravelerId::Other,
            preferred_degrees: vec![0, 4, 1, 3], // D, B, E, A - wide leaps
            contour: Contour::Descending,
            octave_range: (0, 2),
            tempo: 45.0,
            rhythm: RhythmPattern {
                durations: vec![2.0, 1.0, 2.0, 3.0],
                rests: vec![1, 3],
            },
            phrase_length: 4,
            interval_tendency: 0.9, // Large intervals
        }
    }
}
