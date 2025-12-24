//! Debris particles - 5000 particles burst from bang origin

use bevy::prelude::*;
use rand::prelude::*;

use super::BangConfig;
use crate::core::{ExperienceClock, TravelerId};

/// Debris particle system configuration
#[derive(Resource)]
pub struct DebrisConfig {
    /// Number of particles
    pub count: usize,
    /// Initial velocity range
    pub min_velocity: f32,
    pub max_velocity: f32,
    /// Velocity decay rate
    pub velocity_decay: f32,
    /// Lifetime range
    pub min_lifetime: f32,
    pub max_lifetime: f32,
    /// Particles that persist as seeds
    pub seed_count: usize,
}

impl Default for DebrisConfig {
    fn default() -> Self {
        Self {
            count: 5000,
            min_velocity: 10.0,
            max_velocity: 50.0,
            velocity_decay: 0.98,
            min_lifetime: 2.0,
            max_lifetime: 8.0,
            seed_count: 5, // One for each traveler
        }
    }
}

/// Individual debris particle
#[derive(Component)]
pub struct DebrisParticle {
    /// Current velocity
    pub velocity: Vec3,
    /// Time alive
    pub age: f32,
    /// Maximum lifetime
    pub lifetime: f32,
    /// Is this a seed particle
    pub is_seed: bool,
    /// Seed index (if is_seed)
    #[allow(dead_code)]
    pub seed_index: Option<usize>,
}

/// Debris system state
#[derive(Resource, Default)]
pub struct DebrisState {
    pub spawned: bool,
    #[allow(dead_code)]
    pub spawn_time: f32,
}

/// Marker for traveler spawn location
#[derive(Component)]
pub struct TravelerSpawnMarker {
    #[allow(dead_code)]
    pub id: TravelerId,
}

/// Spawn debris at peak
pub fn spawn_debris(
    mut commands: Commands,
    clock: Res<ExperienceClock>,
    bang_config: Res<BangConfig>,
    debris_config: Res<DebrisConfig>,
    mut state: ResMut<DebrisState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if state.spawned {
        return;
    }

    let elapsed = clock.elapsed();
    if elapsed < bang_config.peak_time {
        return;
    }

    let mut rng = rand::thread_rng();

    // Particle mesh (small icosphere)
    let mesh = meshes.add(Sphere::new(0.05).mesh().ico(1).unwrap());

    // Spawn particles
    for i in 0..debris_config.count {
        // Random direction (spherical)
        let theta = rng.gen::<f32>() * std::f32::consts::TAU;
        let phi = rng.gen::<f32>() * std::f32::consts::PI;
        let direction =
            Vec3::new(phi.sin() * theta.cos(), phi.sin() * theta.sin(), phi.cos()).normalize();

        // Random velocity
        let speed = rng.gen_range(debris_config.min_velocity..debris_config.max_velocity);
        let velocity = direction * speed;

        // Is this a seed particle?
        let is_seed = i < debris_config.seed_count;
        let seed_index = if is_seed { Some(i) } else { None };

        // Lifetime (seeds live longer)
        let lifetime = if is_seed {
            debris_config.max_lifetime * 2.0
        } else {
            rng.gen_range(debris_config.min_lifetime..debris_config.max_lifetime)
        };

        // Initial color (white hot)
        let material = materials.add(StandardMaterial {
            base_color: Color::WHITE,
            emissive: LinearRgba::new(1.0, 0.98, 0.95, 1.0),
            unlit: true,
            alpha_mode: AlphaMode::Add,
            ..default()
        });

        commands.spawn((
            PbrBundle {
                mesh: mesh.clone(),
                material,
                transform: Transform::from_translation(Vec3::ZERO)
                    .with_scale(Vec3::splat(if is_seed { 0.1 } else { 0.05 })),
                ..default()
            },
            DebrisParticle {
                velocity,
                age: 0.0,
                lifetime,
                is_seed,
                seed_index,
            },
            Name::new(if is_seed {
                format!("Seed Particle {}", i)
            } else {
                format!("Debris {}", i)
            }),
        ));
    }

    state.spawned = true;
    state.spawn_time = elapsed;

    info!(target: "lightwatch::bang", "Spawned {} debris particles at {:.2}s", debris_config.count, elapsed);
}

