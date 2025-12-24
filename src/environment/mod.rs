//! Environment systems: Starfield, Nebula, Dust, Fog, Reflection

use bevy::prelude::*;

mod nebula;
mod starfield;

pub use nebula::*;
pub use starfield::*;

/// Environment plugin for cosmic backdrop
pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(StarfieldPlugin)
            .add_plugins(NebulaPlugin);

        // TODO: Dust particles (10,000)
        // TODO: Volumetric fog
        // TODO: Reflection plane
    }
}
