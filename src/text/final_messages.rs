//! Final messages as experience ends

use bevy::prelude::*;

use super::{TextPosition, Transmission, TransmissionCommands, TransmissionQueue};
use crate::core::ExperienceClock;

/// State for final messages
#[derive(Resource, Default)]
pub struct FinalMessageState {
    pub messages_started: bool,
    #[allow(dead_code)]
    pub messages_shown: usize,
}

/// Show final messages as experience ends
pub fn show_final_messages(
    clock: Res<ExperienceClock>,
    mut state: ResMut<FinalMessageState>,
    mut queue: ResMut<TransmissionQueue>,
) {
    let elapsed = clock.elapsed();

    // Start final messages at 130 seconds
    if elapsed >= 130.0 && !state.messages_started {
        state.messages_started = true;

        queue.transmit_full(
            Transmission::new("They built beacons")
                .with_position(TextPosition::Center)
                .with_speed(6.0)
                .with_hold(3.0),
            0.0,
        );

        queue.transmit_full(
            Transmission::new("that would outlast their stars")
                .with_position(TextPosition::Center)
                .with_speed(6.0)
                .with_hold(3.0),
            3.5,
        );

        queue.transmit_full(
            Transmission::new("We receive their light")
                .with_position(TextPosition::Center)
                .with_speed(6.0)
                .with_hold(3.0),
            7.0,
        );

        queue.transmit_full(
            Transmission::new("long after they are gone")
                .with_position(TextPosition::Center)
                .with_speed(6.0)
                .with_hold(4.0),
            10.5,
        );

        info!(target: "lightwatch::text", "Final messages started at {:.2}s", elapsed);
    }
}
