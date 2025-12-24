//! Bang sequence: Core explosion, Expansion, God rays, Shockwave, Debris

use bevy::prelude::*;

mod core;
mod expansion;

pub use core::*;
pub use expansion::*;

/// Bang plugin for the origin explosion sequence
pub struct BangPlugin;

impl Plugin for BangPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BangCorePlugin).add_plugins(ExpansionPlugin);

        // TODO: Screen-space god rays
        // TODO: Expanding shockwave
        // TODO: Debris particles (5000)
    }
}
