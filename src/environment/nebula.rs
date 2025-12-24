//! Nebula - raymarched volumetric background

use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

use crate::core::{ExperienceClock, Phase};

/// Nebula rendering configuration
#[derive(Resource)]
pub struct NebulaConfig {
    /// Overall intensity (0-1)
    pub intensity: f32,
    /// Drift speed
    pub drift_speed: f32,
    /// Primary color (warm amber)
    pub color1: Color,
    /// Secondary color (cool violet)
    pub color2: Color,
    /// Noise scale
    pub noise_scale: f32,
}

impl Default for NebulaConfig {
    fn default() -> Self {
        Self {
            intensity: 0.6,
            drift_speed: 0.02,
            color1: Color::srgb(0.91, 0.64, 0.27), // Amber
            color2: Color::srgb(0.42, 0.36, 0.58), // Violet
            noise_scale: 0.015,
        }
    }
}

/// Nebula shader material
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct NebulaMaterial {
    #[uniform(0)]
    pub color1: LinearRgba,
    #[uniform(0)]
    pub color2: LinearRgba,
    #[uniform(0)]
    pub time: f32,
    #[uniform(0)]
    pub intensity: f32,
    #[uniform(0)]
    pub drift_speed: f32,
    #[uniform(0)]
    pub noise_scale: f32,
}

impl Default for NebulaMaterial {
    fn default() -> Self {
        Self {
            color1: LinearRgba::new(0.91, 0.64, 0.27, 1.0),
            color2: LinearRgba::new(0.42, 0.36, 0.58, 1.0),
            time: 0.0,
            intensity: 0.6,
            drift_speed: 0.02,
            noise_scale: 0.015,
        }
    }
}

impl Material for NebulaMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/nebula.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Opaque
    }
}

/// Marker for nebula background entity
#[derive(Component)]
pub struct NebulaBackground;

/// Spawn nebula background quad
pub fn spawn_nebula_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<NebulaMaterial>>,
) {
    // Create large background quad
    let mesh = meshes.add(Rectangle::new(2000.0, 2000.0));

    let material = materials.add(NebulaMaterial::default());

    // Spawn far behind everything
    commands.spawn((
        MaterialMeshBundle {
            mesh,
            material,
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, -500.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        NebulaBackground,
        Name::new("Nebula Background"),
    ));

    info!(target: "lightwatch::environment", "Spawned nebula background");
}

/// Update nebula intensity based on phase
pub fn update_nebula_intensity(clock: Res<ExperienceClock>, mut config: ResMut<NebulaConfig>) {
    let base_intensity = 0.6;

    config.intensity = match clock.phase() {
        Phase::Signal => 0.0,
        Phase::Bang => {
            // Fade in during bang settling
            let progress = clock.phase_progress();
            if progress > 0.7 {
                (progress - 0.7) / 0.3 * base_intensity
            } else {
                0.0
            }
        }
        Phase::Awakening => base_intensity,
        Phase::Discovery => base_intensity * 1.2,
        Phase::Connection => base_intensity * 0.8,
        Phase::Acceptance => {
            // Fade out
            base_intensity * (1.0 - clock.phase_progress())
        }
        Phase::Ended => 0.0,
    };
}

/// Update nebula material uniforms
pub fn update_nebula_material(
    time: Res<Time>,
    config: Res<NebulaConfig>,
    mut materials: ResMut<Assets<NebulaMaterial>>,
) {
    for (_, material) in materials.iter_mut() {
        material.time = time.elapsed_seconds();
        material.intensity = config.intensity;
        material.drift_speed = config.drift_speed;
        material.color1 = config.color1.to_linear();
        material.color2 = config.color2.to_linear();
        material.noise_scale = config.noise_scale;
    }
}

/// Nebula plugin
pub struct NebulaPlugin;

impl Plugin for NebulaPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<NebulaMaterial>::default())
            .init_resource::<NebulaConfig>()
            .add_systems(Startup, spawn_nebula_background)
            .add_systems(Update, (update_nebula_intensity, update_nebula_material));
    }
}
