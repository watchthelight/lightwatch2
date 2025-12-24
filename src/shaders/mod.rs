//! Shader utilities, embedded sources, and custom materials

use bevy::prelude::*;

pub mod loader;
pub mod material;
pub mod sources;

pub use loader::*;
pub use material::*;

/// Shader plugin - loads all embedded shaders and registers custom materials
pub struct ShadersPlugin;

impl Plugin for ShadersPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ShaderHandles>()
            .add_plugins(MaterialPlugin::<TravelerGlowMaterial>::default())
            .add_systems(Startup, load_embedded_shaders);
    }
}
