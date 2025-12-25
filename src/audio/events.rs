//! Audio event system - bang rumble, grief, transitions

use bevy::prelude::*;

use super::bang_sound::BangRumble;
use super::grief_sound::GriefDissonance;
use super::output::{AudioTrigger, AudioTriggerQueue};
use super::silence::SilenceManager;
use super::transitions::TransitionSound;
use crate::core::{BangEvent, BangStage, PhaseChangedEvent, TravelerId, TravelerFadedEvent};

/// Event sound configuration
#[derive(Resource)]
pub struct EventSoundConfig {
    /// Bang rumble duration
    #[allow(dead_code)]
    pub bang_duration: f32,
    /// Bang rumble base frequency
    #[allow(dead_code)]
    pub bang_frequency: f32,
    /// Grief dissonance duration
    #[allow(dead_code)]
    pub grief_duration: f32,
    /// Grief frequencies (dissonant cluster)
    #[allow(dead_code)]
    pub grief_frequencies: [f32; 3],
    /// Silence fade duration
    #[allow(dead_code)]
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

/// Event sound state
#[derive(Resource, Default)]
pub struct EventSounds {
    pub bang_rumble: BangRumble,
    pub grief: GriefDissonance,
    pub transitions: TransitionSound,
}

/// Handle bang events
pub fn handle_bang_events(
    mut events: EventReader<BangEvent>,
    mut sounds: ResMut<EventSounds>,
    trigger_queue: Res<AudioTriggerQueue>,
) {
    for event in events.read() {
        if event.stage == BangStage::Expansion {
            sounds.bang_rumble.trigger();
            trigger_queue.send(AudioTrigger::BangRumble);
            info!(target: "lightwatch::audio", "Bang rumble triggered");
        }
    }
}

/// Handle traveler faded events (grief for Child)
pub fn handle_traveler_faded(
    mut events: EventReader<TravelerFadedEvent>,
    mut sounds: ResMut<EventSounds>,
    trigger_queue: Res<AudioTriggerQueue>,
) {
    for event in events.read() {
        if event.id == TravelerId::Child {
            sounds.grief.trigger();
            trigger_queue.send(AudioTrigger::GriefDissonance);
            info!(target: "lightwatch::audio", "Grief dissonance triggered for Child");
        }
    }
}

/// Handle phase transitions
pub fn handle_phase_transitions(
    mut events: EventReader<PhaseChangedEvent>,
    mut sounds: ResMut<EventSounds>,
    trigger_queue: Res<AudioTriggerQueue>,
) {
    for event in events.read() {
        sounds.transitions.trigger_for_phase(event.to);
        trigger_queue.send(AudioTrigger::PhaseTransition(event.to));
        info!(target: "lightwatch::audio", "Phase transition sound: {:?}", event.to);
    }
}

/// Update event sounds
pub fn update_event_sounds(time: Res<Time>, mut silence: ResMut<SilenceManager>) {
    let dt = time.delta_seconds();
    silence.update(dt);
}

/// Event sound plugin
pub struct EventSoundPlugin;

impl Plugin for EventSoundPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EventSoundConfig>()
            .init_resource::<EventSounds>()
            .init_resource::<SilenceManager>()
            .add_systems(
                Update,
                (
                    handle_bang_events,
                    handle_traveler_faded,
                    handle_phase_transitions,
                    update_event_sounds,
                ),
            );
    }
}
