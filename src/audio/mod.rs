//! Audio systems: Synthesis, Leitmotifs, Spatial audio, Events

mod engine;
mod envelope;
mod filter;
mod oscillator;
mod voice;

pub use engine::AudioEngine;
pub use envelope::{Envelope, EnvelopeStage};
pub use filter::{BiquadFilter, FilterType};
pub use oscillator::{Oscillator, Waveform};
pub use voice::Voice;

use bevy::prelude::*;

/// Audio synthesis plugin
pub struct AudioSynthesisPlugin;

impl Plugin for AudioSynthesisPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AudioEngine>();
    }
}

/// Audio plugin for synthesis and spatial sound
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioSynthesisPlugin);
        // TODO: Traveler leitmotifs
        // TODO: Spatial audio positioning
        // TODO: Reverb
        // TODO: Audio event triggers
    }
}
