//! LIGHTWATCH - A 143-second contemplative experience
//!
//! Five travelers. Fourteen billion years. One final transmission.

use bevy::prelude::*;
use bevy::render::settings::{Backends, RenderCreation, WgpuSettings};
use bevy::render::RenderPlugin;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod audio;
mod bang;
mod camera;
mod core;
mod environment;
mod narrative;
mod post;
mod shaders;
mod text;
mod travelers;

fn main() {
    // Initialize tracing with structured output
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                if cfg!(debug_assertions) {
                    "lightwatch=debug,lightwatch::events=info,wgpu=warn,bevy=info".into()
                } else {
                    "lightwatch=info,wgpu=error,bevy=warn".into()
                }
            }),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .with_thread_ids(false)
                .with_file(false)
                .with_line_number(false),
        )
        .init();

    core::BuildInfo::log_info();

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
            // Shader loading (must be first for material registration)
            .add_plugins(shaders::ShadersPlugin)
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
            .add_plugins(post::PostPlugin)
            // Final integration (performance, polish, verification)
            .add_plugins(core::IntegrationPlugin);
    }
}
