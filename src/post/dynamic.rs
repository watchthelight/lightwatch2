//! Dynamic post-processing state

use bevy::prelude::*;

use crate::core::easing::{ease_in_out_cubic, smooth_step};
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
    let base = config.chromatic_aberration.base_intensity;
    let max = config.chromatic_aberration.max_intensity;

    // Increase CA during bang (3-6 seconds) with smooth easing
    let ca_intensity = if elapsed >= 3.0 && elapsed < 6.0 {
        if elapsed < 4.0 {
            // Building to peak with smooth easing
            let t = ease_in_out_cubic((elapsed - 3.0) / 1.0);
            base + t * (max - base)
        } else {
            // Decaying from peak with smooth easing
            let t = ease_in_out_cubic((elapsed - 4.0) / 2.0);
            max - t * (max - base)
        }
    } else if elapsed >= 2.5 && elapsed < 3.0 {
        // Smooth ramp-in before bang
        let t = smooth_step((elapsed - 2.5) / 0.5);
        base * (1.0 + t * 0.5)
    } else if elapsed >= 6.0 && elapsed < 6.5 {
        // Smooth ramp-out after bang
        let t = smooth_step((elapsed - 6.0) / 0.5);
        base * (1.5 - t * 0.5)
    } else {
        base
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
    let base = config.grain.base_intensity;

    // Increase grain at start and end for "old film" feel with smooth transitions
    let grain_intensity = if elapsed < 2.0 {
        // Start - more grain (coming into focus), smooth fade out
        let t = smooth_step(elapsed / 2.0);
        base * (1.5 - t * 0.5)
    } else if elapsed > 133.0 {
        // End - more grain (fading out), smooth fade in
        let t = smooth_step((elapsed - 133.0) / 10.0);
        base * (1.0 + t * 0.5)
    } else {
        base
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

    let base = config.vignette.base_intensity;

    // Smooth pulse at phase transitions using entry/exit factors
    let entry_factor = clock.phase_entry_factor();
    let exit_factor = clock.phase_exit_factor();

    // Boost vignette slightly during phase transitions (smooth blend)
    let entry_boost = (1.0 - entry_factor) * 0.1;  // Fades out as we enter
    let exit_boost = (1.0 - exit_factor) * 0.1;   // Builds up as we exit

    dynamic.vignette_intensity = base + entry_boost + exit_boost;
}
