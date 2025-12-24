//! Custom WGSL shader materials for travelers

use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

use super::TravelerPulse;

/// Custom traveler glow material - inner glow, Fresnel rim, pulse
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct TravelerGlowMaterial {
    #[uniform(0)]
    pub base_color: LinearRgba,
    #[uniform(0)]
    pub emissive: LinearRgba,
    #[uniform(0)]
    pub pulse_intensity: f32,
    #[uniform(0)]
    pub pulse_phase: f32,
    #[uniform(0)]
    pub time: f32,
    #[uniform(0)]
    pub fresnel_power: f32,
    #[uniform(0)]
    pub inner_glow_strength: f32,
    #[uniform(0)]
    pub rim_color: LinearRgba,
    #[uniform(0)]
    pub grief_amount: f32,
    #[uniform(0)]
    pub _padding: Vec3,
}

impl Default for TravelerGlowMaterial {
    fn default() -> Self {
        Self {
            base_color: LinearRgba::new(0.91, 0.64, 0.27, 1.0),
            emissive: LinearRgba::new(0.91, 0.64, 0.27, 1.0),
            pulse_intensity: 0.3,
            pulse_phase: 0.0,
            time: 0.0,
            fresnel_power: 3.0,
            inner_glow_strength: 0.5,
            rim_color: LinearRgba::new(1.0, 0.9, 0.8, 1.0),
            grief_amount: 0.0,
            _padding: Vec3::ZERO,
        }
    }
}

impl Material for TravelerGlowMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/traveler_glow.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

/// Shell material - translucent outer layer
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct TravelerShellMaterial {
    #[uniform(0)]
    pub base_color: LinearRgba,
    #[uniform(0)]
    pub refraction_strength: f32,
    #[uniform(0)]
    pub thickness: f32,
    #[uniform(0)]
    pub ior: f32,
    #[uniform(0)]
    pub pulse_intensity: f32,
    #[uniform(0)]
    pub time: f32,
    #[uniform(0)]
    pub _padding: Vec3,
}

impl Default for TravelerShellMaterial {
    fn default() -> Self {
        Self {
            base_color: LinearRgba::new(1.0, 1.0, 1.0, 0.3),
            refraction_strength: 0.1,
            thickness: 0.5,
            ior: 1.5,
            pulse_intensity: 0.2,
            time: 0.0,
            _padding: Vec3::ZERO,
        }
    }
}

impl Material for TravelerShellMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/traveler_shell.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

/// Edge material - glowing wireframe
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct TravelerEdgeMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
    #[uniform(0)]
    pub glow_intensity: f32,
    #[uniform(0)]
    pub pulse_phase: f32,
    #[uniform(0)]
    pub time: f32,
    #[uniform(0)]
    pub line_width: f32,
}

impl Default for TravelerEdgeMaterial {
    fn default() -> Self {
        Self {
            color: LinearRgba::new(1.0, 0.8, 0.5, 1.0),
            glow_intensity: 1.0,
            pulse_phase: 0.0,
            time: 0.0,
            line_width: 2.0,
        }
    }
}

impl Material for TravelerEdgeMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/traveler_edge.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Add
    }
}

/// Update time uniform for all traveler materials
pub fn update_shader_time(
    time: Res<Time>,
    mut glow_materials: ResMut<Assets<TravelerGlowMaterial>>,
    mut shell_materials: ResMut<Assets<TravelerShellMaterial>>,
    mut edge_materials: ResMut<Assets<TravelerEdgeMaterial>>,
) {
    let t = time.elapsed_seconds();

    for (_, material) in glow_materials.iter_mut() {
        material.time = t;
    }

    for (_, material) in shell_materials.iter_mut() {
        material.time = t;
    }

    for (_, material) in edge_materials.iter_mut() {
        material.time = t;
    }
}

/// Sync pulse phase from TravelerPulse to shader materials
pub fn sync_pulse_to_shader_materials(
    travelers: Query<(&TravelerPulse, &Children)>,
    glow_handles: Query<&Handle<TravelerGlowMaterial>>,
    edge_handles: Query<&Handle<TravelerEdgeMaterial>>,
    mut glow_materials: ResMut<Assets<TravelerGlowMaterial>>,
    mut edge_materials: ResMut<Assets<TravelerEdgeMaterial>>,
) {
    for (pulse, children) in travelers.iter() {
        for &child in children.iter() {
            if let Ok(handle) = glow_handles.get(child) {
                if let Some(material) = glow_materials.get_mut(handle) {
                    material.pulse_phase = pulse.phase * std::f32::consts::TAU;
                    material.pulse_intensity = 0.3 + pulse.intensity * 0.4;
                }
            }

            if let Ok(handle) = edge_handles.get(child) {
                if let Some(material) = edge_materials.get_mut(handle) {
                    material.pulse_phase = pulse.phase * std::f32::consts::TAU;
                    material.glow_intensity = 0.7 + pulse.intensity * 0.5;
                }
            }
        }
    }
}

/// Plugin for traveler shader materials
pub struct TravelerShaderPlugin;

impl Plugin for TravelerShaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<TravelerGlowMaterial>::default())
            .add_plugins(MaterialPlugin::<TravelerShellMaterial>::default())
            .add_plugins(MaterialPlugin::<TravelerEdgeMaterial>::default())
            .add_systems(Update, (update_shader_time, sync_pulse_to_shader_materials));
    }
}
