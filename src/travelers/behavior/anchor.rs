//! Anchor behavior - formation during Connection phase

use bevy::prelude::*;

use crate::core::{ExperienceClock, Phase, TravelerId};
use crate::travelers::Traveler;

/// Anchor points for Connection phase
#[derive(Component, Debug)]
pub struct TravelerAnchor {
    /// Target position during Connection
    pub target: Vec3,
    /// How strongly to pull toward target
    pub strength: f32,
    /// Is anchoring active?
    pub active: bool,
}

impl TravelerAnchor {
    pub fn for_traveler(id: TravelerId) -> Self {
        // Formation positions for Connection
        let target = match id {
            TravelerId::Archivist => Vec3::new(0.0, 0.0, -2.0),  // Center-back
            TravelerId::Wanderer => Vec3::new(-1.5, 0.5, -1.0), // Left
            TravelerId::Keeper => Vec3::new(1.5, -0.3, -1.0),   // Right
            TravelerId::Child => Vec3::new(0.0, 0.8, 0.0),      // Front-top
            TravelerId::Other => Vec3::new(0.0, -1.0, -5.0),    // Distant
        };

        Self {
            target,
            strength: match id {
                TravelerId::Other => 0.1, // Weak - never fully joins
                _ => 0.5,
            },
            active: false,
        }
    }
}

/// Pull travelers toward anchor points during Connection
pub fn apply_anchor_behavior(
    time: Res<Time>,
    clock: Res<ExperienceClock>,
    mut travelers: Query<(&Traveler, &mut Transform, &mut TravelerAnchor)>,
) {
    let phase = clock.phase();

    // Anchoring only during Connection
    for (_traveler, mut transform, mut anchor) in travelers.iter_mut() {
        anchor.active = phase == Phase::Connection;

        if !anchor.active {
            continue;
        }

        // Pull toward target
        let direction = anchor.target - transform.translation;
        let distance = direction.length();

        if distance > 0.1 {
            let pull = direction.normalize() * anchor.strength * time.delta_seconds();
            transform.translation += pull;
        }
    }
}
