//! Bloom post-processing

use bevy::core_pipeline::bloom::BloomSettings;
use bevy::prelude::*;

use crate::bang::BangConfig;
use crate::camera::ExperienceCamera;
use crate::core::ExperienceClock;

use super::PostProcessConfig;

/// Update bloom based on experience phase and bang intensity
pub fn update_bloom_for_bang(
    clock: Res<ExperienceClock>,
    bang_config: Res<BangConfig>,
    post_config: Res<PostProcessConfig>,
    mut cameras: Query<&mut BloomSettings, With<ExperienceCamera>>,
) {
    if !post_config.bloom.enabled {
        return;
    }

    let elapsed = clock.elapsed();

    // Calculate bloom boost during bang
    let boost = if elapsed >= bang_config.expansion_start && elapsed < bang_config.settle_time {
        let t = (elapsed - bang_config.expansion_start)
            / (bang_config.peak_time - bang_config.expansion_start);
        if elapsed < bang_config.peak_time {
            // Building to peak
            t.clamp(0.0, 1.0) * 0.5
        } else {
            // Decaying from peak
            let decay =
                (elapsed - bang_config.peak_time) / (bang_config.settle_time - bang_config.peak_time);
            0.5 * (1.0 - decay.clamp(0.0, 1.0))
        }
    } else {
        0.0
    };

    for mut bloom in cameras.iter_mut() {
        bloom.intensity = post_config.bloom.base_intensity + boost;
    }
}
