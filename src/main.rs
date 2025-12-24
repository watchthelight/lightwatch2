//! LIGHTWATCH - A 143-second contemplative experience
//!
//! Five travelers. Fourteen billion years. One final transmission.

use bevy::prelude::*;

mod audio;
mod bang;
mod camera;
mod core;
mod environment;
mod narrative;
mod post;
mod shaders;
mod travelers;

fn main() {
    App::new()
        // Window configuration
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "LIGHTWATCH".into(),
                resolution: (1920., 1080.).into(),
                present_mode: bevy::window::PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        // Set background to pure black
        .insert_resource(ClearColor(Color::BLACK))
        // Initialize our systems
        .add_plugins(LightwatchPlugin)
        .run();
}

/// Main plugin that coordinates all LIGHTWATCH systems
pub struct LightwatchPlugin;

impl Plugin for LightwatchPlugin {
    fn build(&self, app: &mut App) {
        app
            // Core systems (clock, state, events)
            .add_plugins(core::CorePlugin)
            // Camera systems
            .add_plugins(camera::CameraPlugin)
            // Traveler systems
            .add_plugins(travelers::TravelersPlugin)
            // Environment systems
            .add_plugins(environment::EnvironmentPlugin)
            // Bang sequence
            .add_plugins(bang::BangPlugin)
            // Audio systems
            .add_plugins(audio::AudioPlugin)
            // Narrative/text systems
            .add_plugins(narrative::NarrativePlugin)
            // Post-processing
            .add_plugins(post::PostPlugin);
    }
}
