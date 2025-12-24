//! Renderer configuration for HDR and tonemapping

#![allow(dead_code)]

use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy::render::camera::Exposure;

/// Camera bundle with HDR and tonemapping configured for cinematic rendering
#[derive(Bundle)]
pub struct CinematicCameraBundle {
    pub camera: Camera3d,
    pub camera_settings: Camera,
    pub transform: Transform,
    pub tonemapping: Tonemapping,
    pub exposure: Exposure,
    pub bloom: BloomSettings,
}

impl Default for CinematicCameraBundle {
    fn default() -> Self {
        Self {
            camera: Camera3d::default(),
            camera_settings: Camera {
                hdr: true,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            tonemapping: Tonemapping::AcesFitted,
            exposure: Exposure::SUNLIGHT,
            bloom: BloomSettings::NATURAL,
        }
    }
}

/// Configure MSAA globally (4x for good balance)
pub fn configure_msaa() -> Msaa {
    Msaa::Sample4
}
