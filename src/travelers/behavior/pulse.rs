//! Pulse system - rhythmic pulsing for travelers

use bevy::prelude::*;

use crate::core::TravelerId;
use crate::travelers::{Traveler, TravelerPulse};

/// Update traveler pulse state
pub fn update_traveler_pulse(time: Res<Time>, mut travelers: Query<(&Traveler, &mut TravelerPulse)>) {
    let t = time.elapsed_seconds();

    for (traveler, mut pulse) in travelers.iter_mut() {
        // Natural frequency with variance
        let variance_offset = (t * 0.3).sin() * pulse.variance;
        let effective_freq = pulse.frequency + variance_offset;

        // Update phase
        pulse.phase = (t * effective_freq) % 1.0;

        // Calculate intensity (sinusoidal pulse)
        let base_intensity = (pulse.phase * std::f32::consts::TAU).sin() * 0.5 + 0.5;

        // Apply character-specific pulse shape
        pulse.intensity = match traveler.id {
            TravelerId::Archivist => base_intensity,          // Smooth
            TravelerId::Wanderer => base_intensity.powf(0.7), // Sharper
            TravelerId::Keeper => base_intensity.powf(1.5),   // Softer
            TravelerId::Child => (base_intensity * 2.0).min(1.0), // Quick bright
            TravelerId::Other => base_intensity.powf(2.0),    // Very subtle
        };

        // Sync mode modifies phase
        if pulse.synced {
            pulse.phase = (t * 0.14 + pulse.sync_offset) % 1.0; // All sync to Archivist's frequency
        }
    }
}
