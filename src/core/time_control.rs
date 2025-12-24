//! Development time control - pause, speed, phase jumping

#![allow(dead_code)]

use bevy::prelude::*;

/// Development time control
#[derive(Resource)]
pub struct TimeControl {
    pub paused: bool,
    pub speed: f32,
    pub scrub_position: Option<f32>,
}

impl Default for TimeControl {
    fn default() -> Self {
        Self {
            paused: false,
            speed: 1.0,
            scrub_position: None,
        }
    }
}

/// Handle time control inputs (development only)
#[cfg(debug_assertions)]
pub fn handle_time_control(keyboard: Res<ButtonInput<KeyCode>>, mut control: ResMut<TimeControl>) {
    // Space: pause/resume
    if keyboard.just_pressed(KeyCode::Space) {
        control.paused = !control.paused;
        info!(
            target: "lightwatch::debug",
            "Time {}",
            if control.paused { "PAUSED" } else { "RESUMED" }
        );
    }

    // [ and ]: adjust speed
    if keyboard.just_pressed(KeyCode::BracketLeft) {
        control.speed = (control.speed * 0.5).max(0.1);
        info!(target: "lightwatch::debug", "Speed: {:.1}x", control.speed);
    }
    if keyboard.just_pressed(KeyCode::BracketRight) {
        control.speed = (control.speed * 2.0).min(4.0);
        info!(target: "lightwatch::debug", "Speed: {:.1}x", control.speed);
    }

    // Number keys: jump to phase
    let jumps = [
        (KeyCode::Digit1, 0.0, "signal"),
        (KeyCode::Digit2, 2.0, "bang"),
        (KeyCode::Digit3, 12.0, "awakening"),
        (KeyCode::Digit4, 27.0, "discovery"),
        (KeyCode::Digit5, 57.0, "connection"),
        (KeyCode::Digit6, 87.0, "acceptance"),
        (KeyCode::Digit0, 140.0, "end"),
    ];

    for (key, time, phase) in jumps {
        if keyboard.just_pressed(key) {
            control.scrub_position = Some(time);
            info!(
                target: "lightwatch::debug",
                "Jumping to {} ({:.0}s)", phase, time
            );
        }
    }
}
