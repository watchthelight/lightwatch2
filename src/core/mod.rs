//! Core systems: Window, Renderer, Exposure, Clock, State, Events

use bevy::prelude::*;

pub mod exposure;
pub mod renderer;
pub mod window;

pub use exposure::*;
pub use renderer::*;
pub use window::*;

/// Core plugin for window, rendering, and core systems
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ExposureControl>().add_systems(
            Update,
            (
                handle_window_close,
                #[cfg(debug_assertions)]
                toggle_fullscreen,
                update_exposure,
            ),
        );

        // TODO: Clock system
        // TODO: State machine
        // TODO: Event bus
        // TODO: Phase controller
    }
}
