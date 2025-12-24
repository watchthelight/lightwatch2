//! Traveler systems: Components, Geometry, Materials, Particles, Behaviors

use bevy::prelude::*;

// Re-export TravelerId from core::events
pub use crate::core::TravelerId;

/// Traveler plugin for rendering and behavior
pub struct TravelersPlugin;

impl Plugin for TravelersPlugin {
    fn build(&self, _app: &mut App) {
        // TODO: Traveler components
        // TODO: Procedural geometry
        // TODO: PBR materials
        // TODO: Custom shaders
        // TODO: Aura particles
        // TODO: Trail particles
        // TODO: Behaviors (rhythm, sync, grief)
    }
}
