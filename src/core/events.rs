//! Event catalog - all event types for decoupled system communication

use super::Phase;
use bevy::prelude::*;

// ============================================================================
// PHASE EVENTS
// ============================================================================


/// Event fired at specific moments within phases
#[derive(Event, Debug, Clone)]
pub struct MomentEvent {
    pub name: String,
    pub elapsed: f32,
    pub phase: Phase,
}

// ============================================================================
// TRAVELER EVENTS
// ============================================================================

/// Traveler identity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TravelerId {
    Archivist,
    Wanderer,
    Keeper,
    Child,
    Other,
}

impl TravelerId {
    pub fn name(&self) -> &'static str {
        match self {
            TravelerId::Archivist => "archivist",
            TravelerId::Wanderer => "wanderer",
            TravelerId::Keeper => "keeper",
            TravelerId::Child => "child",
            TravelerId::Other => "other",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            TravelerId::Archivist => "THE ARCHIVIST",
            TravelerId::Wanderer => "THE WANDERER",
            TravelerId::Keeper => "THE KEEPER",
            TravelerId::Child => "THE CHILD",
            TravelerId::Other => "THE OTHER",
        }
    }

    /// Get all travelers in order
    pub fn all() -> &'static [TravelerId] {
        &[
            TravelerId::Archivist,
            TravelerId::Wanderer,
            TravelerId::Keeper,
            TravelerId::Child,
            TravelerId::Other,
        ]
    }
}

/// Traveler spawned into existence
#[derive(Event, Debug)]
pub struct TravelerSpawnedEvent {
    pub id: TravelerId,
    pub position: Vec3,
    pub elapsed: f32,
}

/// Traveler pulse/heartbeat occurred
#[derive(Event, Debug)]
pub struct TravelerPulseEvent {
    pub id: TravelerId,
    pub intensity: f32,
    pub elapsed: f32,
}

/// Traveler began fading (dying)
#[derive(Event, Debug)]
pub struct TravelerFadingEvent {
    pub id: TravelerId,
    pub elapsed: f32,
}

/// Traveler fully faded (dead)
#[derive(Event, Debug)]
pub struct TravelerFadedEvent {
    pub id: TravelerId,
    pub elapsed: f32,
}

/// Traveler grief response (reacting to another's death)
#[derive(Event, Debug)]
pub struct TravelerGriefEvent {
    pub mourner: TravelerId,
    pub deceased: TravelerId,
    pub elapsed: f32,
}

/// Travelers synchronized (pulses aligned)
#[derive(Event, Debug)]
pub struct TravelersSyncedEvent {
    pub participants: Vec<TravelerId>,
    pub elapsed: f32,
}

// ============================================================================
// CAMERA EVENTS
// ============================================================================

/// Camera behavior modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CameraBehavior {
    #[default]
    Static,    // No movement
    Breathing, // Subtle sway
    Drift,     // Slow lateral movement
    Approach,  // Moving toward subject
    Pullback,  // Moving away from subject
    Shake,     // Trauma response
}

#[derive(Event, Debug)]
pub struct CameraBehaviorChangedEvent {
    pub from: CameraBehavior,
    pub to: CameraBehavior,
    pub elapsed: f32,
}

#[derive(Event, Debug)]
pub struct CameraShakeEvent {
    pub intensity: f32,
    pub duration: f32,
    pub elapsed: f32,
}

#[derive(Event, Debug)]
pub struct CameraFocusEvent {
    pub target: Option<TravelerId>,
    pub distance: f32,
    pub elapsed: f32,
}

// ============================================================================
// AUDIO EVENTS
// ============================================================================

/// Request to play a sound/note
#[derive(Event, Debug)]
pub struct PlayNoteEvent {
    pub frequency: f32,
    pub duration: f32,
    pub volume: f32,
    pub traveler: Option<TravelerId>,
}

/// Request to play a leitmotif
#[derive(Event, Debug)]
pub struct PlayLeitmotifEvent {
    pub motif: String,
    pub traveler: Option<TravelerId>,
    pub elapsed: f32,
}

/// Audio layer state change
#[derive(Event, Debug)]
pub struct AudioLayerEvent {
    pub layer: String,
    pub action: AudioAction,
    pub elapsed: f32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AudioAction {
    Start,
    Stop,
    FadeIn,
    FadeOut,
}

// ============================================================================
// NARRATIVE EVENTS
// ============================================================================

/// Display a text fragment
#[derive(Event, Debug)]
pub struct DisplayTextEvent {
    pub text: String,
    pub traveler: Option<TravelerId>,
    pub style: NarrativeTextStyle,
    pub elapsed: f32,
}

#[derive(Debug, Clone, Default)]
pub enum NarrativeTextStyle {
    #[default]
    Normal,
    Fragmented,
    Scattered,
    Breath,
    Signal,
}

/// Hide current text
#[derive(Event, Debug)]
pub struct HideTextEvent;

/// Signal overlay state
#[derive(Event, Debug)]
pub struct SignalOverlayEvent {
    pub action: SignalAction,
    pub elapsed: f32,
}

#[derive(Debug, Clone)]
pub enum SignalAction {
    Show,
    Typewriter(String),
    Hide,
}

// ============================================================================
// VISUAL EVENTS
// ============================================================================

/// The Bang sequence events
#[derive(Event, Debug)]
pub struct BangEvent {
    pub stage: BangStage,
    pub elapsed: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BangStage {
    Start,
    LightPoint,
    Expansion,
    Peak,
    Settling,
    Complete,
}

/// Glitch effect trigger
#[derive(Event, Debug)]
pub struct GlitchEvent {
    pub intensity: f32,
    pub duration: f32,
    pub elapsed: f32,
}

/// Environment state change
#[derive(Event, Debug)]
pub struct EnvironmentEvent {
    pub layer: String,
    pub action: String,
    pub elapsed: f32,
}

// ============================================================================
// EVENT PLUGIN
// ============================================================================

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Phase events
            .add_event::<MomentEvent>()
            // Traveler events
            .add_event::<TravelerSpawnedEvent>()
            .add_event::<TravelerPulseEvent>()
            .add_event::<TravelerFadingEvent>()
            .add_event::<TravelerFadedEvent>()
            .add_event::<TravelerGriefEvent>()
            .add_event::<TravelersSyncedEvent>()
            // Camera events
            .add_event::<CameraBehaviorChangedEvent>()
            .add_event::<CameraShakeEvent>()
            .add_event::<CameraFocusEvent>()
            // Audio events
            .add_event::<PlayNoteEvent>()
            .add_event::<PlayLeitmotifEvent>()
            .add_event::<AudioLayerEvent>()
            // Narrative events
            .add_event::<DisplayTextEvent>()
            .add_event::<HideTextEvent>()
            .add_event::<SignalOverlayEvent>()
            // Visual events
            .add_event::<BangEvent>()
            .add_event::<GlitchEvent>()
            .add_event::<EnvironmentEvent>();
    }
}
