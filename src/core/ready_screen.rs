//! Ready screen - "click to begin" overlay

use bevy::prelude::*;

/// Marker for the ready screen
#[derive(Component)]
pub struct ReadyScreen;

/// Spawn the "click to begin" overlay
pub fn spawn_ready_screen(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::BLACK),
                ..default()
            },
            ReadyScreen,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "click to begin",
                TextStyle {
                    font_size: 18.0,
                    color: Color::srgba(0.9, 0.87, 0.82, 0.6),
                    ..default()
                },
            ));
        });
}

/// Hide ready screen when experience starts
pub fn hide_ready_screen(mut commands: Commands, query: Query<Entity, With<ReadyScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
