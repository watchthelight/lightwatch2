//! Custom material definitions for LIGHTWATCH

#![allow(dead_code)]

use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};

/// Traveler glow material - emissive pulsing effect
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct TravelerGlowMaterial {
    /// Base color of the traveler
    #[uniform(0)]
    pub base_color: LinearRgba,
    /// Intensity of the pulse effect (0.0 - 1.0)
    #[uniform(0)]
    pub pulse_intensity: f32,
    /// Current time for animation
    #[uniform(0)]
    pub time: f32,
    /// Phase offset for pulse synchronization
    #[uniform(0)]
    pub pulse_phase: f32,
}

impl Default for TravelerGlowMaterial {
    fn default() -> Self {
        Self {
            base_color: LinearRgba::WHITE,
            pulse_intensity: 0.5,
            time: 0.0,
            pulse_phase: 0.0,
        }
    }
}

impl Material for TravelerGlowMaterial {
    fn fragment_shader() -> ShaderRef {
        // Will be replaced with embedded shader handle
        ShaderRef::Default
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

impl TravelerGlowMaterial {
    /// Create a new traveler glow material with the specified color
    pub fn new(color: Color, pulse_intensity: f32) -> Self {
        Self {
            base_color: color.into(),
            pulse_intensity,
            time: 0.0,
            pulse_phase: 0.0,
        }
    }

    /// Create material for a specific traveler
    pub fn for_traveler(traveler_id: &str) -> Self {
        let (color, phase) = match traveler_id {
            "archivist" => (Color::srgb(1.0, 0.702, 0.278), 0.0),    // Amber
            "wanderer" => (Color::srgb(0.0, 0.808, 0.820), 0.25),    // Cyan
            "keeper" => (Color::srgb(1.0, 0.420, 0.208), 0.5),       // Orange
            "child" => (Color::srgb(1.0, 1.0, 1.0), 0.75),           // White
            "other" => (Color::srgb(0.545, 0.361, 0.965), 1.0),      // Violet
            _ => (Color::WHITE, 0.0),
        };

        Self {
            base_color: color.into(),
            pulse_intensity: 0.5,
            time: 0.0,
            pulse_phase: phase,
        }
    }
}
