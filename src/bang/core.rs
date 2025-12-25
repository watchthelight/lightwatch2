//! Bang core light - Central explosion point with custom shader

use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

use crate::core::ExperienceClock;

/// Custom bang core material
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct BangCoreMaterial {
    #[uniform(0)]
    pub time: f32,
    #[uniform(0)]
    pub intensity: f32,
    #[uniform(0)]
    pub temperature: f32,
    #[uniform(0)]
    pub expansion: f32,
    #[uniform(0)]
    pub color: LinearRgba,
}

impl Default for BangCoreMaterial {
    fn default() -> Self {
        Self {
            time: 0.0,
            intensity: 0.0,
            temperature: 1.0,
            expansion: 0.0,
            color: LinearRgba::WHITE,
        }
    }
}

impl Material for BangCoreMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/bang_core.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Add // Additive blending for glow effect
    }
}

/// Bang core light state
#[derive(Component, Debug)]
pub struct BangCore {
    /// Current intensity (0-1+, can exceed 1 for bloom)
    pub intensity: f32,
    /// Current expansion (0 = point, 1 = full)
    pub expansion: f32,
    /// Color temperature (1 = white-hot, 0 = cool red)
    pub temperature: f32,
}

impl Default for BangCore {
    fn default() -> Self {
        Self {
            intensity: 0.0,
            expansion: 0.0,
            temperature: 1.0,
        }
    }
}

/// Bang timeline configuration
#[derive(Resource)]
pub struct BangConfig {
    pub start_time: f32,
    pub light_point_time: f32,
    pub expansion_start: f32,
    pub peak_time: f32,
    pub settle_time: f32,
    pub complete_time: f32,
}

impl Default for BangConfig {
    fn default() -> Self {
        Self {
            start_time: 2.0,
            light_point_time: 2.5,
            expansion_start: 3.0,
            peak_time: 4.0,
            settle_time: 6.0,
            complete_time: 10.0,
        }
    }
}

/// Spawn bang core geometry with custom material
pub fn spawn_bang_core(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BangCoreMaterial>>,
) {
    // Core sphere - icosphere for smooth surface
    let mesh = meshes.add(Sphere::new(0.1).mesh().ico(3).unwrap());

    let material = materials.add(BangCoreMaterial::default());

    commands.spawn((
        MaterialMeshBundle {
            mesh,
            material,
            transform: Transform::from_scale(Vec3::ZERO), // Start invisible
            visibility: Visibility::Hidden,
            ..default()
        },
        BangCore::default(),
        Name::new("Bang Core"),
    ));

    info!(target: "lightwatch::bang", "Spawned bang core with custom shader");
}

/// Update bang core based on timeline
pub fn update_bang_core(
    clock: Res<ExperienceClock>,
    config: Res<BangConfig>,
    time: Res<Time>,
    mut cores: Query<(
        &mut BangCore,
        &mut Transform,
        &mut Visibility,
        &Handle<BangCoreMaterial>,
    )>,
    mut materials: ResMut<Assets<BangCoreMaterial>>,
) {
    let elapsed = clock.elapsed();
    let current_time = time.elapsed_seconds();

    for (mut core, mut transform, mut visibility, material_handle) in cores.iter_mut() {
        // Before start
        if elapsed < config.start_time {
            core.intensity = 0.0;
            *visibility = Visibility::Hidden;
            continue;
        }

        *visibility = Visibility::Visible;

        // Phase calculations
        if elapsed < config.light_point_time {
            // Point of light appears
            let t =
                (elapsed - config.start_time) / (config.light_point_time - config.start_time);
            core.intensity = t * 0.3;
            core.expansion = 0.0;
            core.temperature = 1.0;
        } else if elapsed < config.expansion_start {
            // Building intensity
            let t = (elapsed - config.light_point_time)
                / (config.expansion_start - config.light_point_time);
            core.intensity = 0.3 + t * 0.7;
            core.expansion = 0.0;
            core.temperature = 1.0;
        } else if elapsed < config.peak_time {
            // Explosive expansion
            let t =
                (elapsed - config.expansion_start) / (config.peak_time - config.expansion_start);
            let eased_t = ease_out_expo(t);
            core.intensity = 1.0 + eased_t * 2.0; // Exceed 1 for bloom
            core.expansion = eased_t;
            core.temperature = 1.0 - eased_t * 0.3; // Start cooling
        } else if elapsed < config.settle_time {
            // Deceleration and cooling
            let t = (elapsed - config.peak_time) / (config.settle_time - config.peak_time);
            core.intensity = 3.0 - t * 2.0;
            core.expansion = 1.0;
            core.temperature = 0.7 - t * 0.3;
        } else if elapsed < config.complete_time {
            // Settling to void
            let t = (elapsed - config.settle_time) / (config.complete_time - config.settle_time);
            core.intensity = 1.0 - t;
            core.expansion = 1.0;
            core.temperature = 0.4 - t * 0.4;
        } else {
            // Complete
            core.intensity = 0.0;
            *visibility = Visibility::Hidden;
        }

        // Apply to transform
        let scale = 0.1 + core.expansion * 50.0;
        transform.scale = Vec3::splat(scale);

        // Sync to custom material
        if let Some(material) = materials.get_mut(material_handle) {
            material.time = current_time;
            material.intensity = core.intensity;
            material.temperature = core.temperature;
            material.expansion = core.expansion;
            // Color is computed in shader based on temperature
            material.color = LinearRgba::new(1.0, 1.0, 1.0, core.intensity.min(1.0));
        }
    }
}

/// Exponential ease out
fn ease_out_expo(t: f32) -> f32 {
    if t >= 1.0 {
        1.0
    } else {
        1.0 - 2.0_f32.powf(-10.0 * t)
    }
}

/// Bang core plugin
pub struct BangCorePlugin;

impl Plugin for BangCorePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<BangCoreMaterial>::default())
            .init_resource::<BangConfig>()
            .add_systems(Startup, spawn_bang_core)
            .add_systems(Update, update_bang_core);
    }
}
