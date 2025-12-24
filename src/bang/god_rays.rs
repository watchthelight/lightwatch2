//! God rays - Screen-space radial blur from bang core

#![allow(dead_code)]

use bevy::prelude::*;

use super::{BangConfig, BangCore};
use crate::camera::ExperienceCamera;
use crate::core::ExperienceClock;

/// God ray effect configuration
#[derive(Resource)]
pub struct GodRayConfig {
    /// Effect intensity (0-1)
    pub intensity: f32,
    /// Decay per sample (0.9-0.99)
    pub decay: f32,
    /// Ray density
    pub density: f32,
    /// Number of samples
    pub samples: i32,
    /// Exposure multiplier
    pub exposure: f32,
}

impl Default for GodRayConfig {
    fn default() -> Self {
        Self {
            intensity: 0.0, // Starts off
            decay: 0.96,
            density: 0.8,
            samples: 50,
            exposure: 0.3,
        }
    }
}

/// Current god ray state
#[derive(Resource)]
pub struct GodRayState {
    /// Light source screen position (0-1)
    pub light_position: Vec2,
    /// Current intensity
    pub current_intensity: f32,
    /// Target intensity
    pub target_intensity: f32,
    /// Whether effect is active
    pub active: bool,
}

impl Default for GodRayState {
    fn default() -> Self {
        Self {
            light_position: Vec2::new(0.5, 0.5), // Center
            current_intensity: 0.0,
            target_intensity: 0.0,
            active: false,
        }
    }
}

/// Update god ray intensity based on bang timeline
pub fn update_god_ray_intensity(
    clock: Res<ExperienceClock>,
    bang_config: Res<BangConfig>,
    mut state: ResMut<GodRayState>,
) {
    let elapsed = clock.elapsed();

    // Calculate target intensity based on bang phase
    let target = if elapsed < bang_config.expansion_start {
        // Before expansion - no rays
        0.0
    } else if elapsed < bang_config.peak_time {
        // During expansion - ramp up
        let t = (elapsed - bang_config.expansion_start)
            / (bang_config.peak_time - bang_config.expansion_start);
        t * 1.0
    } else if elapsed < bang_config.settle_time {
        // Peak to settle - max intensity then decay
        let t =
            (elapsed - bang_config.peak_time) / (bang_config.settle_time - bang_config.peak_time);
        1.0 - t * 0.5
    } else if elapsed < bang_config.complete_time {
        // Settling - gradual fade
        let t = (elapsed - bang_config.settle_time)
            / (bang_config.complete_time - bang_config.settle_time);
        0.5 * (1.0 - t)
    } else {
        0.0
    };

    state.target_intensity = target;
    state.active = target > 0.01;
}

/// Interpolate god ray intensity
pub fn interpolate_god_rays(time: Res<Time>, mut state: ResMut<GodRayState>) {
    let speed = 3.0;
    state.current_intensity +=
        (state.target_intensity - state.current_intensity) * speed * time.delta_seconds();
}

/// Update light position from world to screen space
pub fn update_light_screen_position(
    bang_core: Query<&GlobalTransform, With<BangCore>>,
    camera: Query<(&Camera, &GlobalTransform), With<ExperienceCamera>>,
    mut state: ResMut<GodRayState>,
) {
    let Ok(core_transform) = bang_core.get_single() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera.get_single() else {
        return;
    };

    // Project world position to screen
    if let Some(screen_pos) = camera.world_to_ndc(camera_transform, core_transform.translation()) {
        // Convert NDC (-1 to 1) to UV (0 to 1)
        state.light_position = Vec2::new((screen_pos.x + 1.0) * 0.5, (screen_pos.y + 1.0) * 0.5);
    }
}

/// God rays plugin
/// Note: Full post-processing render node integration is deferred to prompt 39
pub struct GodRaysPlugin;

impl Plugin for GodRaysPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GodRayConfig>()
            .init_resource::<GodRayState>()
            .add_systems(
                Update,
                (
                    update_light_screen_position,
                    update_god_ray_intensity,
                    interpolate_god_rays,
                ),
            );

        info!(target: "lightwatch::bang", "God rays system initialized (render integration in post-processing)");
    }
}
