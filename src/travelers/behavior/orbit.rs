//! Orbit behavior - orbital motion for travelers

use bevy::prelude::*;
use rand::Rng;

use crate::core::TravelerId;

/// Orbital motion component
#[derive(Component, Debug)]
pub struct TravelerOrbit {
    /// Center of orbit
    pub center: Vec3,
    /// Current orbital angle
    pub angle: f32,
    /// Orbital radius
    pub radius: f32,
    /// Orbital speed (radians per second)
    pub speed: f32,
    /// Orbital plane tilt
    pub tilt: Quat,
    /// Is orbiting active?
    pub active: bool,
}

impl TravelerOrbit {
    pub fn for_traveler(id: TravelerId) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            center: Vec3::ZERO,
            angle: rng.gen::<f32>() * std::f32::consts::TAU,
            radius: match id {
                TravelerId::Archivist => 3.0,
                TravelerId::Wanderer => 4.0,
                TravelerId::Keeper => 2.5,
                TravelerId::Child => 2.0,
                TravelerId::Other => 6.0,
            },
            speed: match id {
                TravelerId::Archivist => 0.05,
                TravelerId::Wanderer => 0.08,
                TravelerId::Keeper => 0.03,
                TravelerId::Child => 0.1,
                TravelerId::Other => 0.02,
            },
            tilt: Quat::from_euler(
                EulerRot::XYZ,
                (rng.gen::<f32>() - 0.5) * 0.3,
                0.0,
                (rng.gen::<f32>() - 0.5) * 0.2,
            ),
            active: false,
        }
    }
}

/// Apply orbital motion
pub fn apply_orbit_behavior(time: Res<Time>, mut travelers: Query<(&mut Transform, &mut TravelerOrbit)>) {
    for (mut transform, mut orbit) in travelers.iter_mut() {
        if !orbit.active {
            continue;
        }

        // Update angle
        orbit.angle += orbit.speed * time.delta_seconds();

        // Calculate position on orbital plane
        let local_pos = Vec3::new(
            orbit.angle.cos() * orbit.radius,
            0.0,
            orbit.angle.sin() * orbit.radius,
        );

        // Apply tilt and center
        transform.translation = orbit.center + orbit.tilt * local_pos;
    }
}
