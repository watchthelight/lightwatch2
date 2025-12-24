//! Spatial audio system - 3D positioning, attenuation, Doppler

use bevy::prelude::*;

use crate::camera::ExperienceCamera;

/// Spatial audio configuration
#[derive(Resource)]
pub struct SpatialAudioConfig {
    /// Maximum audible distance
    pub max_distance: f32,
    /// Distance at which attenuation begins
    pub reference_distance: f32,
    /// Attenuation rolloff factor
    pub rolloff: f32,
    /// Speed of sound (for Doppler)
    pub speed_of_sound: f32,
    /// Doppler factor
    pub doppler_factor: f32,
    /// Master reverb mix
    #[allow(dead_code)]
    pub reverb_mix: f32,
}

impl Default for SpatialAudioConfig {
    fn default() -> Self {
        Self {
            max_distance: 100.0,
            reference_distance: 5.0,
            rolloff: 1.0,
            speed_of_sound: 343.0,
            doppler_factor: 0.5,
            reverb_mix: 0.4,
        }
    }
}

/// Audio listener (usually on camera)
#[derive(Component, Default)]
pub struct AudioListener {
    /// Previous position (for Doppler)
    pub previous_position: Vec3,
}

/// Spatial audio source
#[derive(Component)]
pub struct SpatialAudioSource {
    /// Base volume
    #[allow(dead_code)]
    pub volume: f32,
    /// Previous position (for Doppler)
    pub previous_position: Vec3,
    /// Computed gain after spatial processing
    #[allow(dead_code)]
    pub computed_gain: f32,
    /// Computed pan (-1 left, +1 right)
    #[allow(dead_code)]
    pub computed_pan: f32,
    /// Computed pitch shift (Doppler)
    #[allow(dead_code)]
    pub computed_pitch: f32,
}

impl Default for SpatialAudioSource {
    fn default() -> Self {
        Self {
            volume: 1.0,
            previous_position: Vec3::ZERO,
            computed_gain: 1.0,
            computed_pan: 0.0,
            computed_pitch: 1.0,
        }
    }
}

/// Calculate distance attenuation
pub fn calculate_attenuation(distance: f32, config: &SpatialAudioConfig) -> f32 {
    if distance >= config.max_distance {
        return 0.0;
    }

    if distance <= config.reference_distance {
        return 1.0;
    }

    // Inverse distance falloff
    let clamped_distance = distance.max(config.reference_distance);
    let attenuation = config.reference_distance
        / (config.reference_distance + config.rolloff * (clamped_distance - config.reference_distance));

    attenuation.clamp(0.0, 1.0)
}

/// Calculate stereo panning from 3D position
pub fn calculate_panning(
    source_pos: Vec3,
    listener_pos: Vec3,
    listener_forward: Vec3,
    listener_right: Vec3,
) -> f32 {
    let direction = (source_pos - listener_pos).normalize_or_zero();

    // Project onto listener's right vector for left/right panning
    let pan = direction.dot(listener_right);

    // Attenuate panning when source is behind listener
    let forward_factor = (direction.dot(listener_forward) + 1.0) * 0.5;

    pan * (0.5 + 0.5 * forward_factor)
}

/// Calculate Doppler pitch shift
pub fn calculate_doppler(
    source_pos: Vec3,
    source_prev_pos: Vec3,
    listener_pos: Vec3,
    listener_prev_pos: Vec3,
    delta_time: f32,
    config: &SpatialAudioConfig,
) -> f32 {
    if delta_time <= 0.0 {
        return 1.0;
    }

    // Calculate velocities
    let source_velocity = (source_pos - source_prev_pos) / delta_time;
    let listener_velocity = (listener_pos - listener_prev_pos) / delta_time;

    // Direction from listener to source
    let direction = (source_pos - listener_pos).normalize_or_zero();

    // Relative velocities toward each other
    let source_toward = source_velocity.dot(-direction);
    let listener_toward = listener_velocity.dot(direction);

    // Doppler shift
    let speed = config.speed_of_sound;
    let numerator = speed + listener_toward * config.doppler_factor;
    let denominator = speed + source_toward * config.doppler_factor;

    if denominator.abs() < 0.001 {
        return 1.0;
    }

    (numerator / denominator).clamp(0.5, 2.0)
}

/// Update spatial audio sources
pub fn update_spatial_audio(
    time: Res<Time>,
    config: Res<SpatialAudioConfig>,
    mut listener_query: Query<(&GlobalTransform, &mut AudioListener)>,
    mut sources: Query<(&GlobalTransform, &mut SpatialAudioSource)>,
) {
    let Ok((listener_transform, mut listener)) = listener_query.get_single_mut() else {
        return;
    };

    let listener_pos = listener_transform.translation();
    let listener_forward = listener_transform.forward().as_vec3();
    let listener_right = listener_transform.right().as_vec3();
    let dt = time.delta_seconds();

    for (source_transform, mut source) in sources.iter_mut() {
        let source_pos = source_transform.translation();

        // Distance attenuation
        let distance = (source_pos - listener_pos).length();
        source.computed_gain = source.volume * calculate_attenuation(distance, &config);

        // Stereo panning
        source.computed_pan = calculate_panning(
            source_pos,
            listener_pos,
            listener_forward,
            listener_right,
        );

        // Doppler pitch shift
        source.computed_pitch = calculate_doppler(
            source_pos,
            source.previous_position,
            listener_pos,
            listener.previous_position,
            dt,
            &config,
        );

        // Update previous positions
        source.previous_position = source_pos;
    }

    listener.previous_position = listener_pos;
}

/// Attach listener to camera
pub fn attach_listener_to_camera(
    mut commands: Commands,
    camera: Query<Entity, (With<ExperienceCamera>, Without<AudioListener>)>,
) {
    for entity in camera.iter() {
        commands.entity(entity).insert(AudioListener::default());
    }
}

/// Spatial audio plugin
pub struct SpatialAudioPlugin;

impl Plugin for SpatialAudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpatialAudioConfig>()
            .add_systems(Update, (attach_listener_to_camera, update_spatial_audio));
    }
}
