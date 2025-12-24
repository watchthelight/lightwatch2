//! Phase controller - orchestrates moment-by-moment actions

#![allow(dead_code)]

use bevy::prelude::*;

use super::events::*;
use super::ExperienceClock;
use crate::wide_event;

/// A scheduled moment in the experience
#[derive(Debug, Clone)]
pub struct Moment {
    /// Time in seconds when this moment triggers
    pub time: f32,
    /// Name for logging
    pub name: &'static str,
    /// The action to take
    pub action: MomentAction,
    /// Has this moment been triggered?
    pub triggered: bool,
}

/// Actions that can happen at a moment
#[derive(Debug, Clone)]
pub enum MomentAction {
    // Traveler actions
    SpawnTraveler(TravelerId),
    FadeTraveler(TravelerId),

    // Camera actions
    SetCameraBehavior(CameraBehavior),
    CameraShake { intensity: f32, duration: f32 },
    FocusOn(Option<TravelerId>),

    // Audio actions
    PlayNote { freq: f32, dur: f32, vol: f32 },
    StartLayer(String),
    StopLayer(String),
    PlayLeitmotif(String),

    // Narrative actions
    ShowText { text: String, traveler: Option<TravelerId> },
    HideText,
    SignalTypewriter(String),

    // Visual actions
    BangStage(BangStage),
    Glitch { intensity: f32, duration: f32 },
    EnvironmentChange { layer: String, action: String },

    // Meta actions
    TriggerGrief { mourner: TravelerId, deceased: TravelerId },
    SyncTravelers(Vec<TravelerId>),
}

impl Moment {
    pub fn new(time: f32, name: &'static str, action: MomentAction) -> Self {
        Self {
            time,
            name,
            action,
            triggered: false,
        }
    }
}

