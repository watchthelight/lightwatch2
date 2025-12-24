//! Core systems: Window, Renderer, Exposure, Clock, State, Events, Integration

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

pub mod build_info;
pub mod clock;
pub mod debug_overlay;
pub mod events;
pub mod exposure;
pub mod hot_reload;
pub mod input;
pub mod logging;
pub mod performance;
pub mod phase_controller;
pub mod polish;
pub mod ready_screen;
pub mod renderer;
pub mod state;
pub mod time_control;
pub mod timeline_verify;
pub mod window;

pub use build_info::*;
pub use clock::*;
pub use debug_overlay::*;
pub use events::*;
pub use exposure::*;
pub use hot_reload::*;
pub use input::*;
pub use phase_controller::*;
pub use ready_screen::*;
pub use renderer::*;
pub use state::*;
pub use time_control::*;
pub use timeline_verify::*;
pub use window::*;
pub use performance::{PerformanceConfig, PerformanceMetrics};
pub use polish::FadeState;

/// Core plugin for window, rendering, and core systems
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app
            // Diagnostics for FPS display
            .add_plugins(FrameTimeDiagnosticsPlugin::default())
            // Events plugin
            .add_plugins(EventsPlugin)
            // Phase controller
            .add_plugins(PhaseControllerPlugin)
            // Input handling
            .add_plugins(InputPlugin)
            // State
            .init_state::<ExperienceState>()
            // Events
            .add_event::<PhaseChangedEvent>()
            .add_event::<StateChangedEvent>()
            // Resources
            .init_resource::<ExperienceClock>()
            .init_resource::<EndingTimer>()
            .init_resource::<ExposureControl>()
            .init_resource::<DebugOverlayState>()
            .init_resource::<HotReloadConfig>()
            .init_resource::<TimeControl>()
            // System sets
            .configure_sets(
                Update,
                (
                    ReadySet.run_if(in_ready_state),
                    RunningSet.run_if(in_running_state),
                    EndingSet.run_if(|s: Res<State<ExperienceState>>| {
                        *s.get() == ExperienceState::Ending
                    }),
                ),
            )
            // Startup systems
            .add_systems(Startup, (spawn_debug_overlay, spawn_ready_screen, setup_hot_reload))
            // State transitions
            .add_systems(OnEnter(ExperienceState::Running), hide_ready_screen)
            // Update systems
            .add_systems(
                Update,
                (
                    handle_window_close,
                    check_loading_complete,
                    update_clock.in_set(RunningSet),
                    emit_phase_changes.after(update_clock).in_set(RunningSet),
                    check_experience_end.in_set(RunningSet),
                    handle_ending_phase.in_set(EndingSet),
                    log_state_transitions,
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
                clear_scrub_position.after(update_clock),
            ),
        );
    }
}

/// Integration plugin for final polish and verification
pub struct IntegrationPlugin;

impl Plugin for IntegrationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PerformanceConfig>()
            .init_resource::<PerformanceMetrics>()
            .init_resource::<TimelineVerification>()
            .init_resource::<FadeState>()
            .add_systems(
                Update,
                (
                    performance::update_metrics,
                    performance::adaptive_quality,
                    timeline_verify::verify_timeline,
                    polish::fade_visuals_at_end,
                    polish::log_experience_ending,
                ),
            )
            .add_systems(Last, timeline_verify::log_timeline_verification);

        info!(target: "lightwatch::integration", "Integration plugin initialized");
    }
}
