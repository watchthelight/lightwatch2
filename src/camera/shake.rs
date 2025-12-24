//! Camera shake system - trauma-based organic shake

use bevy::prelude::*;

use super::CameraRig;
use crate::core::CameraShakeEvent;
use crate::wide_event;

/// Camera shake state using trauma system
/// Reference: https://www.youtube.com/watch?v=tu-Qe66AvtY (GDC talk)
#[derive(Resource)]
pub struct CameraShake {
    /// Current trauma level (0.0 - 1.0)
    pub trauma: f32,
    /// Trauma decay rate per second
    pub decay_rate: f32,
    /// Maximum rotation (degrees)
    pub max_rotation: f32,
    /// Maximum translation
    pub max_offset: f32,
    /// Noise time offset (for variety)
    pub noise_time: f32,
    /// Noise frequency
    pub frequency: f32,
}

impl Default for CameraShake {
    fn default() -> Self {
        Self {
            trauma: 0.0,
            decay_rate: 1.5,   // Trauma decays over ~0.7s
            max_rotation: 3.0, // Degrees
            max_offset: 0.5,   // Units
            noise_time: 0.0,
            frequency: 15.0, // Hz
        }
    }
}

impl CameraShake {
    /// Add trauma (clamped to 1.0)
    pub fn add_trauma(&mut self, amount: f32) {
        self.trauma = (self.trauma + amount).min(1.0);
    }

    /// Get shake amount (trauma squared for nonlinear response)
    pub fn shake_amount(&self) -> f32 {
        self.trauma * self.trauma
    }

    /// Decay trauma over time
    pub fn decay(&mut self, delta: f32) {
        self.trauma = (self.trauma - self.decay_rate * delta).max(0.0);
    }
}

/// Predefined shake intensities for different moments
pub struct ShakePresets;

impl ShakePresets {
    /// Bang peak - overwhelming
    pub const BANG_PEAK: f32 = 0.8;

    /// Bang aftershock
    pub const BANG_AFTERSHOCK: f32 = 0.4;

    /// Grief response (Child dies)
    pub const GRIEF: f32 = 0.3;

    /// Subtle emphasis
    pub const SUBTLE: f32 = 0.15;

    /// Heavy impact
    pub const IMPACT: f32 = 0.6;
}

/// Simple hash-based noise
fn hash_1d(n: i32) -> f32 {
    let n = (n << 13) ^ n;
    let nn = n.wrapping_mul(n.wrapping_mul(n.wrapping_mul(15731) + 789221) + 1376312589);
    1.0 - (nn & 0x7fffffff) as f32 / 1073741824.0
}

/// Simple Perlin-like noise for organic shake
pub fn noise_1d(x: f32) -> f32 {
    let xi = x.floor() as i32;
    let xf = x.fract();

    // Smoothstep interpolation
    let u = xf * xf * (3.0 - 2.0 * xf);

    let a = hash_1d(xi);
    let b = hash_1d(xi + 1);

    a + u * (b - a)
}

/// Multi-octave noise for richer shake
pub fn fbm_noise(x: f32, octaves: i32) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 0.5;
    let mut frequency = 1.0;

    for _ in 0..octaves {
        value += amplitude * noise_1d(x * frequency);
        amplitude *= 0.5;
        frequency *= 2.0;
    }

    value
}

/// Handle shake events
pub fn handle_shake_events(
    mut shake: ResMut<CameraShake>,
    mut events: EventReader<CameraShakeEvent>,
) {
    for event in events.read() {
        shake.add_trauma(event.intensity);

        wide_event!("camera_shake_triggered")
            .with_f32("intensity", event.intensity)
            .with_f32("duration", event.duration)
            .emit(event.elapsed);
    }
}

/// Update shake state
pub fn update_shake(time: Res<Time>, mut shake: ResMut<CameraShake>) {
    // Advance noise time
    shake.noise_time += time.delta_seconds() * shake.frequency;

    // Decay trauma
    shake.decay(time.delta_seconds());
}

/// Apply shake to camera rig
pub fn apply_shake_to_rig(shake: Res<CameraShake>, mut rigs: Query<&mut CameraRig>) {
    if shake.trauma < 0.001 {
        // Clear shake offset when no trauma
        for mut rig in rigs.iter_mut() {
            rig.shake_offset = rig.shake_offset.lerp(Vec3::ZERO, 0.3);
            rig.rotation_offset = rig.rotation_offset.slerp(Quat::IDENTITY, 0.3);
        }
        return;
    }

    let amount = shake.shake_amount();
    let t = shake.noise_time;

    // Generate shake offset using noise
    let offset_x = fbm_noise(t, 3) * shake.max_offset * amount;
    let offset_y = fbm_noise(t + 100.0, 3) * shake.max_offset * amount;
    let offset_z = fbm_noise(t + 200.0, 3) * shake.max_offset * amount * 0.5;

    // Generate rotation shake
    let rot_x = fbm_noise(t + 300.0, 2) * shake.max_rotation * amount;
    let rot_y = fbm_noise(t + 400.0, 2) * shake.max_rotation * amount;
    let rot_z = fbm_noise(t + 500.0, 2) * shake.max_rotation * amount * 0.5;

    for mut rig in rigs.iter_mut() {
        rig.shake_offset = Vec3::new(offset_x, offset_y, offset_z);

        // Apply rotation as quaternion
        rig.rotation_offset = Quat::from_euler(
            EulerRot::XYZ,
            rot_x.to_radians(),
            rot_y.to_radians(),
            rot_z.to_radians(),
        );
    }
}
