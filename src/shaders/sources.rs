//! Embedded shader sources - compiled into the binary

/// Traveler glow shader - pulsing emissive effect
pub const TRAVELER_GLOW: &str = include_str!("../../assets/shaders/traveler_glow.wgsl");

/// Nebula raymarching shader - cosmic background
pub const NEBULA: &str = include_str!("../../assets/shaders/nebula.wgsl");

/// Star twinkle shader - flickering starfield
pub const STAR_TWINKLE: &str = include_str!("../../assets/shaders/star_twinkle.wgsl");

/// Chromatic aberration post-process shader
pub const POST_CHROMATIC: &str = include_str!("../../assets/shaders/post_chromatic.wgsl");

/// Film grain post-process shader
pub const POST_GRAIN: &str = include_str!("../../assets/shaders/post_grain.wgsl");

/// Vignette post-process shader
pub const POST_VIGNETTE: &str = include_str!("../../assets/shaders/post_vignette.wgsl");

/// Bang core explosion shader
pub const BANG_CORE: &str = include_str!("../../assets/shaders/bang_core.wgsl");

/// Bang shockwave shader - expanding ring distortion
pub const BANG_SHOCKWAVE: &str = include_str!("../../assets/shaders/bang_shockwave.wgsl");

/// God ray shader - volumetric light beams
pub const BANG_GODRAY: &str = include_str!("../../assets/shaders/bang_godray.wgsl");

/// Depth of field post-process shader
pub const DOF: &str = include_str!("../../assets/shaders/dof.wgsl");
