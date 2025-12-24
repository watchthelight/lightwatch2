//! Grief text for traveler fading

use bevy::prelude::*;

use super::{TextPosition, Transmission, TransmissionCommands, TransmissionQueue};
use crate::core::{TravelerFadedEvent, TravelerId};

/// State for grief text
#[derive(Resource, Default)]
pub struct GriefTextState {
    pub child_grief_shown: bool,
}

/// Show grief text when Child fades
pub fn show_grief_text(
    mut events: EventReader<TravelerFadedEvent>,
    mut state: ResMut<GriefTextState>,
    mut queue: ResMut<TransmissionQueue>,
) {
    for event in events.read() {
        if event.id == TravelerId::Child && !state.child_grief_shown {
            state.child_grief_shown = true;

            queue.transmit_full(
                Transmission::new("the child fades first")
                    .with_position(TextPosition::Center)
                    .with_speed(8.0)
                    .with_hold(5.0),
                0.5,
            );

            queue.transmit_full(
                Transmission::new("as the youngest always do")
                    .with_position(TextPosition::Center)
                    .with_speed(8.0)
                    .with_hold(4.0)
                    .with_priority(-1),
                3.0,
            );

            info!(target: "lightwatch::text", "Grief text shown for Child");
        }
    }
}
