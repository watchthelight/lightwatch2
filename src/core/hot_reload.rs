//! Hot reloading configuration for development

use bevy::prelude::*;

/// Hot reload configuration
#[derive(Resource)]
pub struct HotReloadConfig {
    pub enabled: bool,
    pub watch_shaders: bool,
    pub watch_assets: bool,
}

impl Default for HotReloadConfig {
    fn default() -> Self {
        Self {
            enabled: cfg!(debug_assertions),
            watch_shaders: true,
            watch_assets: true,
        }
    }
}

/// Setup hot reloading for development
pub fn setup_hot_reload(_commands: Commands) {
    #[cfg(debug_assertions)]
    {
        info!("Hot reloading enabled for development");
        // Bevy's asset server handles hot reloading automatically
        // when running from cargo with CARGO_MANIFEST_DIR set
    }
}

/// Manual reload trigger (F5)
pub fn manual_reload_trigger(keyboard: Res<ButtonInput<KeyCode>>, _asset_server: Res<AssetServer>) {
    #[cfg(debug_assertions)]
    if keyboard.just_pressed(KeyCode::F5) {
        info!("Manual reload triggered");
        // Assets will be reloaded automatically by Bevy's file watcher
    }
}
