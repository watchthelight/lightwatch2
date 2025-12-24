//! LIGHTWATCH - A 143-second contemplative experience
//!
//! Five travelers. Fourteen billion years. One final transmission.

use bevy::prelude::*;
use bevy::render::settings::{Backends, RenderCreation, WgpuSettings};
use bevy::render::RenderPlugin;

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
        .add_plugins(
            DefaultPlugins
                .set(core::configure_window())
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        backends: Some(Backends::PRIMARY),
                        power_preference: bevy::render::settings::PowerPreference::HighPerformance,
                        ..default()
                    }),
                    ..default()
                }),
        )
        // Set background to pure black
        .insert_resource(ClearColor(Color::BLACK))
        // 4x MSAA for anti-aliasing
        .insert_resource(core::configure_msaa())
        // Initialize our systems
        .add_plugins(LightwatchPlugin)
        .run();
}

/// Main plugin that coordinates all LIGHTWATCH systems
pub struct LightwatchPlugin;

impl Plugin for LightwatchPlugin {
    fn build(&self, app: &mut App) {
        app
            // Core systems (window, renderer, exposure, clock, state, events)
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
