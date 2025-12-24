//! Text transmission system - typewriter reveal, hold, fade

mod api;
mod config;
mod queue;
mod spawn;
mod transmission;
mod typewriter;

pub use api::TransmissionCommands;
pub use config::{TextConfig, TextPosition};
pub use queue::TransmissionQueue;
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

/// Text plugin (re-export for convenience)
pub struct TextPlugin;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TransmissionPlugin);
    }
}
