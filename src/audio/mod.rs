//! Audio systems: Synthesis, Leitmotifs, Spatial audio, Events

mod ambiance;
mod bang_sound;
mod engine;
mod envelope;
mod events;
mod filter;
mod grief_sound;
mod leitmotif;
mod leitmotif_player;
mod melody;
mod oscillator;
mod output;
mod reverb;
mod scale;
mod silence;
mod spatial;
mod transitions;
mod voice;

pub use ambiance::CosmicAmbiance;
pub use engine::AudioEngine;
pub use envelope::Envelope;
pub use events::EventSoundPlugin;
pub use filter::{BiquadFilter, FilterType};
pub use leitmotif_player::LeitmotifPlugin;
pub use oscillator::{Oscillator, Waveform};
pub use output::AudioOutputPlugin;
pub use spatial::SpatialAudioPlugin;
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
        app.add_plugins(AudioSynthesisPlugin)
            .add_plugins(AudioOutputPlugin)
            .add_plugins(LeitmotifPlugin)
            .add_plugins(SpatialAudioPlugin)
            .add_plugins(EventSoundPlugin)
            .init_resource::<CosmicAmbiance>();
    }
}
