//! Traveler particle systems - aura and trails

use bevy::prelude::*;

pub mod aura;

pub use aura::*;

/// Plugin for traveler particle effects
pub struct TravelerParticlesPlugin;

impl Plugin for TravelerParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_aura_particles,
                animate_aura_particles,
                control_particle_density,
                fade_aura_with_traveler,
            ),
        );
    }
}
