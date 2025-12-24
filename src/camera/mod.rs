//! Camera systems: Rig, Behaviors, DOF, Shake, Transitions

use bevy::prelude::*;

mod behavior;
mod breathing;
mod config;
mod dof;
mod rig;

pub use behavior::*;
pub use breathing::*;
pub use config::*;
pub use dof::*;
pub use rig::*;

/// Camera plugin for cinematic camera control
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BreathingConfig>()
            .init_resource::<CameraConfig>()
            .init_resource::<CameraBehaviorState>()
            .init_resource::<DepthOfFieldSettings>()
            .add_systems(Startup, spawn_camera)
            .add_systems(
                Update,
                (
                    // Breathing (always runs)
                    update_breathing,
                    // Behavior handling
                    handle_behavior_changes,
                    update_behavior_transition,
                    // Behavior-specific systems
                    apply_drift_behavior,
                    apply_approach_behavior,
                    apply_pullback_behavior,
                    reset_static_behavior,
                    // DOF systems
                    update_dof_for_phase,
                    handle_focus_events,
                    interpolate_focus,
                    // Final application
                    apply_rig_to_transform
                        .after(update_breathing)
                        .after(apply_drift_behavior)
                        .after(apply_approach_behavior)
                        .after(apply_pullback_behavior)
                        .after(reset_static_behavior),
                ),
            );

        // TODO: Trauma-based shake
        // TODO: Cinematic transitions
        // TODO: DOF render node (post-processing integration)
    }
}
