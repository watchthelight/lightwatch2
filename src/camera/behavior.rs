//! Camera behavior system - drift, approach, pullback modes

#![allow(dead_code)]

use bevy::prelude::*;

use super::CameraRig;
use crate::core::easing::{ease_in_out_cubic, smooth_lerp_factor};
use crate::core::{CameraBehavior, CameraBehaviorChangedEvent};
use crate::wide_event;

/// Current camera behavior state
#[derive(Resource)]
pub struct CameraBehaviorState {
    /// Current active behavior
    pub current: CameraBehavior,
    /// Previous behavior (for transitions)
    pub previous: CameraBehavior,
    /// Transition progress (0.0 = previous, 1.0 = current)
    pub transition: f32,
    /// Transition speed (per second)
    pub transition_speed: f32,
    /// Time in current behavior
    pub time_in_behavior: f32,
    /// Behavior-specific parameters
    pub params: BehaviorParams,
}

impl Default for CameraBehaviorState {
    fn default() -> Self {
        Self {
            current: CameraBehavior::Static,
            previous: CameraBehavior::Static,
            transition: 1.0,
            transition_speed: 0.5,
            time_in_behavior: 0.0,
            params: BehaviorParams::default(),
        }
    }
}

/// Parameters specific to each behavior
#[derive(Clone)]
pub struct BehaviorParams {
    // Drift
    pub drift_speed: f32,
    pub drift_amplitude: f32,
    pub drift_direction: Vec2,

    // Approach
    pub approach_target: Vec3,
    pub approach_speed: f32,
    pub approach_min_distance: f32,

    // Pullback
    pub pullback_speed: f32,
    pub pullback_max_distance: f32,
}

impl Default for BehaviorParams {
    fn default() -> Self {
        Self {
            drift_speed: 0.02,
            drift_amplitude: 2.0,
            drift_direction: Vec2::new(1.0, 0.3),

            approach_target: Vec3::ZERO,
            approach_speed: 0.5,
            approach_min_distance: 8.0,

            pullback_speed: 0.3,
            pullback_max_distance: 50.0,
        }
    }
}

/// Handle behavior change events
pub fn handle_behavior_changes(
    mut state: ResMut<CameraBehaviorState>,
    mut events: EventReader<CameraBehaviorChangedEvent>,
) {
    for event in events.read() {
        if state.current != event.to {
            state.previous = state.current;
            state.current = event.to;
            state.transition = 0.0;
            state.time_in_behavior = 0.0;

            wide_event!("camera_behavior_changed")
                .with_str("from", format!("{:?}", state.previous))
                .with_str("to", format!("{:?}", state.current))
                .emit(event.elapsed);
        }
    }
}

/// Update behavior transition
pub fn update_behavior_transition(time: Res<Time>, mut state: ResMut<CameraBehaviorState>) {
    // Update transition progress
    if state.transition < 1.0 {
        state.transition += time.delta_seconds() * state.transition_speed;
        state.transition = state.transition.min(1.0);
    }

    // Update time in behavior
    state.time_in_behavior += time.delta_seconds();
}

/// Apply drift behavior
pub fn apply_drift_behavior(
    time: Res<Time>,
    state: Res<CameraBehaviorState>,
    mut rigs: Query<&mut CameraRig>,
) {
    if state.current != CameraBehavior::Drift && state.previous != CameraBehavior::Drift {
        return;
    }

    let t = state.time_in_behavior;
    let params = &state.params;

    // Calculate drift offset
    let drift_x =
        (t * params.drift_speed).sin() * params.drift_amplitude * params.drift_direction.x;
    let drift_y =
        (t * params.drift_speed * 0.7).cos() * params.drift_amplitude * params.drift_direction.y
            * 0.5;

    let target_offset = Vec3::new(drift_x, drift_y, 0.0);

    // Frame-rate independent smoothing factor
    let smooth_factor = smooth_lerp_factor(time.delta_seconds(), 6.0);

    for mut rig in rigs.iter_mut() {
        // Blend based on transition
        let blend = if state.current == CameraBehavior::Drift {
            ease_in_out_cubic(state.transition)
        } else {
            1.0 - ease_in_out_cubic(state.transition)
        };

        rig.behavior_offset = rig.behavior_offset.lerp(target_offset * blend, smooth_factor);
    }
}

/// Apply approach behavior
pub fn apply_approach_behavior(
    time: Res<Time>,
    state: Res<CameraBehaviorState>,
    mut rigs: Query<&mut CameraRig>,
) {
    if state.current != CameraBehavior::Approach && state.previous != CameraBehavior::Approach {
        return;
    }

    let params = &state.params;

    for mut rig in rigs.iter_mut() {
        let current_distance = rig.base_position.length();

        if current_distance > params.approach_min_distance {
            // Move closer
            let direction = -rig.base_position.normalize();
            let movement = direction * params.approach_speed * time.delta_seconds();

            // Calculate blend: fade in when current, fade out when previous
            let blend = if state.current == CameraBehavior::Approach {
                ease_in_out_cubic(state.transition)
            } else {
                // Fading out from approach
                1.0 - ease_in_out_cubic(state.transition)
            };

            rig.base_position += movement * blend;

            // Clamp to minimum distance
            if rig.base_position.length() < params.approach_min_distance {
                rig.base_position = rig.base_position.normalize() * params.approach_min_distance;
            }
        }
    }
}

/// Apply pullback behavior
pub fn apply_pullback_behavior(
    time: Res<Time>,
    state: Res<CameraBehaviorState>,
    mut rigs: Query<&mut CameraRig>,
) {
    if state.current != CameraBehavior::Pullback && state.previous != CameraBehavior::Pullback {
        return;
    }

    let params = &state.params;

    for mut rig in rigs.iter_mut() {
        let current_distance = rig.base_position.length();

        if current_distance < params.pullback_max_distance {
            // Move away
            let direction = rig.base_position.normalize();
            let movement = direction * params.pullback_speed * time.delta_seconds();

            // Calculate blend: fade in when current, fade out when previous
            let blend = if state.current == CameraBehavior::Pullback {
                ease_in_out_cubic(state.transition)
            } else {
                // Fading out from pullback
                1.0 - ease_in_out_cubic(state.transition)
            };

            rig.base_position += movement * blend;
        }
    }
}

/// Reset behavior offset when returning to static
pub fn reset_static_behavior(
    time: Res<Time>,
    state: Res<CameraBehaviorState>,
    mut rigs: Query<&mut CameraRig>,
) {
    if state.current != CameraBehavior::Static {
        return;
    }

    for mut rig in rigs.iter_mut() {
        // Smoothly return to zero
        rig.behavior_offset = rig.behavior_offset.lerp(Vec3::ZERO, time.delta_seconds() * 2.0);
    }
}

