//! Traveler systems: Components, Geometry, Materials, Particles, Behaviors

use bevy::prelude::*;

mod geometry;
mod identity;
mod lifecycle;
mod materials;
mod spawn;
mod state;

pub use geometry::*;
pub use identity::*;
pub use lifecycle::*;
pub use materials::*;
pub use spawn::*;
pub use state::*;

// Re-export TravelerId from core::events
pub use crate::core::TravelerId;

/// Traveler plugin for components and lifecycle
pub struct TravelersPlugin;

impl Plugin for TravelersPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TravelerRegistry>()
            .init_resource::<TravelerMeshCache>()
            .init_resource::<TravelerMaterialCache>()
            .add_systems(
                Update,
                (
                    handle_traveler_spawns,
                    update_traveler_visibility,
                    finalize_spawn,
                    handle_traveler_fading,
                    check_faded_travelers,
                    handle_grief_events,
                    decay_grief,
                    update_traveler_registry,
                    // Material systems
                    update_pulsing_materials,
                    evolve_materials_for_phase,
                    apply_grief_to_materials,
                ),
            );

        // TODO: Custom shaders
        // TODO: Aura particles
        // TODO: Trail particles
        // TODO: Behaviors (rhythm, sync, grief)
    }
}
