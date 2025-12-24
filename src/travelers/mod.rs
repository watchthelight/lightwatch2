//! Traveler systems: Components, Geometry, Materials, Particles, Behaviors

use bevy::prelude::*;

/// The five travelers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TravelerId {
    /// Icosahedron, amber, 0.14 Hz - The organizer
    Archivist,
    /// Tetrahedron, cyan, 0.11 Hz - The searcher
    Wanderer,
    /// Cube, orange, 0.08 Hz - The listener
    Keeper,
    /// Octahedron, white, 0.18 Hz - The youngest, dies first
    Child,
    /// Dodecahedron, violet, 0.06 Hz - Alien, silent, dies last
    Other,
}

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
