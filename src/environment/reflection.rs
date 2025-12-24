//! Reflection plane - Appears during Connection phase

#![allow(dead_code)]

use bevy::prelude::*;

use crate::camera::ExperienceCamera;
use crate::core::{ExperienceClock, Phase};

/// Reflection plane configuration
#[derive(Resource)]
pub struct ReflectionConfig {
    /// Y position of reflection plane
    pub height: f32,
    /// Plane size
    pub size: f32,
    /// Maximum opacity
    pub max_opacity: f32,
    /// Blur amount (0 = sharp, 1 = very blurred)
    pub blur: f32,
    /// Reflection strength falloff with distance
    pub falloff: f32,
}

impl Default for ReflectionConfig {
    fn default() -> Self {
        Self {
            height: -2.0,
            size: 30.0,
            max_opacity: 0.4,
            blur: 0.3,
            falloff: 0.1,
        }
    }
}

/// Current reflection state
#[derive(Resource, Default)]
pub struct ReflectionState {
    pub opacity: f32,
    pub target_opacity: f32,
    pub active: bool,
}

/// Marker for reflection plane entity
#[derive(Component)]
pub struct ReflectionPlane;

/// Marker for reflection camera (for advanced implementation)
#[derive(Component)]
#[allow(dead_code)]
pub struct ReflectionCamera;

/// Spawn the reflection plane
pub fn spawn_reflection_plane(
    mut commands: Commands,
    config: Res<ReflectionConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create plane mesh
    let mesh = meshes.add(Plane3d::default().mesh().size(config.size, config.size));

    // Reflective material (glass-like)
    let material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.1, 0.12, 0.15, 0.0), // Start invisible
        metallic: 0.9,
        perceptual_roughness: config.blur,
        reflectance: 0.8,
        alpha_mode: AlphaMode::Blend,
        double_sided: true,
        cull_mode: None,
        ..default()
    });

    commands.spawn((
        PbrBundle {
            mesh,
            material,
            transform: Transform::from_xyz(0.0, config.height, 0.0),
            visibility: Visibility::Hidden,
            ..default()
        },
        ReflectionPlane,
        Name::new("Reflection Plane"),
    ));

    info!(target: "lightwatch::environment", "Spawned reflection plane at y={}", config.height);
}

/// Update reflection visibility based on phase
pub fn update_reflection_for_phase(
    clock: Res<ExperienceClock>,
    config: Res<ReflectionConfig>,
    mut state: ResMut<ReflectionState>,
) {
    let phase = clock.phase();
    let progress = clock.phase_progress();

    match phase {
        Phase::Connection => {
            state.active = true;
            // Fade in during first 20% of Connection
            state.target_opacity = if progress < 0.2 {
                (progress / 0.2) * config.max_opacity
            } else {
                config.max_opacity
            };
        }
        Phase::Acceptance => {
            // Fade out during first 30% of Acceptance
            if progress < 0.3 {
                state.target_opacity = config.max_opacity * (1.0 - progress / 0.3);
            } else {
                state.target_opacity = 0.0;
                state.active = false;
            }
        }
        _ => {
            state.target_opacity = 0.0;
            state.active = false;
        }
    }
}

/// Interpolate reflection opacity
pub fn interpolate_reflection(
    time: Res<Time>,
    mut state: ResMut<ReflectionState>,
    mut planes: Query<(&mut Visibility, &Handle<StandardMaterial>), With<ReflectionPlane>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Lerp opacity
    state.opacity += (state.target_opacity - state.opacity) * 2.0 * time.delta_seconds();

    for (mut visibility, material_handle) in planes.iter_mut() {
        // Show/hide based on opacity
        *visibility = if state.opacity > 0.01 {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };

        // Update material
        if let Some(material) = materials.get_mut(material_handle) {
            material.base_color = Color::srgba(0.1, 0.12, 0.15, state.opacity);
        }
    }
}

/// Setup reflection camera (advanced implementation - for true planar reflections)
#[allow(dead_code)]
pub fn setup_reflection_camera(
    mut commands: Commands,
    config: Res<ReflectionConfig>,
    main_camera: Query<&Transform, With<ExperienceCamera>>,
) {
    let Ok(main_transform) = main_camera.get_single() else {
        return;
    };

    // Create reflected transform
    let reflected_pos = Vec3::new(
        main_transform.translation.x,
        2.0 * config.height - main_transform.translation.y,
        main_transform.translation.z,
    );

    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                is_active: false, // Only enable when reflection is visible
                ..default()
            },
            transform: Transform::from_translation(reflected_pos).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        ReflectionCamera,
    ));
}

/// Reflection plugin
pub struct ReflectionPlugin;

impl Plugin for ReflectionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ReflectionConfig>()
            .init_resource::<ReflectionState>()
            .add_systems(Startup, spawn_reflection_plane)
            .add_systems(
                Update,
                (
                    update_reflection_for_phase,
                    interpolate_reflection.after(update_reflection_for_phase),
                ),
            );
    }
}
