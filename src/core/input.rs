//! Input handler - click to start, cursor management, extended dev controls

use bevy::prelude::*;
use bevy::window::CursorGrabMode;

use super::state::ExperienceState;
use super::ExperienceClock;
#[cfg(debug_assertions)]
use super::TimeControl;
use crate::wide_event;

/// Input handler configuration
#[derive(Resource)]
pub struct InputConfig {
    /// Is user input enabled?
    pub enabled: bool,
    /// Should cursor be hidden during experience?
    pub hide_cursor: bool,
}

impl Default for InputConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            hide_cursor: true,
        }
    }
}

/// Handle click to start (any mouse button)
pub fn handle_click_to_start(
    state: Res<State<ExperienceState>>,
    mut next_state: ResMut<NextState<ExperienceState>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut clock: ResMut<ExperienceClock>,
    mut windows: Query<&mut Window>,
    config: Res<InputConfig>,
) {
    // Only in Ready state
    if *state.get() != ExperienceState::Ready {
        return;
    }

    if !config.enabled {
        return;
    }

    // Any click starts the experience
    if mouse.just_pressed(MouseButton::Left)
        || mouse.just_pressed(MouseButton::Right)
        || mouse.just_pressed(MouseButton::Middle)
    {
        // Start the experience
        next_state.set(ExperienceState::Running);
        clock.start();

        // Hide cursor
        if config.hide_cursor {
            if let Ok(mut window) = windows.get_single_mut() {
                window.cursor.visible = false;
                window.cursor.grab_mode = CursorGrabMode::Confined;
            }
        }

        wide_event!("experience_started")
            .with_str("trigger", "click")
            .emit(0.0);
    }
}

/// Disable input after experience starts
pub fn disable_input_during_experience(
    state: Res<State<ExperienceState>>,
    mut config: ResMut<InputConfig>,
    mut done: Local<bool>,
) {
    // Only disable once when transitioning to Running
    if *state.get() == ExperienceState::Running && !*done {
        config.enabled = false;
        *done = true;
    }
}

/// Restore cursor when experience ends
pub fn restore_cursor_on_end(
    state: Res<State<ExperienceState>>,
    mut windows: Query<&mut Window>,
    mut done: Local<bool>,
) {
    if *state.get() == ExperienceState::Ended && !*done {
        if let Ok(mut window) = windows.get_single_mut() {
            window.cursor.visible = true;
            window.cursor.grab_mode = CursorGrabMode::None;
        }
        *done = true;
    }
}

/// Extended development controls (arrow keys, R reset, additional phase jumps)
#[cfg(debug_assertions)]
pub fn handle_extended_dev_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut time_control: ResMut<TimeControl>,
    clock: Res<ExperienceClock>,
    state: Res<State<ExperienceState>>,
) {
    // Only when running
    if *state.get() != ExperienceState::Running {
        return;
    }

    // R: reset to beginning
    if keyboard.just_pressed(KeyCode::KeyR) {
        time_control.scrub_position = Some(0.0);
        info!(target: "lightwatch::input", "Resetting to start");
    }

    // Additional phase jump keys (7, 8)
    if keyboard.just_pressed(KeyCode::Digit7) {
        time_control.scrub_position = Some(130.0);
        info!(target: "lightwatch::input", "Jumping to near end (130s)");
    }
    if keyboard.just_pressed(KeyCode::Digit8) {
        time_control.scrub_position = Some(140.0);
        info!(target: "lightwatch::input", "Jumping to end (140s)");
    }

    // Arrow keys: scrub
    let shift_held = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);
    let scrub_amount = if shift_held { 5.0 } else { 0.5 };

    if keyboard.pressed(KeyCode::ArrowLeft) {
        let new_time = (clock.elapsed() - scrub_amount).max(0.0);
        time_control.scrub_position = Some(new_time);
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        let new_time = (clock.elapsed() + scrub_amount).min(143.0);
        time_control.scrub_position = Some(new_time);
    }
}

/// Show development help (F1)
#[cfg(debug_assertions)]
pub fn show_dev_help(keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::F1) {
        info!(target: "lightwatch::help", "
╔══════════════════════════════════════════════════════════════╗
║                    LIGHTWATCH DEV CONTROLS                   ║
╠══════════════════════════════════════════════════════════════╣
║  SPACE        Pause/Resume                                   ║
║  [ / ]        Slow down / Speed up                           ║
║  R            Reset to start                                 ║
║  1-8          Jump to phase (Signal, Bang, Awakening...)     ║
║  0            Jump to end                                    ║
║  ← / →        Scrub backwards/forwards                       ║
║  SHIFT+←/→    Fast scrub (5s increments)                     ║
║  F1           Show this help                                 ║
║  F3           Toggle debug overlay                           ║
║  F11          Toggle fullscreen                              ║
║  ESC          Quit                                           ║
╚══════════════════════════════════════════════════════════════╝
        ");
    }
}

/// Input plugin
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputConfig>().add_systems(
            Update,
            (
                handle_click_to_start,
                disable_input_during_experience,
                restore_cursor_on_end,
            ),
        );

        #[cfg(debug_assertions)]
        app.add_systems(Update, (handle_extended_dev_controls, show_dev_help));
    }
}
