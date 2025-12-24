//! Camera configuration constants

#![allow(dead_code)]

use bevy::prelude::*;

/// Global camera configuration
#[derive(Resource)]
pub struct CameraConfig {
    /// Near clip plane
    pub near: f32,
    /// Far clip plane
    pub far: f32,
    /// Field of view in degrees
    pub fov: f32,
    /// Default distance from origin
    pub default_distance: f32,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            near: 0.1,
            far: 1000.0,
            fov: 45.0,
            default_distance: 15.0,
        }
    }
}