/// Update debris particles
pub fn update_debris(
    mut commands: Commands,
    time: Res<Time>,
    config: Res<DebrisConfig>,
    mut particles: Query<(
        Entity,
        &mut DebrisParticle,
        &mut Transform,
        &Handle<StandardMaterial>,
    )>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let dt = time.delta_seconds();

    for (entity, mut particle, mut transform, material_handle) in particles.iter_mut() {
        // Update age
        particle.age += dt;

        // Despawn if expired (except seeds)
        if particle.age >= particle.lifetime && !particle.is_seed {
            commands.entity(entity).despawn();
            continue;
        }

        // Apply velocity with decay
        particle.velocity *= config.velocity_decay;
        transform.translation += particle.velocity * dt;

        // Calculate temperature (cools over time)
        let life_ratio = particle.age / particle.lifetime;
        let temperature = (1.0 - life_ratio * 0.8).max(0.2);

        // Color from temperature
        let color = debris_temperature_color(temperature);

        // Opacity fade
        let opacity = if particle.is_seed {
            // Seeds pulse instead of fading
            0.8 + 0.2 * (particle.age * 3.0).sin()
        } else {
            (1.0 - life_ratio).powf(0.5)
        };

        // Scale shrinks (seeds maintain size)
        let scale = if particle.is_seed {
            0.1
        } else {
            0.05 * (1.0 - life_ratio * 0.5)
        };
        transform.scale = Vec3::splat(scale);

        // Update material
        if let Some(material) = materials.get_mut(material_handle) {
            material.emissive = LinearRgba::new(
                color.x * opacity * 2.0,
                color.y * opacity * 2.0,
                color.z * opacity * 2.0,
                1.0,
            );
            material.base_color = Color::srgba(color.x, color.y, color.z, opacity);
        }
    }
}

/// Convert temperature (0-1) to color
fn debris_temperature_color(temp: f32) -> Vec3 {
    let white = Vec3::new(1.0, 0.98, 0.95);
    let amber = Vec3::new(0.91, 0.64, 0.27);
    let red = Vec3::new(0.7, 0.2, 0.1);
    let dark = Vec3::new(0.2, 0.05, 0.02);

    if temp > 0.7 {
        let t = (temp - 0.7) / 0.3;
        white.lerp(amber, 1.0 - t)
    } else if temp > 0.3 {
        let t = (temp - 0.3) / 0.4;
        amber.lerp(red, 1.0 - t)
    } else {
        let t = temp / 0.3;
        red.lerp(dark, 1.0 - t)
    }
}

/// Transform seeds into traveler spawn markers
pub fn transform_seeds_to_travelers(
    mut commands: Commands,
    clock: Res<ExperienceClock>,
    bang_config: Res<BangConfig>,
    seeds: Query<(Entity, &DebrisParticle, &Transform)>,
) {
    let elapsed = clock.elapsed();

    // Transform after bang settles
    if elapsed < bang_config.settle_time {
        return;
    }

    for (entity, particle, transform) in seeds.iter() {
        if !particle.is_seed {
            continue;
        }

        // Map seed index to traveler
        let traveler_id = match particle.seed_index {
            Some(0) => TravelerId::Archivist,
            Some(1) => TravelerId::Wanderer,
            Some(2) => TravelerId::Keeper,
            Some(3) => TravelerId::Child,
            Some(4) => TravelerId::Other,
            _ => continue,
        };

        info!(target: "lightwatch::bang", "Seed transforms to {:?}", traveler_id);

        // Mark position for traveler spawn
        commands.spawn((
            TransformBundle::from_transform(*transform),
            TravelerSpawnMarker { id: traveler_id },
            Name::new(format!("Traveler Spawn {:?}", traveler_id)),
        ));

        // Remove seed
        commands.entity(entity).despawn();
    }
}

/// Reset debris state
pub fn reset_debris_state(clock: Res<ExperienceClock>, mut state: ResMut<DebrisState>) {
    if clock.elapsed() < 1.0 && state.spawned {
        state.spawned = false;
    }
}

/// Debris plugin
pub struct DebrisPlugin;

impl Plugin for DebrisPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DebrisConfig>()
            .init_resource::<DebrisState>()
            .add_systems(
                Update,
                (
                    spawn_debris,
                    update_debris,
                    transform_seeds_to_travelers,
                    reset_debris_state,
                ),
            );
    }
}
