//! Trail particles - motion trails for travelers

use bevy::prelude::*;
use std::collections::VecDeque;

use crate::core::{ExperienceClock, Phase, TravelerId};
use crate::travelers::Traveler;

/// Configuration for traveler trails
#[derive(Component, Debug)]
pub struct TravelerTrail {
    /// Maximum trail length (positions stored)
    pub max_length: usize,
    /// Minimum distance to spawn new trail point
    pub min_distance: f32,
    /// Trail particle size
    pub size: f32,
    /// Base color
    pub color: Color,
    /// Fade duration (seconds)
    pub fade_duration: f32,
    /// Is trail active?
    pub active: bool,
}

impl TravelerTrail {
    pub fn for_traveler(id: TravelerId) -> Self {
        match id {
            TravelerId::Archivist => Self {
                max_length: 20,
                min_distance: 0.15,
                size: 0.08,
                color: Color::srgba(0.91, 0.64, 0.27, 0.5),
                fade_duration: 1.5,
                active: true,
            },
            TravelerId::Wanderer => Self {
                max_length: 30, // Longer trails
                min_distance: 0.1,
                size: 0.06,
                color: Color::srgba(0.31, 0.80, 0.77, 0.4),
                fade_duration: 2.0,
                active: true,
            },
            TravelerId::Keeper => Self {
                max_length: 15,
                min_distance: 0.2,
                size: 0.1,
                color: Color::srgba(0.83, 0.46, 0.18, 0.6),
                fade_duration: 1.0,
                active: true,
            },
            TravelerId::Child => Self {
                max_length: 25,
                min_distance: 0.08,
                size: 0.05,
                color: Color::srgba(0.96, 0.94, 0.91, 0.3),
                fade_duration: 0.8, // Quick fade
                active: true,
            },
            TravelerId::Other => Self {
                max_length: 40, // Long, persistent trails
                min_distance: 0.25,
                size: 0.12,
                color: Color::srgba(0.42, 0.36, 0.58, 0.2),
                fade_duration: 3.0,
                active: true,
            },
        }
    }
}

/// Position history for trail spawning
#[derive(Component, Debug, Default)]
pub struct PositionHistory {
    /// Previous positions with timestamps
    pub positions: VecDeque<(Vec3, f32)>,
    /// Last recorded position
    pub last_position: Option<Vec3>,
}

/// Individual trail particle
#[derive(Component, Debug)]
pub struct TrailParticle {
    /// Time when spawned
    pub spawn_time: f32,
    /// Fade duration
    pub fade_duration: f32,
    /// Original color
    pub color: Color,
    /// Original size
    pub size: f32,
}

/// Marker for trail particles
#[derive(Component)]
pub struct TrailParticleMarker;

/// Cache for trail mesh
#[derive(Resource)]
pub struct TrailMeshCache {
    pub mesh: Handle<Mesh>,
}

/// Add trail components when traveler spawns
pub fn setup_traveler_trails(
    mut commands: Commands,
    travelers: Query<(Entity, &Traveler), Added<Traveler>>,
) {
    for (entity, traveler) in travelers.iter() {
        commands.entity(entity).insert((
            TravelerTrail::for_traveler(traveler.id),
            PositionHistory::default(),
        ));
    }
}

/// Update position history for trail-enabled entities
pub fn update_position_history(
    time: Res<Time>,
    mut travelers: Query<(&GlobalTransform, &TravelerTrail, &mut PositionHistory)>,
) {
    let current_time = time.elapsed_seconds();

    for (transform, trail, mut history) in travelers.iter_mut() {
        if !trail.active {
            continue;
        }

        let current_pos = transform.translation();

        // Check if we've moved enough to record
        let should_record = match history.last_position {
            Some(last) => (current_pos - last).length() >= trail.min_distance,
            None => true,
        };

        if should_record {
            history.positions.push_back((current_pos, current_time));
            history.last_position = Some(current_pos);

            // Trim old positions
            while history.positions.len() > trail.max_length {
                history.positions.pop_front();
            }
        }

        // Remove positions older than fade duration
        while let Some(&(_, timestamp)) = history.positions.front() {
            if current_time - timestamp > trail.fade_duration {
                history.positions.pop_front();
            } else {
                break;
            }
        }
    }
}

