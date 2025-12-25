//! Audio event system - triggers for bang rumble, grief, transitions
//!
//! This module listens to game events and sends triggers to the audio thread.
//! The actual sound generation happens in output.rs on the audio thread.

use bevy::prelude::*;

use super::output::{AudioTrigger, AudioTriggerQueue};
use super::silence::SilenceManager;
use crate::core::{BangEvent, BangStage, PhaseChangedEvent, TravelerId, TravelerFadedEvent};

/// Event sound configuration (for reference/future tuning)
#[derive(Resource)]
#[allow(dead_code)]
pub struct EventSoundConfig {
    /// Bang rumble duration
    pub bang_duration: f32,
    /// Bang rumble base frequency
    pub bang_frequency: f32,
    /// Grief dissonance duration
    pub grief_duration: f32,
    /// Grief frequencies (dissonant cluster)
    pub grief_frequencies: [f32; 3],
    /// Silence fade duration
    pub silence_fade: f32,
}

impl Default for EventSoundConfig {
    fn default() -> Self {
        Self {
            bang_duration: 6.0,
            bang_frequency: 30.0, // Sub-bass
            grief_duration: 4.0,
            grief_frequencies: [220.0, 233.0, 247.0], // A3, Bb3, B3 - close cluster
            silence_fade: 2.0,
        }
    }
}

/// Handle bang events - sends trigger to audio thread
pub fn handle_bang_events(
    mut events: EventReader<BangEvent>,
    trigger_queue: Res<AudioTriggerQueue>,
) {
    for event in events.read() {
        if event.stage == BangStage::Expansion {
            trigger_queue.send(AudioTrigger::BangRumble);
            info!(target: "lightwatch::audio", "Bang rumble triggered");
        }
    }
}

/// Handle traveler faded events (grief for Child)
pub fn handle_traveler_faded(
    mut events: EventReader<TravelerFadedEvent>,
    trigger_queue: Res<AudioTriggerQueue>,
) {
    for event in events.read() {
        if event.id == TravelerId::Child {
            trigger_queue.send(AudioTrigger::GriefDissonance);
            info!(target: "lightwatch::audio", "Grief dissonance triggered for Child");
        }
    }
}

/// Handle phase transitions
pub fn handle_phase_transitions(
    mut events: EventReader<PhaseChangedEvent>,
    trigger_queue: Res<AudioTriggerQueue>,
) {
    for event in events.read() {
        trigger_queue.send(AudioTrigger::PhaseTransition(event.to));
        info!(target: "lightwatch::audio", "Phase transition sound: {:?}", event.to);
    }
}

/// Update silence manager
pub fn update_silence(time: Res<Time>, mut silence: ResMut<SilenceManager>) {
    let dt = time.delta_seconds();
    silence.update(dt);
}

/// Track if ambiance fade has been triggered
#[derive(Resource, Default)]
pub struct AmbianceFadeState {
    pub triggered: bool,
}

/// Fade ambiance at experience end (139s+)
pub fn fade_ambiance_at_end(
    clock: Res<crate::core::ExperienceClock>,
    trigger_queue: Res<AudioTriggerQueue>,
    mut fade_state: ResMut<AmbianceFadeState>,
) {
    // Fade starts at 139s (Ended phase), takes 4 seconds
    if !fade_state.triggered && clock.elapsed() >= 139.0 {
        trigger_queue.send(AudioTrigger::FadeAmbiance { duration: 4.0 });
        fade_state.triggered = true;
        info!(target: "lightwatch::audio", "Ambiance fade started");
    }
}

/// Event sound plugin
pub struct EventSoundPlugin;

impl Plugin for EventSoundPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EventSoundConfig>()
            .init_resource::<SilenceManager>()
            .init_resource::<AmbianceFadeState>()
            .add_systems(
                Update,
                (
                    handle_bang_events,
                    handle_traveler_faded,
                    handle_phase_transitions,
                    update_silence,
                    fade_ambiance_at_end,
                ),
            );
    }
}
