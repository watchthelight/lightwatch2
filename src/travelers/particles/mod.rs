//! Traveler particle systems - aura and trails

use bevy::prelude::*;

pub mod aura;
pub mod trails;

pub use aura::*;
pub use trails::*;

/// Plugin for traveler particle effects
pub struct TravelerParticlesPlugin;

impl Plugin for TravelerParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_trail_mesh_cache)
            .add_systems(
                Update,
                (
                    // Aura systems
                    spawn_aura_particles,
                    animate_aura_particles,
                    control_particle_density,
                    fade_aura_with_traveler,
                    // Trail systems
                    setup_traveler_trails,
                    update_position_history,
                    spawn_trail_particles,
                    update_trail_particles,
                    control_trail_activation,
                ),
            );
    }
}
