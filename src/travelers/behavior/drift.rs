//! Drift behavior - organic movement for travelers

use bevy::prelude::*;
use rand::Rng;

use crate::core::{ExperienceClock, Phase, TravelerId};
use crate::travelers::Traveler;

/// Drift configuration per traveler
#[derive(Component, Debug)]
pub struct TravelerDrift {
    /// Current drift velocity
    pub velocity: Vec3,
    /// Maximum drift speed
    pub max_speed: f32,
    /// Drift noise seed
    pub noise_seed: f32,
    /// Is drifting active?
    pub active: bool,
}

impl TravelerDrift {
    pub fn for_traveler(id: TravelerId) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            velocity: Vec3::ZERO,
            max_speed: match id {
                TravelerId::Archivist => 0.05,
                TravelerId::Wanderer => 0.15, // More movement
                TravelerId::Keeper => 0.03,   // Slow, steady
                TravelerId::Child => 0.12,    // Playful
                TravelerId::Other => 0.02,    // Barely moves
            },
            noise_seed: rng.gen::<f32>() * 1000.0,
            active: true,
        }
    }
}

/// Apply drift to traveler positions
pub fn apply_traveler_drift(
    time: Res<Time>,
    clock: Res<ExperienceClock>,
    mut travelers: Query<(&Traveler, &mut Transform, &mut TravelerDrift)>,
) {
    let t = time.elapsed_seconds();
    let phase = clock.phase();

    for (_traveler, mut transform, mut drift) in travelers.iter_mut() {
        if !drift.active {
            continue;
        }

        // Drift intensity based on phase
        let phase_multiplier = match phase {
            Phase::Awakening => 0.5,
            Phase::Discovery => 1.0,
            Phase::Connection => 0.3, // Less drift, more anchored
            Phase::Acceptance => 0.2,
            _ => 0.0,
        };

        if phase_multiplier < 0.01 {
            continue;
        }

        // Perlin-like noise for organic drift
        let noise_x =
            ((t + drift.noise_seed) * 0.3).sin() * ((t + drift.noise_seed) * 0.7).cos();
        let noise_y = ((t + drift.noise_seed + 100.0) * 0.2).sin() * 0.5;
        let noise_z = ((t + drift.noise_seed + 200.0) * 0.25).cos();

        drift.velocity = Vec3::new(noise_x, noise_y, noise_z) * drift.max_speed * phase_multiplier;

        // Apply velocity
        transform.translation += drift.velocity * time.delta_seconds();

        // Soft bounds - gently push back if too far from center
        let distance = transform.translation.length();
        if distance > 8.0 {
            let push_back = -transform.translation.normalize() * 0.01;
            transform.translation += push_back;
        }
    }
}
