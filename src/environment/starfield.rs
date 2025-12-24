//! Starfield - 2000 stars with twinkling and phase-driven reveal/fade

use bevy::prelude::*;
use rand::Rng;

use crate::core::{ExperienceClock, Phase};

/// Individual star
#[derive(Component, Debug)]
pub struct Star {
    /// Base brightness (0-1)
    pub brightness: f32,
    /// Twinkle frequency
    pub twinkle_freq: f32,
    /// Twinkle phase offset
    pub phase_offset: f32,
    /// Distance from origin
    pub distance: f32,
    /// Has been revealed post-bang?
    pub revealed: bool,
    /// Current opacity
    pub opacity: f32,
}

/// Marker component for star entities
#[derive(Component)]
pub struct StarMarker;

/// Starfield configuration
#[derive(Resource)]
pub struct StarfieldConfig {
    pub star_count: usize,
    pub min_distance: f32,
    pub max_distance: f32,
    pub base_brightness: f32,
    pub reveal_start_time: f32,
    pub reveal_duration: f32,
}

impl Default for StarfieldConfig {
    fn default() -> Self {
        Self {
            star_count: 2000,
            min_distance: 50.0,
            max_distance: 200.0,
            base_brightness: 0.8,
            reveal_start_time: 8.0,  // After bang settles
            reveal_duration: 10.0,   // Gradual reveal
        }
    }
}

/// Cached star mesh and material
#[derive(Resource)]
pub struct StarfieldAssets {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

/// Spawn the starfield
pub fn spawn_starfield(
    mut commands: Commands,
    config: Res<StarfieldConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::thread_rng();

    // Star mesh (tiny sphere)
    let star_mesh = meshes.add(Sphere::new(0.1).mesh().ico(0).unwrap());

    // Star material (emissive white)
    let star_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        emissive: LinearRgba::new(1.0, 0.98, 0.9, 1.0),
        unlit: true,
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    // Cache assets
    commands.insert_resource(StarfieldAssets {
        mesh: star_mesh.clone(),
        material: star_material.clone(),
    });

    for i in 0..config.star_count {
        // Spherical distribution using Fibonacci sphere
        let golden_ratio = (1.0 + 5.0_f32.sqrt()) / 2.0;
        let theta = 2.0 * std::f32::consts::PI * i as f32 / golden_ratio;
        let phi = (1.0 - 2.0 * (i as f32 + 0.5) / config.star_count as f32).acos();

        // Random distance within range
        let distance =
            config.min_distance + rng.gen::<f32>() * (config.max_distance - config.min_distance);

        let position = Vec3::new(
            distance * phi.sin() * theta.cos(),
            distance * phi.sin() * theta.sin(),
            distance * phi.cos(),
        );

        // Vary star properties
        let brightness = config.base_brightness * (0.3 + rng.gen::<f32>() * 0.7);
        let twinkle_freq = 0.5 + rng.gen::<f32>() * 2.0;
        let phase_offset = rng.gen::<f32>() * std::f32::consts::TAU;

        commands.spawn((
            PbrBundle {
                mesh: star_mesh.clone(),
                material: star_material.clone(),
                transform: Transform::from_translation(position).with_scale(Vec3::ZERO), // Start invisible
                visibility: Visibility::Hidden,
                ..default()
            },
            Star {
                brightness,
                twinkle_freq,
                phase_offset,
                distance,
                revealed: false,
                opacity: 0.0,
            },
            StarMarker,
        ));
    }

    info!(target: "lightwatch::environment", "Spawned {} stars", config.star_count);
}

/// Reveal stars gradually after bang
pub fn reveal_stars(
    clock: Res<ExperienceClock>,
    config: Res<StarfieldConfig>,
    mut stars: Query<(&mut Star, &mut Visibility), With<StarMarker>>,
) {
    let elapsed = clock.elapsed();

    // Before reveal time
    if elapsed < config.reveal_start_time {
        return;
    }

    // Calculate reveal progress
    let reveal_elapsed = elapsed - config.reveal_start_time;
    let reveal_progress = (reveal_elapsed / config.reveal_duration).min(1.0);

    for (mut star, mut visibility) in stars.iter_mut() {
        if star.revealed {
            continue;
        }

        // Reveal based on distance (closer stars reveal first)
        let distance_factor =
            (star.distance - config.min_distance) / (config.max_distance - config.min_distance);

        if reveal_progress >= distance_factor {
            star.revealed = true;
            *visibility = Visibility::Visible;
            star.opacity = 0.0;
        }
    }
}

/// Fade in revealed stars
pub fn fade_in_stars(time: Res<Time>, mut stars: Query<&mut Star, With<StarMarker>>) {
    for mut star in stars.iter_mut() {
        if star.revealed && star.opacity < 1.0 {
            star.opacity = (star.opacity + time.delta_seconds() * 0.5).min(1.0);
        }
    }
}

/// Update star twinkle and opacity
pub fn update_stars(
    time: Res<Time>,
    mut stars: Query<(&Star, &mut Transform, &Handle<StandardMaterial>), With<StarMarker>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let t = time.elapsed_seconds();

    for (star, mut transform, material_handle) in stars.iter_mut() {
        if !star.revealed {
            continue;
        }

        // Twinkle calculation
        let twinkle = (t * star.twinkle_freq + star.phase_offset).sin() * 0.5 + 0.5;
        let effective_brightness = star.brightness * (0.5 + twinkle * 0.5) * star.opacity;

        // Update scale based on brightness
        let scale = 0.05 + effective_brightness * 0.1;
        transform.scale = Vec3::splat(scale.max(0.001));

        // Update material emissive
        if let Some(material) = materials.get_mut(material_handle) {
            let color_temp = 0.9 + star.brightness * 0.1; // Warmer = brighter
            material.emissive = LinearRgba::new(
                effective_brightness,
                effective_brightness * color_temp,
                effective_brightness * color_temp * 0.9,
                1.0,
            );
            material.base_color = Color::srgba(1.0, 1.0, 1.0, star.opacity);
        }
    }
}

/// Fade stars during Acceptance
pub fn fade_stars_during_acceptance(
    clock: Res<ExperienceClock>,
    mut stars: Query<&mut Star, With<StarMarker>>,
) {
    if clock.phase() != Phase::Acceptance {
        return;
    }

    let progress = clock.phase_progress();

    // Start fading at 50% through Acceptance
    if progress > 0.5 {
        let fade_progress = (progress - 0.5) * 2.0; // 0 to 1 in second half

        for mut star in stars.iter_mut() {
            if star.revealed {
                star.opacity = 1.0 - fade_progress;
            }
        }
    }
}

/// Starfield plugin
pub struct StarfieldPlugin;

impl Plugin for StarfieldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarfieldConfig>().add_systems(
            Startup,
            spawn_starfield,
        ).add_systems(
            Update,
            (
                reveal_stars,
                fade_in_stars,
                update_stars,
                fade_stars_during_acceptance,
            ),
        );
    }
}
