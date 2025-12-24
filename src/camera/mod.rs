//! Camera systems: Rig, Behaviors, DOF, Shake, Transitions

use bevy::prelude::*;

mod breathing;
mod config;
mod rig;

pub use breathing::*;
pub use config::*;
pub use rig::*;

/// Camera plugin for cinematic camera control
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BreathingConfig>()
            .init_resource::<CameraConfig>()
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, (update_breathing, apply_rig_to_transform.after(update_breathing)));

        // TODO: Camera behaviors (drift, approach, pullback)
        // TODO: Depth of field
        // TODO: Trauma-based shake
        // TODO: Cinematic transitions
    }
}
