//! Volumetric fog - Phase-driven atmospheric fog

use bevy::pbr::FogSettings;
use bevy::prelude::*;

use crate::camera::ExperienceCamera;
use crate::core::{ExperienceClock, Phase};

/// Phase-specific fog colors
#[derive(Clone)]
pub struct PhaseColors {
    pub signal: Color,
    pub bang: Color,
    pub awakening: Color,
    pub discovery: Color,
    pub connection: Color,
    pub acceptance: Color,
}

impl Default for PhaseColors {
    fn default() -> Self {
        Self {
            signal: Color::BLACK,
            bang: Color::srgb(0.1, 0.05, 0.02),       // Warm from explosion
            awakening: Color::srgb(0.03, 0.02, 0.04), // Deep purple-black
            discovery: Color::srgb(0.04, 0.03, 0.05), // Slightly brighter
            connection: Color::srgb(0.05, 0.04, 0.03), // Warm amber tint
            acceptance: Color::srgb(0.01, 0.01, 0.02), // Fading to black
        }
    }
}

/// LIGHTWATCH fog configuration
#[derive(Resource)]
pub struct FogConfig {
    /// Fog density
    pub density: f32,
    /// Base fog color
    pub base_color: Color,
    /// Phase-specific fog colors
    pub phase_colors: PhaseColors,
    /// Transition speed
    pub transition_speed: f32,
}

impl Default for FogConfig {
    fn default() -> Self {
        Self {
            density: 0.02,
            base_color: Color::srgb(0.02, 0.02, 0.03),
            phase_colors: PhaseColors::default(),
            transition_speed: 0.5,
        }
    }
}

/// Current fog state for smooth transitions
#[derive(Resource)]
pub struct FogState {
    pub current_color: Color,
    pub target_color: Color,
    pub current_density: f32,
    pub target_density: f32,
}

impl Default for FogState {
    fn default() -> Self {
        Self {
            current_color: Color::BLACK,
            target_color: Color::BLACK,
            current_density: 0.0,
            target_density: 0.0,
        }
    }
}

/// Event to temporarily change fog
#[derive(Event)]
pub struct FogPulseEvent {
    pub color: Color,
    pub density: f32,
    #[allow(dead_code)]
    pub duration: f32,
}

/// Setup fog on camera
pub fn setup_fog(
    mut commands: Commands,
    cameras: Query<Entity, With<ExperienceCamera>>,
    config: Res<FogConfig>,
) {
    for entity in cameras.iter() {
        commands.entity(entity).insert(FogSettings {
            color: config.base_color,
            falloff: bevy::pbr::FogFalloff::Exponential {
                density: config.density,
            },
            ..default()
        });
    }

    info!(target: "lightwatch::environment", "Fog system initialized");
}

/// Update fog based on phase
pub fn update_fog_for_phase(
    clock: Res<ExperienceClock>,
    config: Res<FogConfig>,
    mut state: ResMut<FogState>,
) {
    let phase = clock.phase();
    let progress = clock.phase_progress();

    // Determine target color and density
    let (target_color, target_density) = match phase {
        Phase::Signal => (config.phase_colors.signal, 0.0),
        Phase::Bang => {
            // Fog intensity follows bang
            let intensity = if progress < 0.3 {
                progress / 0.3 // Ramp up
            } else {
                1.0 - (progress - 0.3) / 0.7 // Ramp down
            };
            (config.phase_colors.bang, config.density * intensity * 2.0)
        }
        Phase::Awakening => (config.phase_colors.awakening, config.density),
        Phase::Discovery => (config.phase_colors.discovery, config.density),
        Phase::Connection => (config.phase_colors.connection, config.density * 0.8),
        Phase::Acceptance => {
            // Fade to complete darkness
            let fade = 1.0 - progress;
            (config.phase_colors.acceptance, config.density * fade)
        }
        Phase::Ended => (Color::BLACK, 0.0),
    };

    state.target_color = target_color;
    state.target_density = target_density;
}

/// Interpolate fog to target
pub fn interpolate_fog(
    time: Res<Time>,
    config: Res<FogConfig>,
    mut state: ResMut<FogState>,
    mut fog_query: Query<&mut FogSettings>,
) {
    let dt = time.delta_seconds();

    // Lerp color
    let current = state.current_color.to_srgba();
    let target = state.target_color.to_srgba();
    state.current_color = Color::srgba(
        current.red + (target.red - current.red) * config.transition_speed * dt,
        current.green + (target.green - current.green) * config.transition_speed * dt,
        current.blue + (target.blue - current.blue) * config.transition_speed * dt,
        1.0,
    );

    // Lerp density
    state.current_density +=
        (state.target_density - state.current_density) * config.transition_speed * dt;

    // Apply to fog settings
    for mut fog in fog_query.iter_mut() {
        fog.color = state.current_color;
        fog.falloff = bevy::pbr::FogFalloff::Exponential {
            density: state.current_density,
        };
    }
}

/// Handle fog pulse events (e.g., on explosion)
pub fn handle_fog_pulses(mut events: EventReader<FogPulseEvent>, mut state: ResMut<FogState>) {
    for event in events.read() {
        // Override target temporarily
        state.target_color = event.color;
        state.target_density = event.density;
    }
}

/// Fog plugin
pub struct FogPlugin;

impl Plugin for FogPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FogConfig>()
            .init_resource::<FogState>()
            .add_event::<FogPulseEvent>()
            .add_systems(Startup, setup_fog)
            .add_systems(
                Update,
                (
                    update_fog_for_phase,
                    interpolate_fog.after(update_fog_for_phase),
                    handle_fog_pulses,
                ),
            );
    }
}
