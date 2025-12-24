//! Performance monitoring and adaptive quality

use bevy::core_pipeline::bloom::BloomSettings;
use bevy::prelude::*;

use crate::camera::ExperienceCamera;

/// Performance configuration
#[derive(Resource)]
pub struct PerformanceConfig {
    /// Target frame time (ms) - 60 FPS
    pub target_frame_time: f32,
    /// Maximum particles
    #[allow(dead_code)]
    pub max_particles: usize,
    /// LOD distance thresholds
    #[allow(dead_code)]
    pub lod_distances: [f32; 3],
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            target_frame_time: 16.67, // 60 FPS
            max_particles: 10000,
            lod_distances: [20.0, 50.0, 100.0],
        }
    }
}

/// Performance metrics
#[derive(Resource, Default)]
pub struct PerformanceMetrics {
    /// Recent frame times in milliseconds
    pub frame_times: Vec<f32>,
    /// Current entity count
    #[allow(dead_code)]
    pub entity_count: usize,
}

/// Update performance metrics
pub fn update_metrics(
    time: Res<Time>,
    mut metrics: ResMut<PerformanceMetrics>,
    entities: Query<Entity>,
) {
    let frame_time = time.delta_seconds() * 1000.0;
    metrics.frame_times.push(frame_time);

    // Keep last 60 frames
    if metrics.frame_times.len() > 60 {
        metrics.frame_times.remove(0);
    }

    metrics.entity_count = entities.iter().count();
}

/// Adaptive quality based on performance
pub fn adaptive_quality(
    metrics: Res<PerformanceMetrics>,
    config: Res<PerformanceConfig>,
    mut bloom: Query<&mut BloomSettings, With<ExperienceCamera>>,
) {
    if metrics.frame_times.len() < 30 {
        return;
    }

    let avg_frame_time: f32 =
        metrics.frame_times.iter().sum::<f32>() / metrics.frame_times.len() as f32;

    // If running slow (below 40 FPS), reduce bloom quality
    if avg_frame_time > config.target_frame_time * 1.5 {
        for mut bloom_settings in bloom.iter_mut() {
            bloom_settings.intensity = (bloom_settings.intensity * 0.95).max(0.05);
        }
    }
}
