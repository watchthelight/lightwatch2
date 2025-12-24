//! Shader loading system - loads embedded shaders at startup

use bevy::prelude::*;
use bevy::render::render_resource::Shader;

use super::sources;

/// Resource holding all loaded shader handles
#[derive(Resource, Default)]
pub struct ShaderHandles {
    pub traveler_glow: Handle<Shader>,
    pub nebula: Handle<Shader>,
    pub star_twinkle: Handle<Shader>,
    pub post_chromatic: Handle<Shader>,
    pub post_grain: Handle<Shader>,
    pub post_vignette: Handle<Shader>,
    pub bang_core: Handle<Shader>,
    pub bang_shockwave: Handle<Shader>,
    pub bang_godray: Handle<Shader>,
}

/// Load all embedded shaders at startup
pub fn load_embedded_shaders(
    mut shaders: ResMut<Assets<Shader>>,
    mut handles: ResMut<ShaderHandles>,
) {
    // Traveler shaders
    handles.traveler_glow = shaders.add(Shader::from_wgsl(
        sources::TRAVELER_GLOW,
        "embedded://traveler_glow.wgsl",
    ));

    // Environment shaders
    handles.nebula = shaders.add(Shader::from_wgsl(
        sources::NEBULA,
        "embedded://nebula.wgsl",
    ));

    handles.star_twinkle = shaders.add(Shader::from_wgsl(
        sources::STAR_TWINKLE,
        "embedded://star_twinkle.wgsl",
    ));

    // Post-processing shaders
    handles.post_chromatic = shaders.add(Shader::from_wgsl(
        sources::POST_CHROMATIC,
        "embedded://post_chromatic.wgsl",
    ));

    handles.post_grain = shaders.add(Shader::from_wgsl(
        sources::POST_GRAIN,
        "embedded://post_grain.wgsl",
    ));

    handles.post_vignette = shaders.add(Shader::from_wgsl(
        sources::POST_VIGNETTE,
        "embedded://post_vignette.wgsl",
    ));

    // Bang effect shaders
    handles.bang_core = shaders.add(Shader::from_wgsl(
        sources::BANG_CORE,
        "embedded://bang_core.wgsl",
    ));

    handles.bang_shockwave = shaders.add(Shader::from_wgsl(
        sources::BANG_SHOCKWAVE,
        "embedded://bang_shockwave.wgsl",
    ));

    handles.bang_godray = shaders.add(Shader::from_wgsl(
        sources::BANG_GODRAY,
        "embedded://bang_godray.wgsl",
    ));

    info!("Loaded 9 embedded shaders");
}
