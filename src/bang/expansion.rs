//! Bang expansion rings - Radiating shockwaves from core

use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::PrimitiveTopology;

use super::BangConfig;
use crate::core::ExperienceClock;

/// Individual expansion ring
#[derive(Component, Debug)]
pub struct ExpansionRing {
    /// Time when ring was created
    pub spawn_time: f32,
    /// Current expansion (0-1)
    pub expansion: f32,
    /// Ring index (for staggered timing)
    #[allow(dead_code)]
    pub index: usize,
}

/// Ring system configuration
#[derive(Resource)]
pub struct ExpansionConfig {
    pub max_rings: usize,
    pub ring_spawn_interval: f32,
    pub expansion_duration: f32,
    pub max_radius: f32,
}

impl Default for ExpansionConfig {
    fn default() -> Self {
        Self {
            max_rings: 5,
            ring_spawn_interval: 0.2,
            expansion_duration: 3.0,
            max_radius: 80.0,
        }
    }
}

/// Ring spawn tracker
#[derive(Resource, Default)]
pub struct RingSpawnState {
    pub rings_spawned: usize,
    pub last_spawn_time: f32,
    pub spawning_active: bool,
}

/// Create ring mesh
pub fn create_ring_mesh(inner_radius: f32, outer_radius: f32, segments: usize) -> Mesh {
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut indices = Vec::new();

    for i in 0..=segments {
        let angle = (i as f32 / segments as f32) * std::f32::consts::TAU;
        let cos = angle.cos();
        let sin = angle.sin();

        // Inner vertex
        positions.push([inner_radius * cos, 0.0, inner_radius * sin]);
        normals.push([0.0, 1.0, 0.0]);
        uvs.push([i as f32 / segments as f32, 0.0]);

        // Outer vertex
        positions.push([outer_radius * cos, 0.0, outer_radius * sin]);
        normals.push([0.0, 1.0, 0.0]);
        uvs.push([i as f32 / segments as f32, 1.0]);

        if i < segments {
            let base = (i * 2) as u32;
            indices.extend_from_slice(&[base, base + 1, base + 2, base + 1, base + 3, base + 2]);
        }
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

/// Spawn expansion rings during bang
pub fn spawn_expansion_rings(
    mut commands: Commands,
    clock: Res<ExperienceClock>,
    bang_config: Res<BangConfig>,
    config: Res<ExpansionConfig>,
    mut state: ResMut<RingSpawnState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let elapsed = clock.elapsed();

    // Only spawn during expansion phase
    if elapsed < bang_config.expansion_start || elapsed > bang_config.peak_time + 1.0 {
        state.spawning_active = false;
        return;
    }

    state.spawning_active = true;

    // Check if we should spawn a new ring
    if state.rings_spawned >= config.max_rings {
        return;
    }

    if elapsed - state.last_spawn_time < config.ring_spawn_interval {
        return;
    }

    // Spawn ring
    let mesh = meshes.add(create_ring_mesh(0.1, 0.15, 64));

    let material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.91, 0.64, 0.27, 0.6),
        emissive: LinearRgba::new(0.91, 0.64, 0.27, 1.0),
        unlit: true,
        alpha_mode: AlphaMode::Add,
        double_sided: true,
        cull_mode: None,
        ..default()
    });

    commands.spawn((
        PbrBundle {
            mesh,
            material,
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..default()
        },
        ExpansionRing {
            spawn_time: elapsed,
            expansion: 0.0,
            index: state.rings_spawned,
        },
        Name::new(format!("Expansion Ring {}", state.rings_spawned)),
    ));

    state.rings_spawned += 1;
    state.last_spawn_time = elapsed;

    info!(target: "lightwatch::bang", "Spawned expansion ring {}", state.rings_spawned);
}

/// Update expansion rings
pub fn update_expansion_rings(
    mut commands: Commands,
    clock: Res<ExperienceClock>,
    config: Res<ExpansionConfig>,
    mut rings: Query<(
        Entity,
        &mut ExpansionRing,
        &mut Transform,
        &Handle<StandardMaterial>,
    )>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let elapsed = clock.elapsed();

    for (entity, mut ring, mut transform, material_handle) in rings.iter_mut() {
        let ring_elapsed = elapsed - ring.spawn_time;
        ring.expansion = (ring_elapsed / config.expansion_duration).min(1.0);

        // Ease out for natural deceleration
        let eased = ease_out_expo(ring.expansion);

        // Scale
        let scale = 1.0 + eased * config.max_radius;
        transform.scale = Vec3::splat(scale);

        // Opacity decreases with expansion
        let opacity = (1.0 - ring.expansion * 0.8).max(0.0);

        // Color cools with expansion
        let temperature = 1.0 - ring.expansion * 0.6;
        let color = temperature_to_color(temperature);

        if let Some(material) = materials.get_mut(material_handle) {
            material.base_color = Color::srgba(color.x, color.y, color.z, opacity * 0.6);
            material.emissive =
                LinearRgba::new(color.x * opacity, color.y * opacity, color.z * opacity, 1.0);
        }

        // Despawn when fully expanded and faded
        if ring.expansion >= 1.0 {
            commands.entity(entity).despawn();
        }
    }
}

/// Exponential ease out
fn ease_out_expo(t: f32) -> f32 {
    if t >= 1.0 {
        1.0
    } else {
        1.0 - 2.0_f32.powf(-10.0 * t)
    }
}

/// Convert temperature (0-1) to color
fn temperature_to_color(temp: f32) -> Vec3 {
    let amber = Vec3::new(0.91, 0.64, 0.27);
    let red = Vec3::new(0.6, 0.15, 0.08);
    amber.lerp(red, 1.0 - temp)
}

/// Reset ring state when bang completes
pub fn reset_ring_state(
    clock: Res<ExperienceClock>,
    bang_config: Res<BangConfig>,
    mut state: ResMut<RingSpawnState>,
) {
    if clock.elapsed() > bang_config.complete_time + 1.0 && state.rings_spawned > 0 {
        state.rings_spawned = 0;
        state.last_spawn_time = 0.0;
        state.spawning_active = false;
    }
}

/// Expansion rings plugin
pub struct ExpansionPlugin;

impl Plugin for ExpansionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ExpansionConfig>()
            .init_resource::<RingSpawnState>()
            .add_systems(
                Update,
                (spawn_expansion_rings, update_expansion_rings, reset_ring_state),
            );
    }
}
