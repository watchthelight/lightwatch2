//! Traveler spawn system - entity creation and registry

#![allow(dead_code)]

use bevy::prelude::*;

use super::{
    Traveler, TravelerDef, TravelerGlowMaterial, TravelerGrief, TravelerMeshCache, TravelerPulse,
    TravelerShellMaterial, TravelerState, TravelerVisibility,
};
use crate::audio::SpatialAudioSource;
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
    pub spatial_audio: SpatialAudioSource,
    pub spatial: SpatialBundle,
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
            spatial_audio: SpatialAudioSource {
                volume: 1.0,
                previous_position: def.spawn_position,
                computed_gain: 1.0,
                computed_pan: 0.0,
                computed_pitch: 1.0,
            },
            spatial: SpatialBundle::from_transform(Transform::from_translation(
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

/// Marker component indicating visuals have been spawned
#[derive(Component)]
pub struct TravelerVisualsSpawned;

/// Spawn visual meshes for travelers that need them
pub fn spawn_traveler_visuals(
    mut commands: Commands,
    travelers: Query<(Entity, &Traveler), Without<TravelerVisualsSpawned>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mesh_cache: ResMut<TravelerMeshCache>,
    mut glow_materials: ResMut<Assets<TravelerGlowMaterial>>,
    mut shell_materials: ResMut<Assets<TravelerShellMaterial>>,
) {
    for (entity, traveler) in travelers.iter() {
        let def = TravelerDef::get(traveler.id);

        // Get or create meshes for this geometry type
        let (core_mesh, shell_mesh, _edge_mesh) = mesh_cache.get_or_create(def.geometry, &mut meshes);

        // Create glow material with traveler's color
        let base_srgba = def.color.base.to_srgba();
        let glow_material = glow_materials.add(TravelerGlowMaterial {
            base_color: LinearRgba::new(base_srgba.red, base_srgba.green, base_srgba.blue, 1.0),
            emissive: LinearRgba::new(
                base_srgba.red * 2.0,
                base_srgba.green * 2.0,
                base_srgba.blue * 2.0,
                1.0,
            ),
            pulse_intensity: 0.3,
            pulse_phase: traveler.id as u8 as f32 * 0.7, // Different phase per traveler
            time: 0.0,
            fresnel_power: 3.0,
            inner_glow_strength: 0.5,
            rim_color: LinearRgba::new(1.0, 0.95, 0.9, 1.0),
            grief_amount: 0.0,
            _padding: Vec3::ZERO,
        });

        // Create shell material for translucent outer layer
        let shell_material = shell_materials.add(TravelerShellMaterial {
            base_color: LinearRgba::new(base_srgba.red, base_srgba.green, base_srgba.blue, 0.2),
            refraction_strength: 0.1,
            thickness: 0.5,
            ior: 1.5,
            pulse_intensity: 0.15,
            time: 0.0,
            _padding: Vec3::ZERO,
        });

        // Spawn visual meshes as children
        commands.entity(entity).with_children(|parent| {
            // Core mesh with glow material
            parent.spawn(MaterialMeshBundle {
                mesh: core_mesh,
                material: glow_material,
                transform: Transform::from_scale(Vec3::splat(0.5)), // Traveler size
                ..default()
            });

            // Shell mesh with translucent material (slightly larger)
            parent.spawn(MaterialMeshBundle {
                mesh: shell_mesh,
                material: shell_material,
                transform: Transform::from_scale(Vec3::splat(0.55)), // Slightly larger than core
                ..default()
            });
        });

        // Mark as having visuals
        commands.entity(entity).insert(TravelerVisualsSpawned);

        info!(
            target: "lightwatch::travelers",
            "Spawned visual meshes for {:?} with glow and shell shaders",
            traveler.id
        );
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
