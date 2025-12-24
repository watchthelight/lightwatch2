//! Shockwave - Expanding torus with refraction effect

use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::PrimitiveTopology;

use crate::camera::ExperienceCamera;
use crate::core::ExperienceClock;

/// Shockwave state
#[derive(Component, Debug)]
pub struct Shockwave {
    /// Current radius
    pub radius: f32,
    /// Maximum radius
    pub max_radius: f32,
    /// Current thickness
    pub thickness: f32,
    /// Expansion speed
    #[allow(dead_code)]
    pub speed: f32,
    /// Refraction strength
    #[allow(dead_code)]
    pub refraction: f32,
    /// Time alive
    pub age: f32,
    /// Lifetime
    pub lifetime: f32,
}

impl Default for Shockwave {
    fn default() -> Self {
        Self {
            radius: 0.1,
            max_radius: 100.0,
            thickness: 0.5,
            speed: 30.0,
            refraction: 0.1,
            age: 0.0,
            lifetime: 4.0,
        }
    }
}

/// Shockwave configuration
#[derive(Resource)]
pub struct ShockwaveConfig {
    pub spawn_time: f32, // When shockwave spawns (at peak)
    pub initial_speed: f32,
    pub max_radius: f32,
    pub refraction_strength: f32,
}

impl Default for ShockwaveConfig {
    fn default() -> Self {
        Self {
            spawn_time: 4.0, // At bang peak
            initial_speed: 40.0,
            max_radius: 150.0,
            refraction_strength: 0.15,
        }
    }
}

/// Shockwave spawn state
#[derive(Resource, Default)]
pub struct ShockwaveState {
    pub spawned: bool,
}

/// Create torus mesh for shockwave
pub fn create_shockwave_mesh(
    major_radius: f32,
    minor_radius: f32,
    major_segments: usize,
    minor_segments: usize,
) -> Mesh {
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut indices = Vec::new();

    for i in 0..=major_segments {
        let major_angle = (i as f32 / major_segments as f32) * std::f32::consts::TAU;
        let cos_major = major_angle.cos();
        let sin_major = major_angle.sin();

        for j in 0..=minor_segments {
            let minor_angle = (j as f32 / minor_segments as f32) * std::f32::consts::TAU;
            let cos_minor = minor_angle.cos();
            let sin_minor = minor_angle.sin();

            // Position
            let x = (major_radius + minor_radius * cos_minor) * cos_major;
            let y = minor_radius * sin_minor;
            let z = (major_radius + minor_radius * cos_minor) * sin_major;
            positions.push([x, y, z]);

            // Normal
            let nx = cos_minor * cos_major;
            let ny = sin_minor;
            let nz = cos_minor * sin_major;
            normals.push([nx, ny, nz]);

            // UV
            uvs.push([
                i as f32 / major_segments as f32,
                j as f32 / minor_segments as f32,
            ]);
        }
    }

    // Indices
    for i in 0..major_segments {
        for j in 0..minor_segments {
            let a = i * (minor_segments + 1) + j;
            let b = a + 1;
            let c = a + minor_segments + 1;
            let d = c + 1;

            indices.extend_from_slice(&[
                a as u32, c as u32, b as u32, b as u32, c as u32, d as u32,
            ]);
        }
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

/// Spawn shockwave at peak
pub fn spawn_shockwave(
    mut commands: Commands,
    clock: Res<ExperienceClock>,
    config: Res<ShockwaveConfig>,
    mut state: ResMut<ShockwaveState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if state.spawned {
        return;
    }

    if clock.elapsed() < config.spawn_time {
        return;
    }

    let mesh = meshes.add(create_shockwave_mesh(1.0, 0.3, 64, 16));

    // Transparent, refractive material
    let material = materials.add(StandardMaterial {
        base_color: Color::srgba(1.0, 0.95, 0.9, 0.3),
        emissive: LinearRgba::new(0.5, 0.4, 0.3, 1.0),
        alpha_mode: AlphaMode::Blend,
        specular_transmission: 0.8,
        thickness: 0.2,
        ior: 1.1,
        double_sided: true,
        cull_mode: None,
        ..default()
    });

    commands.spawn((
        PbrBundle {
            mesh,
            material,
            transform: Transform::from_scale(Vec3::splat(0.1)),
            ..default()
        },
        Shockwave {
            speed: config.initial_speed,
            max_radius: config.max_radius,
            refraction: config.refraction_strength,
            ..default()
        },
        Name::new("Shockwave"),
    ));

    state.spawned = true;

    info!(target: "lightwatch::bang", "Shockwave spawned at {:.2}s", clock.elapsed());
}

/// Update shockwave expansion
pub fn update_shockwave(
    mut commands: Commands,
    time: Res<Time>,
    camera: Query<&GlobalTransform, With<ExperienceCamera>>,
    mut shockwaves: Query<(
        Entity,
        &mut Shockwave,
        &mut Transform,
        &Handle<StandardMaterial>,
    )>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let camera_pos = camera
        .get_single()
        .map(|t| t.translation())
        .unwrap_or(Vec3::ZERO);

    for (entity, mut wave, mut transform, material_handle) in shockwaves.iter_mut() {
        // Update age
        wave.age += time.delta_seconds();

        // Calculate expansion
        let progress = wave.age / wave.lifetime;
        let eased = ease_out_quad(progress);

        // Expand radius
        wave.radius = eased * wave.max_radius;

        // Thickness decreases as it expands
        wave.thickness = 0.5 * (1.0 - eased * 0.8);

        // Update scale
        transform.scale = Vec3::splat(wave.radius);

        // Check if passing through camera
        let camera_dist = camera_pos.length(); // Distance from origin
        let passing_camera = (wave.radius - camera_dist).abs() < wave.thickness * wave.radius;

        // Update material
        if let Some(material) = materials.get_mut(material_handle) {
            let opacity = (1.0 - progress) * 0.4;

            // Increase refraction when passing camera
            let refraction_mult = if passing_camera { 2.0 } else { 1.0 };

            material.base_color = Color::srgba(1.0, 0.95, 0.9, opacity);
            material.emissive = LinearRgba::new(
                0.5 * opacity * refraction_mult,
                0.4 * opacity * refraction_mult,
                0.3 * opacity * refraction_mult,
                1.0,
            );
        }

        // Despawn when complete
        if progress >= 1.0 {
            commands.entity(entity).despawn();
        }
    }
}

/// Quadratic ease out
fn ease_out_quad(t: f32) -> f32 {
    1.0 - (1.0 - t) * (1.0 - t)
}

/// Reset shockwave state
pub fn reset_shockwave_state(clock: Res<ExperienceClock>, mut state: ResMut<ShockwaveState>) {
    // Reset when jumping to before spawn time
    if clock.elapsed() < 2.0 && state.spawned {
        state.spawned = false;
    }
}

/// Shockwave plugin
pub struct ShockwavePlugin;

impl Plugin for ShockwavePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ShockwaveConfig>()
            .init_resource::<ShockwaveState>()
            .add_systems(
                Update,
                (spawn_shockwave, update_shockwave, reset_shockwave_state),
            );
    }
}
