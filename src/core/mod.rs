//! Core systems: Window, Renderer, Exposure, Clock, State, Events

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

pub mod debug_overlay;
pub mod exposure;
pub mod hot_reload;
pub mod logging;
pub mod renderer;
pub mod time_control;
pub mod window;

pub use debug_overlay::*;
pub use exposure::*;
pub use hot_reload::*;
pub use logging::*;
pub use renderer::*;
pub use time_control::*;
pub use window::*;

/// Core plugin for window, rendering, and core systems
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app
            // Diagnostics for FPS display
            .add_plugins(FrameTimeDiagnosticsPlugin::default())
            // Resources
            .init_resource::<ExposureControl>()
            .init_resource::<DebugOverlayState>()
            .init_resource::<HotReloadConfig>()
            .init_resource::<TimeControl>()
            // Startup systems
            .add_systems(Startup, (spawn_debug_overlay, setup_hot_reload))
            // Update systems
            .add_systems(
                Update,
                (
                    handle_window_close,
                    update_exposure,
                    update_debug_overlay,
                    toggle_debug_overlay,
                ),
            );

        // Debug-only systems
        #[cfg(debug_assertions)]
        app.add_systems(
            Update,
            (
                toggle_fullscreen,
                manual_reload_trigger,
                time_control::handle_time_control,
            ),
        );

        // TODO: Clock system
        // TODO: State machine
        // TODO: Event bus
        // TODO: Phase controller
    }
}
