//! Nebula dust particles - 10,000 particles at varied depths

use bevy::prelude::*;
use rand::Rng;

use crate::core::{ExperienceClock, Phase};

/// Individual dust particle
#[derive(Component, Debug)]
pub struct DustParticle {
    /// Depth layer (0 = close, 1 = far)
    pub depth: f32,
    /// Base opacity
    pub opacity: f32,
    /// Drift velocity
    pub velocity: Vec3,
    /// Seed for noise
    pub noise_seed: f32,
}

/// Marker for dust particles
#[derive(Component)]
pub struct DustMarker;

/// Marker for dust layer (for culling optimization)
#[derive(Component)]
pub struct DustLayer(pub u8);

/// Dust system configuration
#[derive(Resource)]
pub struct DustConfig {
    pub particle_count: usize,
    pub min_depth: f32,
    pub max_depth: f32,
    pub drift_speed: f32,
    pub spawn_radius: f32,
    pub intensity: f32,
}

impl Default for DustConfig {
    fn default() -> Self {
        Self {
            particle_count: 10000,
            min_depth: 20.0,
            max_depth: 150.0,
            drift_speed: 0.02,
            spawn_radius: 100.0,
            intensity: 0.5,
        }
    }
}

/// Cached dust assets
#[derive(Resource)]
pub struct DustAssets {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

/// Spawn dust particles
pub fn spawn_dust_particles(
    mut commands: Commands,
    config: Res<DustConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::thread_rng();

    // Shared mesh (tiny quad)
    let dust_mesh = meshes.add(Rectangle::new(0.1, 0.1));

    // Shared material
    let dust_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.9, 0.85, 0.8, 0.3),
        emissive: LinearRgba::new(0.2, 0.18, 0.15, 1.0),
        unlit: true,
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    // Cache assets
    commands.insert_resource(DustAssets {
        mesh: dust_mesh.clone(),
        material: dust_material.clone(),
    });

    // Spawn particles
    for _ in 0..config.particle_count {
        // Spherical distribution
        let theta = rng.gen::<f32>() * std::f32::consts::TAU;
        let phi = rng.gen::<f32>() * std::f32::consts::PI;
        let depth = config.min_depth + rng.gen::<f32>() * (config.max_depth - config.min_depth);

        let position = Vec3::new(
            phi.sin() * theta.cos() * depth,
            phi.sin() * theta.sin() * depth * 0.5, // Flatten vertically
            phi.cos() * depth,
        );

        // Opacity decreases with distance
        let distance_factor =
            1.0 - (depth - config.min_depth) / (config.max_depth - config.min_depth);
        let opacity = 0.1 + distance_factor * 0.3;

        // Random drift direction
        let velocity = Vec3::new(
            (rng.gen::<f32>() - 0.5) * config.drift_speed,
            (rng.gen::<f32>() - 0.5) * config.drift_speed * 0.3,
            (rng.gen::<f32>() - 0.5) * config.drift_speed,
        );

        // Size varies with depth (perspective)
        let size = 0.05 + distance_factor * 0.15;

        // Depth layer (0-3)
        let layer = ((depth - config.min_depth) / (config.max_depth - config.min_depth) * 4.0)
            .floor() as u8;

        commands.spawn((
            PbrBundle {
                mesh: dust_mesh.clone(),
                material: dust_material.clone(),
                transform: Transform::from_translation(position).with_scale(Vec3::splat(size)),
                visibility: Visibility::Hidden, // Start hidden
                ..default()
            },
            DustParticle {
                depth: depth / config.max_depth, // Normalized
                opacity,
                velocity,
                noise_seed: rng.gen::<f32>() * 1000.0,
            },
            DustLayer(layer),
            DustMarker,
        ));
    }

    info!(target: "lightwatch::environment", "Spawned {} dust particles", config.particle_count);
}

/// Animate dust particles
pub fn animate_dust(
    time: Res<Time>,
    config: Res<DustConfig>,
    mut particles: Query<(&DustParticle, &mut Transform), With<DustMarker>>,
) {
    let t = time.elapsed_seconds();

    for (dust, mut transform) in particles.iter_mut() {
        // Base drift
        let drift = dust.velocity * t;

        // Add noise-based motion
        let noise_offset = Vec3::new(
            ((t * 0.1 + dust.noise_seed) * 0.5).sin() * 2.0,
            ((t * 0.08 + dust.noise_seed + 100.0) * 0.5).sin() * 1.0,
            ((t * 0.12 + dust.noise_seed + 200.0) * 0.5).cos() * 2.0,
        );

        // Apply drift with noise
        transform.translation += (drift + noise_offset * 0.01) * time.delta_seconds();

        // Wrap around when too far
        if transform.translation.length() > config.spawn_radius * 1.5 {
            transform.translation = -transform.translation.normalize() * config.min_depth;
        }
    }
}

/// Control dust visibility by phase
pub fn update_dust_visibility(
    clock: Res<ExperienceClock>,
    config: Res<DustConfig>,
    mut particles: Query<(&DustParticle, &mut Visibility, &Handle<StandardMaterial>), With<DustMarker>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let phase = clock.phase();
    let progress = clock.phase_progress();

    let intensity = match phase {
        Phase::Signal | Phase::Bang => 0.0,
        Phase::Awakening => progress * config.intensity, // Fade in
        Phase::Discovery => config.intensity,
        Phase::Connection => config.intensity * 0.7,
        Phase::Acceptance => config.intensity * (1.0 - progress), // Fade out
        Phase::Ended => 0.0,
    };

    for (dust, mut visibility, material_handle) in particles.iter_mut() {
        if intensity < 0.01 {
            *visibility = Visibility::Hidden;
        } else {
            *visibility = Visibility::Visible;

            // Update material opacity
            if let Some(material) = materials.get_mut(material_handle) {
                let alpha = dust.opacity * intensity;
                material.base_color = Color::srgba(0.9, 0.85, 0.8, alpha);
            }
        }
    }
}

/// Dust plugin
pub struct DustPlugin;

impl Plugin for DustPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DustConfig>()
            .add_systems(Startup, spawn_dust_particles)
            .add_systems(Update, (animate_dust, update_dust_visibility));
    }
}
