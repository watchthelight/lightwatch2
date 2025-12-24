//! Bang core light - Central explosion point

use bevy::prelude::*;

use crate::core::ExperienceClock;

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

/// Spawn bang core geometry
pub fn spawn_bang_core(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Core sphere
    let mesh = meshes.add(Sphere::new(0.1).mesh().ico(3).unwrap());

    let material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        emissive: LinearRgba::new(0.0, 0.0, 0.0, 1.0), // Start dark
        unlit: true,
        alpha_mode: AlphaMode::Add,
        ..default()
    });

    commands.spawn((
        PbrBundle {
            mesh,
            material,
            transform: Transform::from_scale(Vec3::ZERO), // Start invisible
            visibility: Visibility::Hidden,
            ..default()
        },
        BangCore::default(),
        Name::new("Bang Core"),
    ));

    info!(target: "lightwatch::bang", "Spawned bang core");
}

/// Update bang core based on timeline
pub fn update_bang_core(
    clock: Res<ExperienceClock>,
    config: Res<BangConfig>,
    mut cores: Query<(
        &mut BangCore,
        &mut Transform,
        &mut Visibility,
        &Handle<StandardMaterial>,
    )>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let elapsed = clock.elapsed();

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

        // Apply to material
        if let Some(material) = materials.get_mut(material_handle) {
            let color = temperature_to_color(core.temperature);
            material.emissive = LinearRgba::new(
                color.x * core.intensity * 2.0,
                color.y * core.intensity * 2.0,
                color.z * core.intensity * 2.0,
                1.0,
            );
            material.base_color =
                Color::srgba(color.x, color.y, color.z, core.intensity.min(1.0));
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

/// Convert temperature (0-1) to color
fn temperature_to_color(temp: f32) -> Vec3 {
    // White -> Amber -> Red
    let white = Vec3::new(1.0, 0.98, 0.95);
    let amber = Vec3::new(0.91, 0.64, 0.27);
    let red = Vec3::new(0.7, 0.2, 0.1);

    if temp > 0.5 {
        let t = (temp - 0.5) * 2.0;
        white.lerp(amber, 1.0 - t)
    } else {
        let t = temp * 2.0;
        amber.lerp(red, 1.0 - t)
    }
}

/// Bang core plugin
pub struct BangCorePlugin;

impl Plugin for BangCorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BangConfig>()
            .add_systems(Startup, spawn_bang_core)
            .add_systems(Update, update_bang_core);
    }
}
