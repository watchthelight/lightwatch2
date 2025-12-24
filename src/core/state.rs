//! Experience state machine - Loading → Ready → Running → Ending → Ended

#![allow(dead_code)]

use super::clock::{ExperienceClock, Phase};
use bevy::prelude::*;

/// High-level experience states
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ExperienceState {
    /// Assets loading, systems initializing
    #[default]
    Loading,
    /// Ready to start, waiting for click
    Ready,
    /// Experience is running (143 seconds)
    Running,
    /// Experience complete, fading to black
    Ending,
    /// Fully ended
    Ended,
}

impl ExperienceState {
    pub fn name(&self) -> &'static str {
        match self {
            ExperienceState::Loading => "loading",
            ExperienceState::Ready => "ready",
            ExperienceState::Running => "running",
            ExperienceState::Ending => "ending",
            ExperienceState::Ended => "ended",
        }
    }

    /// Can we transition to the given state?
    pub fn can_transition_to(&self, target: &ExperienceState) -> bool {
        matches!(
            (self, target),
            (ExperienceState::Loading, ExperienceState::Ready)
                | (ExperienceState::Ready, ExperienceState::Running)
                | (ExperienceState::Running, ExperienceState::Ending)
                | (ExperienceState::Ending, ExperienceState::Ended)
        )
    }
}

/// Event for state changes
#[derive(Event, Debug, Clone)]
pub struct StateChangedEvent {
    pub from: ExperienceState,
    pub to: ExperienceState,
}

/// Check if loading is complete and transition to Ready
pub fn check_loading_complete(
    state: Res<State<ExperienceState>>,
    mut next_state: ResMut<NextState<ExperienceState>>,
) {
    if *state.get() != ExperienceState::Loading {
        return;
    }

    // For now, immediately transition (assets are embedded)
    next_state.set(ExperienceState::Ready);

    crate::wide_event!("loading_complete").emit(0.0);
}

/// Transition to Ending when clock finishes
pub fn check_experience_end(
    state: Res<State<ExperienceState>>,
    mut next_state: ResMut<NextState<ExperienceState>>,
    clock: Res<ExperienceClock>,
) {
    if *state.get() != ExperienceState::Running {
        return;
    }

    if clock.phase() == Phase::Ended {
        next_state.set(ExperienceState::Ending);

        crate::wide_event!("experience_ending").emit(clock.elapsed());
    }
}

/// Timer for ending fade-out
#[derive(Resource)]
pub struct EndingTimer(pub Timer);

impl Default for EndingTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(3.0, TimerMode::Once))
    }
}

/// Handle the ending fade-out
pub fn handle_ending_phase(
    state: Res<State<ExperienceState>>,
    mut next_state: ResMut<NextState<ExperienceState>>,
    mut timer: ResMut<EndingTimer>,
    time: Res<Time>,
) {
    if *state.get() != ExperienceState::Ending {
        return;
    }

    timer.0.tick(time.delta());

    if timer.0.finished() {
        next_state.set(ExperienceState::Ended);

        crate::wide_event!("experience_ended").emit(143.0);
    }
}

/// Log state transitions
pub fn log_state_transitions(
    state: Res<State<ExperienceState>>,
    mut events: EventWriter<StateChangedEvent>,
    mut last_state: Local<Option<ExperienceState>>,
) {
    let current = *state.get();

    if *last_state != Some(current) {
        if let Some(from) = *last_state {
            events.send(StateChangedEvent { from, to: current });

            crate::wide_event!("state_changed")
                .with_str("from", from.name())
                .with_str("to", current.name())
                .emit(0.0);
        }
        *last_state = Some(current);
    }
}

/// System set that runs only when Ready (pre-start)
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReadySet;

/// System set that runs only during experience
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct RunningSet;

/// System set that runs during ending fade
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct EndingSet;

/// Run condition: only in Ready state
pub fn in_ready_state(state: Res<State<ExperienceState>>) -> bool {
    *state.get() == ExperienceState::Ready
}

/// Run condition: only in Running state
pub fn in_running_state(state: Res<State<ExperienceState>>) -> bool {
    *state.get() == ExperienceState::Running
}

/// Run condition: in Running or Ending state
pub fn experience_active(state: Res<State<ExperienceState>>) -> bool {
    matches!(
        *state.get(),
        ExperienceState::Running | ExperienceState::Ending
    )
}

/// Run condition: experience has started
pub fn experience_started(state: Res<State<ExperienceState>>) -> bool {
    !matches!(
        *state.get(),
        ExperienceState::Loading | ExperienceState::Ready
    )
}
