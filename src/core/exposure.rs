//! Dynamic exposure control tied to experience phase

use bevy::prelude::*;
use bevy::render::camera::Exposure;

/// Dynamic exposure control tied to experience phase
#[derive(Resource)]
pub struct ExposureControl {
    /// Current exposure value (EV100)
    pub current: f32,
    /// Target exposure value (EV100)
    pub target: f32,
    /// Transition speed (units per second)
    pub speed: f32,
}

impl Default for ExposureControl {
    fn default() -> Self {
        Self {
            current: 10.0,
            target: 10.0,
            speed: 1.0,
        }
    }
}

impl ExposureControl {
    /// Get exposure value for each phase
    pub fn for_phase(phase: &str) -> f32 {
        match phase {
            "signal" => 0.5,      // Dark, waiting
            "bang" => 15.0,       // Blindingly bright at peak
            "awakening" => 8.0,   // Settling down
            "discovery" => 10.0,  // Normal exposure
            "connection" => 12.0, // Warm, bright
            "acceptance" => 6.0,  // Dimming toward darkness
            _ => 10.0,
        }
    }

    /// Set target exposure for a phase
    pub fn set_phase(&mut self, phase: &str) {
        self.target = Self::for_phase(phase);
    }

    /// Set target exposure with custom transition speed
    pub fn set_target(&mut self, target: f32, speed: f32) {
        self.target = target;
        self.speed = speed;
    }
}

/// Update exposure smoothly over time
pub fn update_exposure(
    mut control: ResMut<ExposureControl>,
    mut cameras: Query<&mut Exposure>,
    time: Res<Time>,
) {
    // Lerp current toward target
    let delta = (control.target - control.current) * control.speed * time.delta_seconds();
    control.current += delta;

    // Apply to all cameras
    for mut exposure in cameras.iter_mut() {
        exposure.ev100 = control.current;
    }
}
