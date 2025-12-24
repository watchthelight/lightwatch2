//! Camera cinematic transitions - keyframed paths with easing

use bevy::prelude::*;

use super::CameraRig;
use crate::core::{Phase, PhaseChangedEvent};
use crate::wide_event;

/// A camera transition between two states
#[derive(Clone)]
pub struct CameraTransition {
    /// Starting position
    pub from_position: Vec3,
    /// Ending position
    pub to_position: Vec3,
    /// Starting rotation
    pub from_rotation: Quat,
    /// Ending rotation
    pub to_rotation: Quat,
    /// Duration in seconds
    pub duration: f32,
    /// Easing function
    pub easing: EasingFunction,
    /// Optional look-at target
    pub look_at: Option<Vec3>,
}

impl Default for CameraTransition {
    fn default() -> Self {
        Self {
            from_position: Vec3::new(0.0, 0.0, 15.0),
            to_position: Vec3::new(0.0, 0.0, 10.0),
            from_rotation: Quat::IDENTITY,
            to_rotation: Quat::IDENTITY,
            duration: 3.0,
            easing: EasingFunction::EaseInOutCubic,
            look_at: Some(Vec3::ZERO),
        }
    }
}

/// Easing functions for smooth transitions
#[derive(Clone, Copy, Debug, Default)]
pub enum EasingFunction {
    Linear,
    EaseIn,
    EaseOut,
    #[default]
    EaseInOutCubic,
    EaseInOutQuart,
    EaseOutExpo,
    EaseInExpo,
}

impl EasingFunction {
    pub fn apply(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            EasingFunction::Linear => t,
            EasingFunction::EaseIn => t * t * t,
            EasingFunction::EaseOut => 1.0 - (1.0 - t).powi(3),
            EasingFunction::EaseInOutCubic => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
                }
            }
            EasingFunction::EaseInOutQuart => {
                if t < 0.5 {
                    8.0 * t * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(4) / 2.0
                }
            }
            EasingFunction::EaseOutExpo => {
                if t >= 1.0 {
                    1.0
                } else {
                    1.0 - 2.0_f32.powf(-10.0 * t)
                }
            }
            EasingFunction::EaseInExpo => {
                if t <= 0.0 {
                    0.0
                } else {
                    2.0_f32.powf(10.0 * t - 10.0)
                }
            }
        }
    }
}

/// Currently active transition
#[derive(Resource)]
pub struct ActiveTransition {
    /// The transition being executed
    pub transition: Option<CameraTransition>,
    /// Progress (0.0 - 1.0)
    pub progress: f32,
    /// Time elapsed in transition
    pub elapsed: f32,
}

impl Default for ActiveTransition {
    fn default() -> Self {
        Self {
            transition: None,
            progress: 0.0,
            elapsed: 0.0,
        }
    }
}

impl ActiveTransition {
    /// Start a new transition
    pub fn start(&mut self, transition: CameraTransition) {
        self.transition = Some(transition);
        self.progress = 0.0;
        self.elapsed = 0.0;
    }

    /// Is a transition active?
    pub fn is_active(&self) -> bool {
        self.transition.is_some() && self.progress < 1.0
    }

    /// Cancel current transition
    pub fn cancel(&mut self) {
        self.transition = None;
        self.progress = 0.0;
        self.elapsed = 0.0;
    }
}

/// Predefined camera transitions for LIGHTWATCH phases
pub struct TransitionPresets;

impl TransitionPresets {
    /// Awakening: camera begins to drift
    pub fn awakening_drift() -> CameraTransition {
        CameraTransition {
            from_position: Vec3::new(0.0, 0.0, 15.0),
            to_position: Vec3::new(2.0, 0.5, 14.0),
            duration: 8.0,
            easing: EasingFunction::EaseInOutCubic,
            look_at: Some(Vec3::ZERO),
            ..default()
        }
    }

