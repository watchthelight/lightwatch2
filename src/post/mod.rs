//! Post-processing: Bloom, DOF, Chromatic aberration, Grain, Tonemapping

use bevy::prelude::*;

/// Post-processing plugin for final visual polish
pub struct PostPlugin;

impl Plugin for PostPlugin {
    fn build(&self, _app: &mut App) {
        // TODO: HDR Bloom
        // TODO: Depth of field (Bokeh)
        // TODO: Chromatic aberration
        // TODO: Film grain
        // TODO: ACES tonemapping
    }
}
