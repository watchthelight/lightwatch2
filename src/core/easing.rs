//! Unified easing functions for LIGHTWATCH
//!
//! All animation systems should use these functions for consistent motion.

/// Smooth step easing (cubic hermite)
/// Used for phase transition buffers and general smooth interpolation
#[inline]
pub fn smooth_step(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Smoother step (quintic hermite) - even smoother acceleration/deceleration
#[inline]
pub fn smoother_step(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

/// Ease in cubic - slow start, fast end
#[inline]
pub fn ease_in_cubic(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * t
}

/// Ease out cubic - fast start, slow end
#[inline]
pub fn ease_out_cubic(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    1.0 - (1.0 - t).powi(3)
}

/// Ease in-out cubic - slow start and end, fast middle
#[inline]
pub fn ease_in_out_cubic(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
    }
}

/// Ease in quartic - slower start than cubic
#[inline]
pub fn ease_in_quart(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * t * t
}

/// Ease out quartic - faster end than cubic
#[inline]
pub fn ease_out_quart(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    1.0 - (1.0 - t).powi(4)
}

/// Ease in-out quartic
#[inline]
pub fn ease_in_out_quart(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    if t < 0.5 {
        8.0 * t * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(4) / 2.0
    }
}

/// Ease out exponential - very fast start, very slow end
/// Good for dramatic effects like shockwaves
#[inline]
pub fn ease_out_expo(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    if t >= 1.0 {
        1.0
    } else {
        1.0 - 2.0_f32.powf(-10.0 * t)
    }
}

/// Ease in exponential - very slow start, very fast end
#[inline]
pub fn ease_in_expo(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    if t <= 0.0 {
        0.0
    } else {
        2.0_f32.powf(10.0 * t - 10.0)
    }
}

/// Ease out quad - simple quadratic ease out
#[inline]
pub fn ease_out_quad(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    1.0 - (1.0 - t) * (1.0 - t)
}

/// Ease in quad - simple quadratic ease in
#[inline]
pub fn ease_in_quad(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t
}

/// Linear interpolation (no easing)
#[inline]
pub fn linear(t: f32) -> f32 {
    t.clamp(0.0, 1.0)
}

/// Calculate frame-rate independent smooth factor for lerping
/// Returns a factor to use with `lerp(current, target, factor)`
/// `speed` is the decay rate (higher = faster convergence)
#[inline]
pub fn smooth_lerp_factor(delta_seconds: f32, speed: f32) -> f32 {
    1.0 - (-speed * delta_seconds).exp()
}
