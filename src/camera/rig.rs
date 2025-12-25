//! Camera rig - position, rotation, and offset management

#![allow(dead_code)]

use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;

use crate::post::{ChromaticAberrationSettings, FilmGrainSettings, VignetteSettings};

/// Marker component for the main experience camera
#[derive(Component)]
pub struct ExperienceCamera;

/// Camera rig that controls camera movement
#[derive(Component)]
pub struct CameraRig {
    /// Base position (before behaviors applied)
    pub base_position: Vec3,
    /// Base rotation (before behaviors applied)
    pub base_rotation: Quat,
    /// Current breathing offset
    pub breathing_offset: Vec3,
    /// Current behavior offset
    pub behavior_offset: Vec3,
    /// Current shake offset
    pub shake_offset: Vec3,
    /// Total accumulated rotation offset
    pub rotation_offset: Quat,
}

impl Default for CameraRig {
    fn default() -> Self {
        Self {
            base_position: Vec3::new(0.0, 0.0, 15.0), // Looking at origin from +Z
            base_rotation: Quat::IDENTITY,
            breathing_offset: Vec3::ZERO,
            behavior_offset: Vec3::ZERO,
            shake_offset: Vec3::ZERO,
            rotation_offset: Quat::IDENTITY,
        }
    }
}

impl CameraRig {
    /// Get the final camera position
    pub fn final_position(&self) -> Vec3 {
        self.base_position + self.breathing_offset + self.behavior_offset + self.shake_offset
    }

    /// Get the final camera rotation
    pub fn final_rotation(&self) -> Quat {
        self.base_rotation * self.rotation_offset
    }
}

/// Spawn the experience camera
pub fn spawn_camera(mut commands: Commands) {
    let rig = CameraRig::default();
    let position = rig.base_position;

    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::AcesFitted,
            transform: Transform::from_translation(position).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        BloomSettings {
            intensity: 0.15,
            low_frequency_boost: 0.7,
            low_frequency_boost_curvature: 0.95,
            high_pass_frequency: 1.0,
            composite_mode: bevy::core_pipeline::bloom::BloomCompositeMode::Additive,
            ..default()
        },
        ChromaticAberrationSettings::new(0.002), // Base intensity, updated dynamically
        VignetteSettings::new(0.3), // Base vignette intensity
        FilmGrainSettings::new(0.05), // Base grain intensity, updated dynamically
        ExperienceCamera,
        rig,
    ));

    info!(
        target: "lightwatch::camera",
        "Experience camera spawned at {:?}",
        position
    );
}

/// Apply rig offsets to camera transform
pub fn apply_rig_to_transform(
    mut cameras: Query<(&CameraRig, &mut Transform), With<ExperienceCamera>>,
) {
    for (rig, mut transform) in cameras.iter_mut() {
        let final_pos = rig.final_position();
        transform.translation = final_pos;

        // Look at origin with subtle rotation influence
        let look_offset = rig.rotation_offset * Vec3::new(0.0, 0.0, -1.0) * 0.1;
        let look_target = Vec3::ZERO + look_offset;
        transform.look_at(look_target, Vec3::Y);
    }
}
