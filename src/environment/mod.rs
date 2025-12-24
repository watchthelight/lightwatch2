//! Environment systems: Starfield, Nebula, Dust, Fog, Reflection

use bevy::prelude::*;

mod dust;
mod fog;
mod nebula;
mod starfield;

pub use dust::*;
pub use fog::*;
pub use nebula::*;
pub use starfield::*;

/// Environment plugin for cosmic backdrop
pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(StarfieldPlugin)
            .add_plugins(NebulaPlugin)
            .add_plugins(DustPlugin)
            .add_plugins(FogPlugin);

        // TODO: Reflection plane
    }
}
