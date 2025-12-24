//! Camera breathing motion - subtle organic movement

use bevy::prelude::*;

use super::CameraRig;

/// Breathing motion parameters
#[derive(Resource)]
pub struct BreathingConfig {
    /// Amplitude of vertical breathing motion
    pub amplitude_y: f32,
    /// Amplitude of horizontal sway
    pub amplitude_x: f32,
    /// Primary breathing frequency (Hz)
    pub frequency_primary: f32,
    /// Secondary frequency for organic feel
    pub frequency_secondary: f32,
    /// Phase offset for secondary motion
    pub phase_offset: f32,
}

impl Default for BreathingConfig {
    fn default() -> Self {
        Self {
            amplitude_y: 0.02,
            amplitude_x: 0.01,
            frequency_primary: 0.15,  // Very slow breath
            frequency_secondary: 0.08, // Even slower secondary
            phase_offset: 1.5,
        }
    }
}

/// Update breathing motion
pub fn update_breathing(time: Res<Time>, config: Res<BreathingConfig>, mut rigs: Query<&mut CameraRig>) {
    let t = time.elapsed_seconds();

    for mut rig in rigs.iter_mut() {
        // Primary breathing (vertical)
        let breath_y =
            (t * config.frequency_primary * std::f32::consts::TAU).sin() * config.amplitude_y;

        // Secondary sway (horizontal, slower)
        let sway_x = (t * config.frequency_secondary * std::f32::consts::TAU + config.phase_offset)
            .sin()
            * config.amplitude_x;

        // Very subtle forward/back
        let breath_z = (t * config.frequency_primary * std::f32::consts::TAU * 0.5).cos()
            * config.amplitude_y
            * 0.5;

        rig.breathing_offset = Vec3::new(sway_x, breath_y, breath_z);
    }
}
