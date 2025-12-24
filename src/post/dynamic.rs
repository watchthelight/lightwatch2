//! Dynamic post-processing state

use bevy::prelude::*;

use crate::core::ExperienceClock;

use super::PostProcessConfig;

/// State for dynamic post-processing values
#[derive(Resource, Default)]
pub struct DynamicPostProcess {
    /// Current chromatic aberration intensity
    pub chromatic_intensity: f32,
    /// Current film grain intensity
    pub grain_intensity: f32,
    /// Current vignette intensity
    pub vignette_intensity: f32,
}

/// Update chromatic aberration based on experience
pub fn update_chromatic_aberration(
    clock: Res<ExperienceClock>,
    config: Res<PostProcessConfig>,
    mut dynamic: ResMut<DynamicPostProcess>,
) {
    if !config.chromatic_aberration.enabled {
        dynamic.chromatic_intensity = 0.0;
        return;
    }

    let elapsed = clock.elapsed();

    // Increase CA during bang (3-6 seconds)
    let ca_intensity = if elapsed >= 3.0 && elapsed < 6.0 {
        if elapsed < 4.0 {
            // Building to peak
            let t = (elapsed - 3.0) / 1.0;
            config.chromatic_aberration.base_intensity
                + t * (config.chromatic_aberration.max_intensity
                    - config.chromatic_aberration.base_intensity)
        } else {
            // Decaying from peak
            let t = (elapsed - 4.0) / 2.0;
            config.chromatic_aberration.max_intensity
                - t * (config.chromatic_aberration.max_intensity
                    - config.chromatic_aberration.base_intensity)
        }
    } else {
        config.chromatic_aberration.base_intensity
    };

    dynamic.chromatic_intensity = ca_intensity;
}

/// Update film grain based on experience
pub fn update_film_grain(
    clock: Res<ExperienceClock>,
    config: Res<PostProcessConfig>,
    mut dynamic: ResMut<DynamicPostProcess>,
) {
    if !config.grain.enabled {
        dynamic.grain_intensity = 0.0;
        return;
    }

    let elapsed = clock.elapsed();

    // Increase grain at start and end for "old film" feel
    let grain_intensity = if elapsed < 2.0 {
        // Start - more grain (coming into focus)
        config.grain.base_intensity * 1.5
    } else if elapsed > 135.0 {
        // End - more grain (fading out)
        let t = (elapsed - 135.0) / 8.0;
        config.grain.base_intensity * (1.0 + t.clamp(0.0, 1.0) * 0.5)
    } else {
        config.grain.base_intensity
    };

    dynamic.grain_intensity = grain_intensity;
}

/// Update vignette dynamically
pub fn update_vignette(
    clock: Res<ExperienceClock>,
    config: Res<PostProcessConfig>,
    mut dynamic: ResMut<DynamicPostProcess>,
) {
    if !config.vignette.enabled {
        dynamic.vignette_intensity = 0.0;
        return;
    }

    let phase_progress = clock.phase_progress();

    // Pulse vignette at phase transitions
    let transition_boost = if phase_progress < 0.1 || phase_progress > 0.9 {
        0.1
    } else {
        0.0
    };

    dynamic.vignette_intensity = config.vignette.base_intensity + transition_boost;
}
