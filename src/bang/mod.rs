//! Bang sequence: Core explosion, Expansion, God rays, Shockwave, Debris

use bevy::prelude::*;

mod core;
mod debris;
mod expansion;
mod god_rays;
mod shockwave;

pub use core::*;
pub use debris::*;
pub use expansion::*;
pub use god_rays::*;
pub use shockwave::*;

/// Bang plugin for the origin explosion sequence
pub struct BangPlugin;

impl Plugin for BangPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BangCorePlugin)
            .add_plugins(ExpansionPlugin)
            .add_plugins(GodRaysPlugin)
            .add_plugins(ShockwavePlugin)
            .add_plugins(DebrisPlugin);
    }
}
