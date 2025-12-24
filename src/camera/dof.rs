//! Depth of field system - focus control and blur settings

#![allow(dead_code)]

use bevy::prelude::*;

use crate::core::{CameraFocusEvent, ExperienceClock, Phase};

/// Depth of field configuration
#[derive(Resource)]
pub struct DepthOfFieldSettings {
    /// Focus distance from camera
    pub focus_distance: f32,
    /// Target focus distance (for lerping)
    pub target_focus: f32,
    /// Focus transition speed
    pub focus_speed: f32,
    /// Aperture (f-stop) - lower = more blur
    pub aperture: f32,
    /// Maximum blur radius in pixels
    pub max_blur: f32,
    /// Near blur distance
    pub near_blur_distance: f32,
    /// Far blur distance
    pub far_blur_distance: f32,
    /// Is DOF enabled?
    pub enabled: bool,
}

impl Default for DepthOfFieldSettings {
    fn default() -> Self {
        Self {
            focus_distance: 15.0,
            target_focus: 15.0,
            focus_speed: 2.0,
            aperture: 2.8,
            max_blur: 8.0,
            near_blur_distance: 5.0,
            far_blur_distance: 30.0,
            enabled: true,
        }
    }
}

impl DepthOfFieldSettings {
    /// Focus on a specific distance
    pub fn set_focus(&mut self, distance: f32) {
        self.target_focus = distance.max(0.1);
    }

    /// Focus on a world position relative to camera
    pub fn focus_on_position(&mut self, camera_pos: Vec3, target_pos: Vec3) {
        self.target_focus = (target_pos - camera_pos).length();
    }

    /// Get blur amount for a given depth
    pub fn blur_for_depth(&self, depth: f32) -> f32 {
        let distance_from_focus = (depth - self.focus_distance).abs();
        let blur_range = if depth < self.focus_distance {
            self.near_blur_distance
        } else {
            self.far_blur_distance
        };

        let normalized = (distance_from_focus / blur_range).min(1.0);
        normalized * self.max_blur
    }
}

/// Update DOF focus based on phase
pub fn update_dof_for_phase(clock: Res<ExperienceClock>, mut settings: ResMut<DepthOfFieldSettings>) {
    let target = match clock.phase() {
        Phase::Signal => 15.0,     // Normal distance
        Phase::Bang => 5.0,        // Close focus for intensity
        Phase::Awakening => 12.0,  // Medium
        Phase::Discovery => 10.0,  // Closer as we explore
        Phase::Connection => 8.0,  // Sharp focus on travelers
        Phase::Acceptance => 25.0, // Pull back, softer focus
        Phase::Ended => 50.0,      // Very soft
    };

    settings.target_focus = target;
}

/// Handle explicit focus events
pub fn handle_focus_events(
    mut settings: ResMut<DepthOfFieldSettings>,
    mut events: EventReader<CameraFocusEvent>,
) {
    for event in events.read() {
        settings.target_focus = event.distance;
    }
}

/// Smoothly interpolate focus distance
pub fn interpolate_focus(time: Res<Time>, mut settings: ResMut<DepthOfFieldSettings>) {
    let diff = settings.target_focus - settings.focus_distance;
    if diff.abs() > 0.01 {
        settings.focus_distance += diff * settings.focus_speed * time.delta_seconds();
    }
}

/// DOF shader uniform (for future render node integration)
#[derive(Clone, Copy)]
pub struct DofSettingsUniform {
    pub focus_distance: f32,
    pub aperture: f32,
    pub max_blur: f32,
    pub near_blur_distance: f32,
    pub far_blur_distance: f32,
    pub enabled: f32,
    pub _padding: [f32; 2],
}

impl From<&DepthOfFieldSettings> for DofSettingsUniform {
    fn from(settings: &DepthOfFieldSettings) -> Self {
        Self {
            focus_distance: settings.focus_distance,
            aperture: settings.aperture,
            max_blur: settings.max_blur,
            near_blur_distance: settings.near_blur_distance,
            far_blur_distance: settings.far_blur_distance,
            enabled: if settings.enabled { 1.0 } else { 0.0 },
            _padding: [0.0; 2],
        }
    }
}