    /// Discovery: approaching the travelers
    pub fn discovery_approach() -> CameraTransition {
        CameraTransition {
            from_position: Vec3::new(2.0, 0.5, 14.0),
            to_position: Vec3::new(0.0, 0.0, 10.0),
            duration: 20.0,
            easing: EasingFunction::EaseInOutQuart,
            look_at: Some(Vec3::ZERO),
            ..default()
        }
    }

    /// Connection: settle in close
    pub fn connection_settle() -> CameraTransition {
        CameraTransition {
            from_position: Vec3::new(0.0, 0.0, 10.0),
            to_position: Vec3::new(0.0, -0.5, 8.0),
            duration: 10.0,
            easing: EasingFunction::EaseOutExpo,
            look_at: Some(Vec3::ZERO),
            ..default()
        }
    }

    /// Acceptance: pull back slowly
    pub fn acceptance_pullback() -> CameraTransition {
        CameraTransition {
            from_position: Vec3::new(0.0, -0.5, 8.0),
            to_position: Vec3::new(0.0, 0.0, 40.0),
            duration: 50.0, // Very slow
            easing: EasingFunction::EaseInOutCubic,
            look_at: Some(Vec3::ZERO),
            ..default()
        }
    }

    /// Get transition for phase
    pub fn for_phase(phase: Phase) -> Option<CameraTransition> {
        match phase {
            Phase::Awakening => Some(Self::awakening_drift()),
            Phase::Discovery => Some(Self::discovery_approach()),
            Phase::Connection => Some(Self::connection_settle()),
            Phase::Acceptance => Some(Self::acceptance_pullback()),
            _ => None,
        }
    }
}

/// Event to manually trigger a transition
#[derive(Event)]
pub struct TriggerTransitionEvent {
    pub transition: CameraTransition,
}

/// Start transitions on phase change
pub fn start_phase_transitions(
    clock: Res<crate::core::ExperienceClock>,
    mut active: ResMut<ActiveTransition>,
    mut events: EventReader<PhaseChangedEvent>,
) {
    for event in events.read() {
        if let Some(transition) = TransitionPresets::for_phase(event.to) {
            let duration = transition.duration;
            active.start(transition);

            wide_event!("camera_transition_started")
                .with_str("phase", event.to.name())
                .with_f32("duration", duration)
                .emit(clock.elapsed());
        }
    }
}

/// Handle manual transition triggers
pub fn handle_transition_triggers(
    mut active: ResMut<ActiveTransition>,
    mut events: EventReader<TriggerTransitionEvent>,
) {
    for event in events.read() {
        active.start(event.transition.clone());
    }
}

/// Update active transition
pub fn update_transition(
    time: Res<Time>,
    clock: Res<crate::core::ExperienceClock>,
    mut active: ResMut<ActiveTransition>,
    mut rigs: Query<&mut CameraRig>,
) {
    // Extract transition data to avoid borrow conflicts
    let Some(ref transition) = active.transition else {
        return;
    };

    if active.progress >= 1.0 {
        return;
    }

    // Copy values we need
    let duration = transition.duration;
    let easing = transition.easing;
    let from_pos = transition.from_position;
    let to_pos = transition.to_position;
    let from_rot = transition.from_rotation;
    let to_rot = transition.to_rotation;
    let use_look_at = transition.look_at.is_some();

    // Update time and progress
    active.elapsed += time.delta_seconds();
    active.progress = (active.elapsed / duration).min(1.0);

    // Apply easing
    let eased_t = easing.apply(active.progress);

    for mut rig in rigs.iter_mut() {
        // Interpolate position
        rig.base_position = from_pos.lerp(to_pos, eased_t);

        // Interpolate rotation (if not using look_at)
        if !use_look_at {
            rig.base_rotation = from_rot.slerp(to_rot, eased_t);
        }
    }

    // Log completion
    if active.progress >= 1.0 {
        wide_event!("camera_transition_completed").emit(clock.elapsed());
    }
}
