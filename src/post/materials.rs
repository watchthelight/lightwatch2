//! Post-processing material definitions
//!
//! These materials define the shader interfaces for custom post-processing effects.
//! Full render graph integration requires render nodes (complex for Bevy 0.14).
//! Values are tracked in DynamicPostProcess for when render integration is added.

use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::Material2d;

/// Chromatic aberration material (for 2D post-processing quad)
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct ChromaticAberrationMaterial {
    #[uniform(0)]
    pub intensity: f32,
    #[uniform(0)]
    pub center_x: f32,
    #[uniform(0)]
    pub center_y: f32,
    #[uniform(0)]
    pub _padding: f32,
}

impl Default for ChromaticAberrationMaterial {
    fn default() -> Self {
        Self {
            intensity: 0.002,
            center_x: 0.5,
            center_y: 0.5,
            _padding: 0.0,
        }
    }
}

impl Material2d for ChromaticAberrationMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/chromatic_aberration.wgsl".into()
    }
}

/// Film grain material
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct FilmGrainMaterial {
    #[uniform(0)]
    pub intensity: f32,
    #[uniform(0)]
    pub time: f32,
    #[uniform(0)]
    pub response: f32,
    #[uniform(0)]
    pub _padding: f32,
}

impl Default for FilmGrainMaterial {
    fn default() -> Self {
        Self {
            intensity: 0.03,
            time: 0.0,
            response: 0.5,
            _padding: 0.0,
        }
    }
}

impl Material2d for FilmGrainMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/film_grain.wgsl".into()
    }
}

/// Vignette material
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct VignetteMaterial {
    #[uniform(0)]
    pub intensity: f32,
    #[uniform(0)]
    pub midpoint: f32,
    #[uniform(0)]
    pub softness: f32,
    #[uniform(0)]
    pub _padding: f32,
}

impl Default for VignetteMaterial {
    fn default() -> Self {
        Self {
            intensity: 0.3,
            midpoint: 0.4,
            softness: 0.3,
            _padding: 0.0,
        }
    }
}

impl Material2d for VignetteMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/vignette.wgsl".into()
    }
}
