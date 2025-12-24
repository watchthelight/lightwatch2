//! Synchronization system - travelers sync during Connection phase

use bevy::prelude::*;

use crate::core::{ExperienceClock, Phase, TravelerId, TravelersSyncedEvent};
use crate::travelers::{Traveler, TravelerPulse};

/// Handle sync events
pub fn handle_sync_events(
    mut events: EventReader<TravelersSyncedEvent>,
    mut travelers: Query<(&Traveler, &mut TravelerPulse)>,
) {
    for event in events.read() {
        for (traveler, mut pulse) in travelers.iter_mut() {
            if event.participants.contains(&traveler.id) {
                pulse.synced = true;
                // Offset so they don't all pulse at exact same moment
                pulse.sync_offset = match traveler.id {
                    TravelerId::Archivist => 0.0,
                    TravelerId::Wanderer => 0.15,
                    TravelerId::Keeper => 0.3,
                    TravelerId::Child => 0.45,
                    TravelerId::Other => 0.6, // If ever synced
                };
            }
        }
    }
}

/// Gradual sync during Connection phase
pub fn gradual_sync_during_connection(
    clock: Res<ExperienceClock>,
    mut travelers: Query<&mut TravelerPulse>,
) {
    if clock.phase() != Phase::Connection {
        return;
    }

    let progress = clock.phase_progress();

    // Gradually reduce frequency variance as phase progresses
    for mut pulse in travelers.iter_mut() {
        if pulse.synced {
            // Reduce variance as sync strengthens
            let base_variance = 0.02; // Original variance
            pulse.variance = base_variance * (1.0 - progress * 0.8);
        }
    }
}

/// Break sync during Acceptance
pub fn break_sync_during_acceptance(
    clock: Res<ExperienceClock>,
    mut travelers: Query<&mut TravelerPulse>,
) {
    if clock.phase() != Phase::Acceptance {
        return;
    }

    let progress = clock.phase_progress();

    // Gradually desync as travelers fade
    if progress > 0.3 {
        for mut pulse in travelers.iter_mut() {
            pulse.synced = false;
        }
    }
}
