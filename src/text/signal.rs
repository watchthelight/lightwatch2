//! Signal detection overlay

use bevy::prelude::*;

use super::fragments::traveler_display_name;
use super::{TextPosition, Transmission, TransmissionCommands, TransmissionQueue};
use crate::core::{ExperienceClock, TravelerId};

/// Signal detection configuration
#[derive(Resource)]
pub struct SignalConfig {
    /// Time between signal reveals
    pub reveal_interval: f32,
    /// Order of traveler reveals
    pub reveal_order: Vec<TravelerId>,
}

impl Default for SignalConfig {
    fn default() -> Self {
        Self {
            reveal_interval: 0.4,
            reveal_order: vec![
                TravelerId::Archivist,
                TravelerId::Wanderer,
                TravelerId::Keeper,
                TravelerId::Child,
                TravelerId::Other,
            ],
        }
    }
}

/// Signal detection state
#[derive(Resource, Default)]
pub struct SignalState {
    pub detection_started: bool,
    pub travelers_revealed: usize,
    pub last_reveal_time: f32,
    pub detection_complete: bool,
}

/// Start signal detection at experience start
pub fn start_signal_detection(
    clock: Res<ExperienceClock>,
    mut state: ResMut<SignalState>,
    mut queue: ResMut<TransmissionQueue>,
) {
    let elapsed = clock.elapsed();

    // Start detection at 0.5 seconds
    if elapsed >= 0.5 && !state.detection_started {
        state.detection_started = true;

        // Opening transmission
        queue.transmit_full(
            Transmission::new("SIGNAL DETECTED")
                .with_position(TextPosition::TopLeft)
                .with_speed(20.0)
                .with_hold(1.5),
            0.0,
        );

        info!(target: "lightwatch::text", "Signal detection started at {:.2}s", elapsed);
    }
}

/// Reveal travelers one by one
pub fn reveal_travelers(
    clock: Res<ExperienceClock>,
    config: Res<SignalConfig>,
    mut state: ResMut<SignalState>,
    mut queue: ResMut<TransmissionQueue>,
) {
    if !state.detection_started || state.detection_complete {
        return;
    }

    let elapsed = clock.elapsed();

    // Check if it's time for next reveal
    if state.travelers_revealed < config.reveal_order.len() {
        let expected_time = 1.0 + (state.travelers_revealed as f32 * config.reveal_interval);

        if elapsed >= expected_time && elapsed - state.last_reveal_time >= config.reveal_interval {
            let traveler = config.reveal_order[state.travelers_revealed];
            let name = traveler_display_name(traveler);

            queue.transmit_full(
                Transmission::new(format!("• {}", name))
                    .with_position(TextPosition::TopLeft)
                    .with_speed(25.0)
                    .with_hold(0.8),
                0.0,
            );

            state.travelers_revealed += 1;
            state.last_reveal_time = elapsed;

            info!(target: "lightwatch::text", "Traveler revealed: {:?}", traveler);
        }
    }

    // Mark complete when all revealed and past Signal phase
    if state.travelers_revealed >= config.reveal_order.len() && elapsed > 2.0 {
        state.detection_complete = true;

        info!(target: "lightwatch::text", "Signal detection complete at {:.2}s", elapsed);
    }
}
