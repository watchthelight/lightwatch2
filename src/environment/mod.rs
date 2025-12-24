//! Environment systems: Starfield, Nebula, Dust, Fog, Reflection

use bevy::prelude::*;

mod dust;
mod nebula;
mod starfield;

pub use dust::*;
pub use nebula::*;
pub use starfield::*;

/// Environment plugin for cosmic backdrop
pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(StarfieldPlugin)
            .add_plugins(NebulaPlugin)
            .add_plugins(DustPlugin);

        // TODO: Volumetric fog
        // TODO: Reflection plane
    }
}
