//! Traveler lifecycle - visibility, fading, grief

use bevy::prelude::*;

use super::{Traveler, TravelerGrief, TravelerState, TravelerVisibility};
use crate::core::{ExperienceClock, TravelerFadedEvent, TravelerFadingEvent, TravelerGriefEvent};
use crate::wide_event;

/// Update visibility based on state
pub fn update_traveler_visibility(
    time: Res<Time>,
    mut travelers: Query<(&TravelerState, &mut TravelerVisibility)>,
) {
    for (state, mut vis) in travelers.iter_mut() {
        // Set target based on state
        vis.target = match state {
            TravelerState::Spawning => 1.0,
            TravelerState::Active => 1.0,
            TravelerState::Grieving => 0.85, // Slightly dimmed
            TravelerState::Fading => 0.0,
            TravelerState::Gone => 0.0,
        };

        // Lerp toward target
        let diff = vis.target - vis.opacity;
        if diff.abs() > 0.001 {
            vis.opacity += diff * vis.speed * time.delta_seconds();
        }
    }
}

/// Transition from Spawning to Active
pub fn finalize_spawn(mut travelers: Query<(&mut TravelerState, &TravelerVisibility)>) {
    for (mut state, vis) in travelers.iter_mut() {
        if *state == TravelerState::Spawning && vis.opacity > 0.95 {
            *state = TravelerState::Active;
        }
    }
}

/// Handle fading events
pub fn handle_traveler_fading(
    mut events: EventReader<TravelerFadingEvent>,
    mut travelers: Query<(&Traveler, &mut TravelerState, &mut TravelerVisibility)>,
    clock: Res<ExperienceClock>,
) {
    for event in events.read() {
        for (traveler, mut state, mut vis) in travelers.iter_mut() {
            if traveler.id == event.id {
                *state = TravelerState::Fading;
                vis.speed = 0.1; // Slow fade

                wide_event!("traveler_fading")
                    .with_str("id", event.id.name())
                    .emit(clock.elapsed());
            }
        }
    }
}

/// Check for fully faded travelers
pub fn check_faded_travelers(
    mut travelers: Query<(&Traveler, &mut TravelerState, &TravelerVisibility)>,
    mut events: EventWriter<TravelerFadedEvent>,
    clock: Res<ExperienceClock>,
) {
    for (traveler, mut state, vis) in travelers.iter_mut() {
        if *state == TravelerState::Fading && vis.opacity < 0.01 {
            *state = TravelerState::Gone;
            events.send(TravelerFadedEvent {
                id: traveler.id,
                elapsed: clock.elapsed(),
            });

            wide_event!("traveler_faded")
                .with_str("id", traveler.id.name())
                .emit(clock.elapsed());
        }
    }
}

/// Handle grief events
pub fn handle_grief_events(
    mut events: EventReader<TravelerGriefEvent>,
    mut travelers: Query<(&Traveler, &mut TravelerState, &mut TravelerGrief)>,
    clock: Res<ExperienceClock>,
) {
    for event in events.read() {
        for (traveler, mut state, mut grief) in travelers.iter_mut() {
            if traveler.id == event.mourner {
                *state = TravelerState::Grieving;
                grief.active = true;
                grief.intensity = 1.0;
                grief.mourning = Some(event.deceased);

                wide_event!("traveler_grieving")
                    .with_str("mourner", event.mourner.name())
                    .with_str("deceased", event.deceased.name())
                    .emit(clock.elapsed());
            }
        }
    }
}

/// Decay grief over time
pub fn decay_grief(time: Res<Time>, mut travelers: Query<(&mut TravelerState, &mut TravelerGrief)>) {
    for (mut state, mut grief) in travelers.iter_mut() {
        if grief.active {
            grief.intensity -= time.delta_seconds() * 0.2; // 5 second decay
            if grief.intensity <= 0.0 {
                grief.active = false;
                grief.intensity = 0.0;
                if *state == TravelerState::Grieving {
                    *state = TravelerState::Active;
                }
            }
        }
    }
}
