//! Post-processing: Bloom, Chromatic aberration, Grain, Vignette, Tonemapping
//!
//! This module provides cinematic post-processing for LIGHTWATCH:
//! - Bloom: Built-in Bevy bloom with dynamic intensity during bang
//! - Tonemapping: ACES Filmic (set on camera spawn)
//! - Chromatic aberration: Edge color fringing during intense moments (render graph)
//! - Vignette: Corner darkening, pulses at phase transitions (render graph)
//! - Film grain: Subtle texture, stronger at start/end
//!
//! Bloom and tonemapping use Bevy's built-in systems.
//! Chromatic aberration and vignette use custom render graph nodes.

mod bloom;
mod chromatic_node;
mod config;
mod dynamic;
mod materials;
mod vignette_node;

pub use chromatic_node::{ChromaticAberrationPlugin, ChromaticAberrationSettings};
pub use config::*;
pub use dynamic::DynamicPostProcess;
pub use materials::*;
pub use vignette_node::{VignettePlugin, VignetteSettings};

use bevy::prelude::*;
use bevy::sprite::Material2dPlugin;

/// Post-processing plugin for final visual polish
pub struct PostPlugin;

impl Plugin for PostPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PostProcessConfig>()
            .init_resource::<DynamicPostProcess>()
            // Render graph post-processing nodes
            .add_plugins((ChromaticAberrationPlugin, VignettePlugin))
            // Register custom material types (for future render integration)
            .add_plugins((
                Material2dPlugin::<ChromaticAberrationMaterial>::default(),
                Material2dPlugin::<FilmGrainMaterial>::default(),
                Material2dPlugin::<VignetteMaterial>::default(),
            ))
            .add_systems(
                Update,
                (
                    bloom::update_bloom_for_bang,
                    dynamic::update_chromatic_aberration,
                    dynamic::update_film_grain,
                    dynamic::update_vignette,
                    sync_chromatic_settings,
                    sync_vignette_settings,
                ),
            );

        info!(target: "lightwatch::post", "Post-processing plugin initialized");
    }
}

/// Sync ChromaticAberrationSettings component with DynamicPostProcess state
fn sync_chromatic_settings(
    dynamic: Res<DynamicPostProcess>,
    mut cameras: Query<&mut ChromaticAberrationSettings>,
) {
    for mut settings in cameras.iter_mut() {
        settings.intensity = dynamic.chromatic_intensity;
    }
}

/// Sync VignetteSettings component with DynamicPostProcess state
fn sync_vignette_settings(
    dynamic: Res<DynamicPostProcess>,
    mut cameras: Query<&mut VignetteSettings>,
) {
    for mut settings in cameras.iter_mut() {
        settings.intensity = dynamic.vignette_intensity;
    }
}
