//! Traveler state - lifecycle, visibility, pulse, grief

use bevy::prelude::*;

use crate::core::TravelerId;

/// Traveler lifecycle state
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TravelerState {
    /// Spawning in (fade up)
    Spawning,
    /// Fully alive and active
    Active,
    /// In grief response
    Grieving,
    /// Fading out (dying)
    Fading,
    /// Fully gone
    Gone,
}

impl Default for TravelerState {
    fn default() -> Self {
        TravelerState::Spawning
    }
}

/// Traveler visibility/opacity
#[derive(Component, Debug)]
pub struct TravelerVisibility {
    /// Current opacity (0.0 - 1.0)
    pub opacity: f32,
    /// Target opacity
    pub target: f32,
    /// Transition speed
    pub speed: f32,
}

impl Default for TravelerVisibility {
    fn default() -> Self {
        Self {
            opacity: 0.0,
            target: 1.0,
            speed: 0.5,
        }
    }
}

/// Traveler pulse state
#[derive(Component, Debug)]
pub struct TravelerPulse {
    /// Current pulse phase (0.0 - 1.0)
    pub phase: f32,
    /// Current pulse intensity
    pub intensity: f32,
    /// Base frequency in Hz
    pub frequency: f32,
    /// Frequency variance
    pub variance: f32,
    /// Is synchronized with others?
    pub synced: bool,
    /// Sync phase offset
    pub sync_offset: f32,
}

impl TravelerPulse {
    pub fn new(frequency: f32, variance: f32) -> Self {
        Self {
            phase: 0.0,
            intensity: 0.0,
            frequency,
            variance,
            synced: false,
            sync_offset: 0.0,
        }
    }
}

/// Grief state for a traveler
#[derive(Component, Debug)]
pub struct TravelerGrief {
    /// Is in grief?
    pub active: bool,
    /// Grief intensity (decays over time)
    pub intensity: f32,
    /// Who died?
    pub mourning: Option<TravelerId>,
}

impl Default for TravelerGrief {
    fn default() -> Self {
        Self {
            active: false,
            intensity: 0.0,
            mourning: None,
        }
    }
}
