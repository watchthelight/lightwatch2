//! Grief behavior - responses to companion loss

use bevy::prelude::*;

use crate::core::{TravelerGriefEvent, TravelerId};
use crate::travelers::{Traveler, TravelerGrief, TravelerPulse};

use super::TravelerDrift;

/// Grief-induced behavior changes
pub fn apply_grief_behavior(
    time: Res<Time>,
    mut travelers: Query<(&Traveler, &TravelerGrief, &mut TravelerPulse, &mut Transform)>,
) {
    for (_traveler, grief, mut pulse, mut transform) in travelers.iter_mut() {
        if !grief.active {
            continue;
        }

        // Grief disrupts pulse
        pulse.variance *= 1.0 + grief.intensity * 0.5;

        // Grief causes slight tremor
        let tremor = Vec3::new(
            (time.elapsed_seconds() * 15.0).sin() * 0.01 * grief.intensity,
            (time.elapsed_seconds() * 17.0).cos() * 0.01 * grief.intensity,
            0.0,
        );
        transform.translation += tremor;

        // Grief slows pulse
        pulse.frequency *= 1.0 - grief.intensity * 0.3;
    }
}

/// Specific grief responses per traveler
pub fn traveler_specific_grief(
    mut events: EventReader<TravelerGriefEvent>,
    mut travelers: Query<(&Traveler, &mut TravelerDrift)>,
) {
    for event in events.read() {
        for (traveler, mut drift) in travelers.iter_mut() {
            if traveler.id != event.mourner {
                continue;
            }

            match traveler.id {
                TravelerId::Archivist => {
                    // Archivist freezes briefly
                    drift.active = false;
                }
                TravelerId::Wanderer => {
                    // Wanderer moves erratically
                    drift.max_speed *= 2.0;
                }
                TravelerId::Keeper => {
                    // Keeper dims but stays steady
                    drift.max_speed *= 0.5;
                }
                _ => {}
            }
        }
    }
}
