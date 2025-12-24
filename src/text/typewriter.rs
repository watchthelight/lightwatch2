//! Typewriter effect system

use bevy::prelude::*;

use super::{Transmission, TransmissionState};

/// Update transmission typewriter effect
pub fn update_typewriter(
    time: Res<Time>,
    mut transmissions: Query<(&mut Transmission, &mut Text)>,
) {
    let dt = time.delta_seconds();

    for (mut transmission, mut text) in transmissions.iter_mut() {
        transmission.state_time += dt;

        match transmission.state {
            TransmissionState::Typing => {
                // Calculate characters to reveal
                let chars_to_reveal =
                    (transmission.state_time * transmission.chars_per_second) as usize;
                transmission.revealed_chars = chars_to_reveal.min(transmission.full_text.len());

                // Update displayed text
                if let Some(section) = text.sections.first_mut() {
                    section.value = transmission.visible_text().to_string();
                }

                // Check for completion
                if transmission.typing_complete() {
                    transmission.state = TransmissionState::Holding;
                    transmission.state_time = 0.0;

                    info!(target: "lightwatch::text", "Transmission revealed: {}", &transmission.full_text);
                }
            }
            TransmissionState::Holding => {
                if transmission.state_time >= transmission.hold_duration {
                    transmission.state = TransmissionState::Fading;
                    transmission.state_time = 0.0;
                }
            }
            TransmissionState::Fading => {
                let progress = transmission.state_time / transmission.fade_duration;
                transmission.opacity = 1.0 - progress.clamp(0.0, 1.0);

                // Update text opacity
                if let Some(section) = text.sections.first_mut() {
                    let color = section.style.color;
                    section.style.color = color.with_alpha(transmission.opacity);
                }

                if progress >= 1.0 {
                    transmission.state = TransmissionState::Complete;

                    info!(target: "lightwatch::text", "Transmission complete: {}", &transmission.full_text);
                }
            }
            TransmissionState::Complete => {
                // Will be despawned by cleanup system
            }
        }
    }
}

/// Cleanup completed transmissions
pub fn cleanup_transmissions(
    mut commands: Commands,
    transmissions: Query<(Entity, &Transmission)>,
) {
    for (entity, transmission) in transmissions.iter() {
        if transmission.state == TransmissionState::Complete {
            commands.entity(entity).despawn_recursive();
        }
    }
}
