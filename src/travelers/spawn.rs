//! Traveler spawn system - entity creation and registry

use bevy::prelude::*;

use super::{Traveler, TravelerDef, TravelerGrief, TravelerPulse, TravelerState, TravelerVisibility};
use crate::core::{TravelerId, TravelerSpawnedEvent};
use crate::wide_event;

/// Bundle for spawning a traveler entity
#[derive(Bundle)]
pub struct TravelerBundle {
    pub traveler: Traveler,
    pub state: TravelerState,
    pub visibility: TravelerVisibility,
    pub pulse: TravelerPulse,
    pub grief: TravelerGrief,
    pub transform: TransformBundle,
}

impl TravelerBundle {
    pub fn new(id: TravelerId, spawn_time: f32) -> Self {
        let def = TravelerDef::get(id);

        Self {
            traveler: Traveler::new(id, spawn_time),
            state: TravelerState::Spawning,
            visibility: TravelerVisibility::default(),
            pulse: TravelerPulse::new(def.rhythm.base_hz, def.rhythm.variance),
            grief: TravelerGrief::default(),
            transform: TransformBundle::from_transform(Transform::from_translation(
                def.spawn_position,
            )),
        }
    }
}

/// Handle traveler spawn events
pub fn handle_traveler_spawns(
    mut commands: Commands,
    mut events: EventReader<TravelerSpawnedEvent>,
    existing: Query<&Traveler>,
    clock: Res<crate::core::ExperienceClock>,
) {
    for event in events.read() {
        // Check if already spawned
        let already_exists = existing.iter().any(|t| t.id == event.id);
        if already_exists {
            warn!("Traveler {:?} already exists, skipping spawn", event.id);
            continue;
        }

        // Spawn the traveler
        let bundle = TravelerBundle::new(event.id, event.elapsed);
        commands.spawn(bundle);

        wide_event!("traveler_spawned")
            .with_str("id", event.id.name())
            .with_str("name", event.id.display_name())
            .emit(clock.elapsed());
    }
}

/// Update traveler registry
#[derive(Resource, Default)]
pub struct TravelerRegistry {
    pub spawned: Vec<TravelerId>,
    pub active: Vec<TravelerId>,
    pub faded: Vec<TravelerId>,
}

pub fn update_traveler_registry(
    mut registry: ResMut<TravelerRegistry>,
    travelers: Query<(&Traveler, &TravelerState)>,
) {
    registry.spawned.clear();
    registry.active.clear();

    for (traveler, state) in travelers.iter() {
        registry.spawned.push(traveler.id);
        if *state == TravelerState::Active {
            registry.active.push(traveler.id);
        }
    }
}