/// All scheduled moments for the experience
pub fn create_moment_schedule() -> Vec<Moment> {
    vec![
        // ====================================================================
        // SIGNAL PHASE (0-2s)
        // ====================================================================
        Moment::new(0.0, "signal_detected", MomentAction::SignalTypewriter("signal detected".into())),
        Moment::new(0.5, "source_info", MomentAction::SignalTypewriter("source: unknown".into())),
        Moment::new(1.0, "age_info", MomentAction::SignalTypewriter("age: 14.3 billion years".into())),

        // ====================================================================
        // BANG PHASE (2-12s)
        // ====================================================================
        Moment::new(2.0, "bang_start", MomentAction::BangStage(BangStage::Start)),
        Moment::new(2.0, "hide_signal", MomentAction::HideText),
        Moment::new(2.5, "light_point", MomentAction::BangStage(BangStage::LightPoint)),
        Moment::new(3.0, "expansion", MomentAction::BangStage(BangStage::Expansion)),
        Moment::new(4.0, "bang_peak", MomentAction::BangStage(BangStage::Peak)),
        Moment::new(4.0, "peak_glitch", MomentAction::Glitch { intensity: 0.8, duration: 0.2 }),
        Moment::new(4.0, "peak_shake", MomentAction::CameraShake { intensity: 0.4, duration: 0.5 }),
        Moment::new(6.0, "settling", MomentAction::BangStage(BangStage::Settling)),
        Moment::new(9.5, "bang_complete", MomentAction::BangStage(BangStage::Complete)),
        Moment::new(10.0, "start_radiation", MomentAction::StartLayer("radiation".into())),

        // ====================================================================
        // AWAKENING PHASE (12-27s)
        // ====================================================================
        Moment::new(12.0, "archivist_spawns", MomentAction::SpawnTraveler(TravelerId::Archivist)),
        Moment::new(12.0, "camera_drift", MomentAction::SetCameraBehavior(CameraBehavior::Drift)),
        Moment::new(15.0, "archivist_text", MomentAction::ShowText {
            text: "we built these for you".into(),
            traveler: Some(TravelerId::Archivist),
        }),
        Moment::new(18.0, "wanderer_spawns", MomentAction::SpawnTraveler(TravelerId::Wanderer)),
        Moment::new(22.0, "keeper_spawns", MomentAction::SpawnTraveler(TravelerId::Keeper)),
        Moment::new(25.0, "keeper_text", MomentAction::ShowText {
            text: "we listened for so long".into(),
            traveler: Some(TravelerId::Keeper),
        }),

        // ====================================================================
        // DISCOVERY PHASE (27-57s)
        // ====================================================================
        Moment::new(27.0, "camera_approach", MomentAction::SetCameraBehavior(CameraBehavior::Approach)),
        Moment::new(30.0, "child_spawns", MomentAction::SpawnTraveler(TravelerId::Child)),
        Moment::new(32.0, "child_text", MomentAction::ShowText {
            text: "here here here".into(),
            traveler: Some(TravelerId::Child),
        }),
        Moment::new(40.0, "wanderer_text", MomentAction::ShowText {
            text: "i found them again / every time".into(),
            traveler: Some(TravelerId::Wanderer),
        }),
        Moment::new(45.0, "other_spawns", MomentAction::SpawnTraveler(TravelerId::Other)),
        Moment::new(50.0, "archivist_text_2", MomentAction::ShowText {
            text: "come closer".into(),
            traveler: Some(TravelerId::Archivist),
        }),

        // ====================================================================
        // CONNECTION PHASE (57-87s)
        // ====================================================================
        Moment::new(57.0, "camera_still", MomentAction::SetCameraBehavior(CameraBehavior::Static)),
        Moment::new(60.0, "first_sync", MomentAction::SyncTravelers(vec![
            TravelerId::Archivist, TravelerId::Keeper
        ])),
        Moment::new(65.0, "leitmotif_primary", MomentAction::PlayLeitmotif("primary".into())),
        Moment::new(70.0, "full_sync", MomentAction::SyncTravelers(vec![
            TravelerId::Archivist, TravelerId::Wanderer, TravelerId::Keeper, TravelerId::Child
        ])),
        Moment::new(75.0, "keeper_text_2", MomentAction::ShowText {
            text: "this was enough".into(),
            traveler: Some(TravelerId::Keeper),
        }),
        Moment::new(80.0, "thank_you", MomentAction::ShowText {
            text: "thank you".into(),
            traveler: None, // collective
        }),

        // ====================================================================
        // ACCEPTANCE PHASE (87-143s)
        // ====================================================================
        Moment::new(87.0, "camera_pullback", MomentAction::SetCameraBehavior(CameraBehavior::Pullback)),
        Moment::new(90.0, "stop_radiation", MomentAction::StopLayer("radiation".into())),
        Moment::new(95.0, "child_fades", MomentAction::FadeTraveler(TravelerId::Child)),
        Moment::new(95.5, "grief_shake", MomentAction::CameraShake { intensity: 0.2, duration: 1.0 }),
        Moment::new(96.0, "archivist_grief", MomentAction::TriggerGrief {
            mourner: TravelerId::Archivist,
            deceased: TravelerId::Child,
        }),
        Moment::new(105.0, "wanderer_fades", MomentAction::FadeTraveler(TravelerId::Wanderer)),
        Moment::new(112.0, "keeper_fades", MomentAction::FadeTraveler(TravelerId::Keeper)),
        Moment::new(120.0, "archivist_fades", MomentAction::FadeTraveler(TravelerId::Archivist)),
        Moment::new(125.0, "other_responds", MomentAction::ShowText {
            text: "...".into(),
            traveler: Some(TravelerId::Other),
        }),
        Moment::new(130.0, "other_fades", MomentAction::FadeTraveler(TravelerId::Other)),
        Moment::new(135.0, "final_pulse", MomentAction::PlayLeitmotif("final_pulse".into())),
        Moment::new(139.0, "silence", MomentAction::StopLayer("all".into())),
        Moment::new(141.0, "end_signal", MomentAction::SignalTypewriter("end of signal".into())),
    ]
}

