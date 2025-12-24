//! Build information embedded at compile time

use bevy::prelude::*;

/// Build information embedded at compile time
pub struct BuildInfo;

impl BuildInfo {
    pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    pub const NAME: &'static str = env!("CARGO_PKG_NAME");
    pub const TARGET: &'static str = env!("TARGET");

    #[cfg(debug_assertions)]
    pub const PROFILE: &'static str = "debug";

    #[cfg(not(debug_assertions))]
    pub const PROFILE: &'static str = "release";

    pub fn log_info() {
        info!(
            "{} v{} ({} build for {})",
            Self::NAME,
            Self::VERSION,
            Self::PROFILE,
            Self::TARGET
        );
    }
}
