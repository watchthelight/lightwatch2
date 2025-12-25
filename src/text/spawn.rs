//! Transmission spawning system

use bevy::prelude::*;

use super::{TextConfig, Transmission, TransmissionQueue};

/// Spawn transmission from queue
pub fn spawn_queued_transmissions(
    mut commands: Commands,
    config: Res<TextConfig>,
    mut queue: ResMut<TransmissionQueue>,
) {
    while let Some(transmission) = queue.next() {
        spawn_transmission(&mut commands, &config, transmission);
    }
}

/// Spawn a single transmission
fn spawn_transmission(commands: &mut Commands, config: &TextConfig, transmission: Transmission) {
    let position = transmission.position;

    // Calculate screen position
    let (x, y) = position.to_offset();

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "", // Start empty
                TextStyle {
                    font: config.font.clone(),
                    font_size: config.font_size,
                    color: config.text_color,
                },
            )
            .with_justify(position.to_justify()),
            transform: Transform::from_xyz(x, y, 100.0), // High Z for UI layer
            ..default()
        },
        transmission,
        Name::new("Transmission"),
    ));

    info!(target: "lightwatch::text", "Transmission spawned");
}

/// Update queue timers
pub fn update_transmission_queue(time: Res<Time>, mut queue: ResMut<TransmissionQueue>) {
    queue.update(time.delta_seconds());
}
