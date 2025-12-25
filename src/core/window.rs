//! Window configuration and handling for LIGHTWATCH

use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode, WindowResolution};

/// Window configuration for LIGHTWATCH
pub fn configure_window() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "LIGHTWATCH".into(),
            resolution: WindowResolution::new(1920., 1080.),
            present_mode: PresentMode::Fifo,  // Strict VSync - caps to display refresh rate
            mode: WindowMode::Windowed,
            resizable: true,
            decorations: true,
            transparent: false,
            focused: true,
            window_level: bevy::window::WindowLevel::Normal,
            ..default()
        }),
        ..default()
    }
}

/// System to handle window close request via ESC key
pub fn handle_window_close(
    mut exit: EventWriter<bevy::app::AppExit>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        exit.send(bevy::app::AppExit::Success);
    }
}

/// Toggle fullscreen with F11
pub fn toggle_fullscreen(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut windows: Query<&mut Window>,
) {
    if keyboard.just_pressed(KeyCode::F11) {
        if let Ok(mut window) = windows.get_single_mut() {
            window.mode = match window.mode {
                WindowMode::Windowed => WindowMode::BorderlessFullscreen,
                _ => WindowMode::Windowed,
            };
        }
    }
}