/// The phase controller resource
#[derive(Resource)]
pub struct PhaseController {
    pub moments: Vec<Moment>,
}

impl Default for PhaseController {
    fn default() -> Self {
        Self {
            moments: create_moment_schedule(),
        }
    }
}

/// Check and trigger scheduled moments
#[allow(clippy::too_many_arguments)]
pub fn process_moments(
    clock: Res<ExperienceClock>,
    mut controller: ResMut<PhaseController>,
    mut moment_events: EventWriter<MomentEvent>,
    mut traveler_spawn_events: EventWriter<TravelerSpawnedEvent>,
    mut traveler_fade_events: EventWriter<TravelerFadingEvent>,
    mut camera_behavior_events: EventWriter<CameraBehaviorChangedEvent>,
    mut camera_shake_events: EventWriter<CameraShakeEvent>,
    mut bang_events: EventWriter<BangEvent>,
    mut glitch_events: EventWriter<GlitchEvent>,
    mut text_events: EventWriter<DisplayTextEvent>,
    mut hide_text_events: EventWriter<HideTextEvent>,
    mut signal_events: EventWriter<SignalOverlayEvent>,
    mut audio_events: EventWriter<AudioLayerEvent>,
    mut leitmotif_events: EventWriter<PlayLeitmotifEvent>,
    mut grief_events: EventWriter<TravelerGriefEvent>,
    mut sync_events: EventWriter<TravelersSyncedEvent>,
) {
    let elapsed = clock.elapsed();

    for moment in controller.moments.iter_mut() {
        // Skip already triggered
        if moment.triggered {
            continue;
        }

        // Check if it's time
        if elapsed >= moment.time {
            moment.triggered = true;

            // Log the moment
            wide_event!("moment_triggered")
                .with_str("name", moment.name)
                .with_f32("scheduled_at", moment.time)
                .emit(elapsed);

            // Fire MomentEvent
            moment_events.send(MomentEvent {
                name: moment.name.to_string(),
                elapsed,
                phase: clock.phase(),
            });

            // Dispatch to appropriate event based on action
            dispatch_moment_action(
                &moment.action,
                elapsed,
                &mut traveler_spawn_events,
                &mut traveler_fade_events,
                &mut camera_behavior_events,
                &mut camera_shake_events,
                &mut bang_events,
                &mut glitch_events,
                &mut text_events,
                &mut hide_text_events,
                &mut signal_events,
                &mut audio_events,
                &mut leitmotif_events,
                &mut grief_events,
                &mut sync_events,
            );
        }
    }
}