/// Spawn trail particles at history positions
pub fn spawn_trail_particles(
    mut commands: Commands,
    time: Res<Time>,
    travelers: Query<(&TravelerTrail, &PositionHistory), Changed<PositionHistory>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cache: Option<Res<TrailMeshCache>>,
) {
    let current_time = time.elapsed_seconds();

    // Use cached mesh or create one
    let trail_mesh = match cache {
        Some(c) => c.mesh.clone(),
        None => meshes.add(Sphere::new(1.0).mesh().ico(0).unwrap()),
    };

    for (trail, history) in travelers.iter() {
        if !trail.active {
            continue;
        }

        // Only spawn for the most recent position
        if let Some(&(position, timestamp)) = history.positions.back() {
            // Only spawn for very recent positions (within this frame)
            if current_time - timestamp > 0.05 {
                continue;
            }

            let srgba = trail.color.to_srgba();

            let trail_material = materials.add(StandardMaterial {
                base_color: trail.color,
                emissive: LinearRgba::new(srgba.red * 0.5, srgba.green * 0.5, srgba.blue * 0.5, 1.0),
                unlit: true,
                alpha_mode: AlphaMode::Blend,
                ..default()
            });

            commands.spawn((
                PbrBundle {
                    mesh: trail_mesh.clone(),
                    material: trail_material,
                    transform: Transform::from_translation(position)
                        .with_scale(Vec3::splat(trail.size)),
                    ..default()
                },
                TrailParticle {
                    spawn_time: timestamp,
                    fade_duration: trail.fade_duration,
                    color: trail.color,
                    size: trail.size,
                },
                TrailParticleMarker,
            ));
        }
    }
}

/// Update trail particle opacity and cleanup dead particles
pub fn update_trail_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut particles: Query<
        (Entity, &TrailParticle, &mut Transform, &Handle<StandardMaterial>),
        With<TrailParticleMarker>,
    >,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let current_time = time.elapsed_seconds();

    for (entity, particle, mut transform, material_handle) in particles.iter_mut() {
        let age = current_time - particle.spawn_time;
        let life_fraction = age / particle.fade_duration;

        if life_fraction >= 1.0 {
            // Particle is dead, despawn
            commands.entity(entity).despawn();
            continue;
        }

        // Quadratic fade out
        let alpha = (1.0 - life_fraction).powi(2);

        // Shrink over time
        let scale = (1.0 - life_fraction * 0.5) * particle.size;
        transform.scale = Vec3::splat(scale.max(0.001));

        // Update material
        if let Some(material) = materials.get_mut(material_handle) {
            let srgba = particle.color.to_srgba();
            material.base_color = Color::srgba(srgba.red, srgba.green, srgba.blue, srgba.alpha * alpha);
            material.emissive = LinearRgba::new(
                srgba.red * alpha * 0.5,
                srgba.green * alpha * 0.5,
                srgba.blue * alpha * 0.5,
                1.0,
            );
        }
    }
}

/// Activate/deactivate trails based on phase and adjust properties
pub fn control_trail_activation(
    clock: Res<ExperienceClock>,
    mut trails: Query<(&Traveler, &mut TravelerTrail, &crate::travelers::TravelerVisibility)>,
) {
    let phase = clock.phase();
    let progress = clock.phase_progress();

    for (traveler, mut trail, visibility) in trails.iter_mut() {
        // Trails are active during movement phases
        trail.active = matches!(
            phase,
            Phase::Discovery | Phase::Connection | Phase::Acceptance
        );

        // The Other always has trails (otherworldly presence)
        if traveler.id == TravelerId::Other {
            trail.active = true;
        }

        // During Acceptance, make trails more ethereal as travelers fade
        if phase == Phase::Acceptance {
            // Shorter duration for ethereal feel
            let base_duration = TravelerTrail::for_traveler(traveler.id).fade_duration;
            trail.fade_duration = base_duration * (0.4 + 0.6 * (1.0 - progress));

            // Smaller size as travelers fade
            let base_size = TravelerTrail::for_traveler(traveler.id).size;
            trail.size = base_size * (0.5 + 0.5 * visibility.opacity);
        } else {
            // Reset to defaults in other phases
            let default_trail = TravelerTrail::for_traveler(traveler.id);
            trail.fade_duration = default_trail.fade_duration;
            trail.size = default_trail.size;
        }
    }
}

/// Initialize trail mesh cache
pub fn init_trail_mesh_cache(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let mesh = meshes.add(Sphere::new(1.0).mesh().ico(0).unwrap());
    commands.insert_resource(TrailMeshCache { mesh });
}
