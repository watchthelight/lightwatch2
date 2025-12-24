//! Post-processing configuration

use bevy::prelude::*;

/// Post-processing master configuration
#[derive(Resource)]
pub struct PostProcessConfig {
    pub enabled: bool,
    pub bloom: BloomConfig,
    pub chromatic_aberration: ChromaticAberrationConfig,
    pub grain: FilmGrainConfig,
    pub vignette: VignetteConfig,
}

impl Default for PostProcessConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            bloom: BloomConfig::default(),
            chromatic_aberration: ChromaticAberrationConfig::default(),
            grain: FilmGrainConfig::default(),
            vignette: VignetteConfig::default(),
        }
    }
}

/// Bloom configuration
#[derive(Clone)]
pub struct BloomConfig {
    pub enabled: bool,
    pub base_intensity: f32,
    pub threshold: f32,
    pub soft_threshold: f32,
    pub composite_mode: BloomCompositeMode,
}

impl Default for BloomConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            base_intensity: 0.15,
            threshold: 0.9,
            soft_threshold: 0.5,
            composite_mode: BloomCompositeMode::Additive,
        }
    }
}

#[derive(Clone, Copy, Default)]
pub enum BloomCompositeMode {
    #[default]
    Additive,
    EnergyConserving,
}

/// Chromatic aberration configuration
#[derive(Clone)]
pub struct ChromaticAberrationConfig {
    pub enabled: bool,
    pub base_intensity: f32,
    pub max_intensity: f32,
}

impl Default for ChromaticAberrationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            base_intensity: 0.002,
            max_intensity: 0.015,
        }
    }
}

/// Film grain configuration
#[derive(Clone)]
pub struct FilmGrainConfig {
    pub enabled: bool,
    pub base_intensity: f32,
    pub response: f32, // How much grain responds to brightness
}

impl Default for FilmGrainConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            base_intensity: 0.03,
            response: 0.5,
        }
    }
}

/// Vignette configuration
#[derive(Clone)]
pub struct VignetteConfig {
    pub enabled: bool,
    pub base_intensity: f32,
    pub midpoint: f32,
    pub softness: f32,
}

impl Default for VignetteConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            base_intensity: 0.3,
            midpoint: 0.4,
            softness: 0.3,
        }
    }
}