/// Dispatch a moment action to the appropriate event
#[allow(clippy::too_many_arguments)]
fn dispatch_moment_action(
    action: &MomentAction,
    elapsed: f32,
    traveler_spawn_events: &mut EventWriter<TravelerSpawnedEvent>,
    traveler_fade_events: &mut EventWriter<TravelerFadingEvent>,
    camera_behavior_events: &mut EventWriter<CameraBehaviorChangedEvent>,
    camera_shake_events: &mut EventWriter<CameraShakeEvent>,
    bang_events: &mut EventWriter<BangEvent>,
    glitch_events: &mut EventWriter<GlitchEvent>,
    text_events: &mut EventWriter<DisplayTextEvent>,
    hide_text_events: &mut EventWriter<HideTextEvent>,
    signal_events: &mut EventWriter<SignalOverlayEvent>,
    audio_events: &mut EventWriter<AudioLayerEvent>,
    leitmotif_events: &mut EventWriter<PlayLeitmotifEvent>,
    grief_events: &mut EventWriter<TravelerGriefEvent>,
    sync_events: &mut EventWriter<TravelersSyncedEvent>,
) {
    match action {
        MomentAction::SpawnTraveler(id) => {
            traveler_spawn_events.send(TravelerSpawnedEvent {
                id: *id,
                position: Vec3::ZERO, // Position determined by spawn system
                elapsed,
            });
        }

        MomentAction::FadeTraveler(id) => {
            traveler_fade_events.send(TravelerFadingEvent {
                id: *id,
                elapsed,
            });
        }

        MomentAction::SetCameraBehavior(behavior) => {
            camera_behavior_events.send(CameraBehaviorChangedEvent {
                from: CameraBehavior::Static, // Will be tracked by camera system
                to: *behavior,
                elapsed,
            });
        }

        MomentAction::CameraShake { intensity, duration } => {
            camera_shake_events.send(CameraShakeEvent {
                intensity: *intensity,
                duration: *duration,
                elapsed,
            });
        }

        MomentAction::FocusOn(target) => {
            // CameraFocusEvent - not yet wired, but action exists
            let _ = target; // Placeholder until camera system implements
        }

        MomentAction::PlayNote { freq, dur, vol } => {
            // PlayNoteEvent - not yet wired
            let _ = (freq, dur, vol); // Placeholder until audio system implements
        }

        MomentAction::BangStage(stage) => {
            bang_events.send(BangEvent {
                stage: *stage,
                elapsed,
            });
        }

        MomentAction::Glitch { intensity, duration } => {
            glitch_events.send(GlitchEvent {
                intensity: *intensity,
                duration: *duration,
                elapsed,
            });
        }

        MomentAction::ShowText { text, traveler } => {
            text_events.send(DisplayTextEvent {
                text: text.clone(),
                traveler: *traveler,
                style: NarrativeTextStyle::Normal,
                elapsed,
            });
        }

        MomentAction::HideText => {
            hide_text_events.send(HideTextEvent);
        }

        MomentAction::SignalTypewriter(text) => {
            signal_events.send(SignalOverlayEvent {
                action: SignalAction::Typewriter(text.clone()),
                elapsed,
            });
        }

        MomentAction::StartLayer(layer) => {
            audio_events.send(AudioLayerEvent {
                layer: layer.clone(),
                action: AudioAction::Start,
                elapsed,
            });
        }

        MomentAction::StopLayer(layer) => {
            audio_events.send(AudioLayerEvent {
                layer: layer.clone(),
                action: AudioAction::Stop,
                elapsed,
            });
        }

        MomentAction::PlayLeitmotif(motif) => {
            leitmotif_events.send(PlayLeitmotifEvent {
                motif: motif.clone(),
                traveler: None,
                elapsed,
            });
        }

        MomentAction::TriggerGrief { mourner, deceased } => {
            grief_events.send(TravelerGriefEvent {
                mourner: *mourner,
                deceased: *deceased,
                elapsed,
            });
        }

        MomentAction::SyncTravelers(travelers) => {
            sync_events.send(TravelersSyncedEvent {
                participants: travelers.clone(),
                elapsed,
            });
        }

        MomentAction::EnvironmentChange { layer, action } => {
            // EnvironmentEvent - not yet wired
            let _ = (layer, action); // Placeholder
        }
    }
}

/// Reset controller when experience restarts (dev only)
#[cfg(debug_assertions)]
pub fn reset_controller_on_restart(
    mut controller: ResMut<PhaseController>,
    clock: Res<ExperienceClock>,
) {
    // If clock jumped backwards, reset triggered moments
    for moment in controller.moments.iter_mut() {
        if moment.triggered && clock.elapsed() < moment.time {
            moment.triggered = false;
        }
    }
}

/// Phase controller plugin
pub struct PhaseControllerPlugin;

impl Plugin for PhaseControllerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PhaseController>()
            .add_systems(Update, process_moments.in_set(super::RunningSet));

        #[cfg(debug_assertions)]
        app.add_systems(Update, reset_controller_on_restart);
    }
}
