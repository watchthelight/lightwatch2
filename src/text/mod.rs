//! Text transmission system - typewriter reveal, hold, fade

mod api;
mod config;
mod final_messages;
mod fragment_display;
mod fragments;
mod grief;
mod queue;
mod signal;
mod spawn;
mod transmission;
mod typewriter;

pub use api::TransmissionCommands;
pub use config::{TextConfig, TextPosition};
pub use final_messages::FinalMessageState;
pub use fragment_display::FragmentState;
pub use grief::GriefTextState;
pub use queue::TransmissionQueue;
pub use signal::{SignalConfig, SignalState};
pub use transmission::{Transmission, TransmissionState};

use bevy::prelude::*;

/// Transmission plugin for text display
pub struct TransmissionPlugin;

impl Plugin for TransmissionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TextConfig>()
            .init_resource::<TransmissionQueue>()
            .add_systems(
                Update,
                (
                    spawn::update_transmission_queue,
                    spawn::spawn_queued_transmissions,
                    typewriter::update_typewriter,
                    typewriter::cleanup_transmissions,
                )
                    .chain(),
            );
    }
}

/// Fragments plugin for signal detection and traveler text
pub struct FragmentsPlugin;

impl Plugin for FragmentsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SignalConfig>()
            .init_resource::<SignalState>()
            .init_resource::<FragmentState>()
            .init_resource::<GriefTextState>()
            .init_resource::<FinalMessageState>()
            .add_systems(
                Update,
                (
                    signal::start_signal_detection,
                    signal::reveal_travelers,
                    fragment_display::display_fragments,
                    grief::show_grief_text,
                    final_messages::show_final_messages,
                ),
            );
    }
}

/// Text plugin (includes transmission and fragments)
pub struct TextPlugin;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TransmissionPlugin)
            .add_plugins(FragmentsPlugin);
    }
}
