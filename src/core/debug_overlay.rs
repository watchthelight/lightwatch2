//! Debug overlay - FPS, time, phase display

use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

/// Marker for debug overlay text
#[derive(Component)]
pub struct DebugOverlayText;

/// Marker for debug overlay container
#[derive(Component)]
pub struct DebugOverlay;

/// Debug overlay state
#[derive(Resource)]
pub struct DebugOverlayState {
    pub visible: bool,
    pub phase: String,
    pub elapsed: f32,
    pub traveler_count: usize,
    pub particle_count: usize,
}

impl Default for DebugOverlayState {
    fn default() -> Self {
        Self {
            visible: cfg!(debug_assertions), // Visible in debug builds
            phase: "signal".into(),
            elapsed: 0.0,
            traveler_count: 0,
            particle_count: 0,
        }
    }
}

/// Spawn the debug overlay UI
pub fn spawn_debug_overlay(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(10.0),
                    left: Val::Px(10.0),
                    padding: UiRect::all(Val::Px(8.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
                ..default()
            },
            DebugOverlay,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "LIGHTWATCH DEBUG",
                    TextStyle {
                        font_size: 14.0,
                        color: Color::srgb(0.0, 1.0, 0.5),
                        ..default()
                    },
                ),
                DebugOverlayText,
            ));
        });
}

/// Update debug overlay text
pub fn update_debug_overlay(
    state: Res<DebugOverlayState>,
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<DebugOverlayText>>,
) {
    if !state.visible {
        return;
    }

    let fps = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|d| d.smoothed())
        .unwrap_or(0.0);

    for mut text in query.iter_mut() {
        text.sections[0].value = format!(
            "LIGHTWATCH DEBUG\n\
             FPS: {:.0}\n\
             Time: {:.2}s / 143s\n\
             Phase: {}\n\
             Travelers: {}\n\
             Particles: {}",
            fps, state.elapsed, state.phase, state.traveler_count, state.particle_count,
        );
    }
}

/// Toggle debug overlay visibility with F3
pub fn toggle_debug_overlay(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<DebugOverlayState>,
    mut query: Query<&mut Visibility, With<DebugOverlay>>,
) {
    if keyboard.just_pressed(KeyCode::F3) {
        state.visible = !state.visible;
        for mut vis in query.iter_mut() {
            *vis = if state.visible {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}
