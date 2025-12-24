//! Post-processing: Bloom, Chromatic aberration, Grain, Vignette, Tonemapping
//!
//! This module provides cinematic post-processing for LIGHTWATCH:
//! - Bloom: Built-in Bevy bloom with dynamic intensity during bang
//! - Tonemapping: ACES Filmic (set on camera spawn)
//! - Chromatic aberration: Edge color fringing during intense moments
//! - Film grain: Subtle texture, stronger at start/end
//! - Vignette: Corner darkening, pulses at phase transitions
//!
//! Bloom and tonemapping use Bevy's built-in systems.
//! Custom effects (CA, grain, vignette) have shaders ready for render graph integration.

mod bloom;
mod config;
mod dynamic;
mod materials;

pub use config::*;
pub use dynamic::DynamicPostProcess;
pub use materials::*;

use bevy::prelude::*;
use bevy::sprite::Material2dPlugin;

/// Post-processing plugin for final visual polish
pub struct PostPlugin;

impl Plugin for PostPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PostProcessConfig>()
            .init_resource::<DynamicPostProcess>()
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
                ),
            );

        info!(target: "lightwatch::post", "Post-processing plugin initialized");
    }
}
