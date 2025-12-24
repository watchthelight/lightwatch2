//! LIGHTWATCH library - Re-exports for clean imports

pub mod audio;
pub mod bang;
pub mod camera;
pub mod core;
pub mod environment;
pub mod narrative;
pub mod post;
pub mod shaders;
pub mod text;
pub mod travelers;

/// Common prelude for LIGHTWATCH
pub mod prelude {
    pub use crate::core::*;
    pub use crate::travelers::TravelerId;
    pub use bevy::prelude::*;
}
