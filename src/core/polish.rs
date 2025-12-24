//! Final polish systems - Smooth fades at experience end

use bevy::prelude::*;
use bevy::render::camera::ClearColorConfig;

use crate::camera::ExperienceCamera;

use super::ExperienceClock;

/// Track fade state
#[derive(Resource, Default)]
pub struct FadeState {
    /// Whether visual fade has started
    pub visual_fade_started: bool,
    /// Final fade opacity
    pub fade_opacity: f32,
}

/// Fade visuals to black at experience end
pub fn fade_visuals_at_end(
    clock: Res<ExperienceClock>,
    mut fade_state: ResMut<FadeState>,
    mut cameras: Query<&mut Camera, With<ExperienceCamera>>,
) {
    let elapsed = clock.elapsed();

    // Start fade at 141 seconds
    if elapsed >= 141.0 {
        let fade = ((elapsed - 141.0) / 2.0).clamp(0.0, 1.0);
        fade_state.fade_opacity = fade;

        if !fade_state.visual_fade_started && fade > 0.0 {
            fade_state.visual_fade_started = true;
            info!(target: "lightwatch::polish", "Visual fade to black started");
        }

        for mut camera in cameras.iter_mut() {
            // Fade to black by darkening the clear color
            let dark = 1.0 - fade;
            camera.clear_color = ClearColorConfig::Custom(Color::srgb(
                0.0 * dark,
                0.0 * dark,
                0.0 * dark,
            ));
        }
    }
}

/// Log when experience is about to end
pub fn log_experience_ending(
    clock: Res<ExperienceClock>,
    mut logged: Local<bool>,
) {
    let elapsed = clock.elapsed();

    if elapsed >= 140.0 && !*logged {
        *logged = true;
        info!(target: "lightwatch::polish", "Experience ending in 3 seconds...");
    }
}
