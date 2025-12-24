//! Camera systems: Rig, Behaviors, DOF, Shake, Transitions

use bevy::prelude::*;

mod behavior;
mod breathing;
mod config;
mod dof;
mod rig;
mod shake;
mod transitions;

pub use behavior::*;
pub use breathing::*;
pub use config::*;
pub use dof::*;
pub use rig::*;
pub use shake::*;
pub use transitions::*;

/// Camera plugin for cinematic camera control
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BreathingConfig>()
            .init_resource::<CameraConfig>()
            .init_resource::<CameraBehaviorState>()
            .init_resource::<DepthOfFieldSettings>()
            .init_resource::<CameraShake>()
            .init_resource::<ActiveTransition>()
            .add_event::<TriggerTransitionEvent>()
            .add_systems(Startup, spawn_camera)
            .add_systems(
                Update,
                (
                    // Breathing (always runs)
                    update_breathing,
                    // Transition systems (takes priority)
                    start_phase_transitions,
                    handle_transition_triggers,
                    update_transition,
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
                    // Shake systems
                    handle_shake_events,
                    update_shake,
                    apply_shake_to_rig,
                    // Final application
                    apply_rig_to_transform
                        .after(update_breathing)
                        .after(update_transition)
                        .after(apply_drift_behavior)
                        .after(apply_approach_behavior)
                        .after(apply_pullback_behavior)
                        .after(reset_static_behavior)
                        .after(apply_shake_to_rig),
                ),
            );
    }
}
