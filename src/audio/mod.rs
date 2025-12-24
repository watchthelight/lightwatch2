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
mod reverb;
mod scale;
mod silence;
mod spatial;
mod transitions;
mod voice;

pub use ambiance::CosmicAmbiance;
pub use bang_sound::BangRumble;
pub use engine::AudioEngine;
pub use envelope::{Envelope, EnvelopeStage};
pub use events::{EventSoundConfig, EventSoundPlugin, EventSounds};
pub use filter::{BiquadFilter, FilterType};
pub use grief_sound::GriefDissonance;
pub use leitmotif::{Contour, Leitmotif, RhythmPattern};
pub use leitmotif_player::{LeitmotifPlayer, LeitmotifPlugin};
pub use melody::{Melody, MelodyGenerator};
pub use oscillator::{Oscillator, Waveform};
pub use reverb::Reverb;
pub use scale::{midi_to_freq, Scale, ScaleDegree, A4, D_PENTATONIC};
pub use silence::{SilenceManager, SilenceState};
pub use spatial::{
    calculate_attenuation, calculate_doppler, calculate_panning, AudioListener,
    SpatialAudioConfig, SpatialAudioPlugin, SpatialAudioSource,
};
pub use transitions::TransitionSound;
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
            .add_plugins(LeitmotifPlugin)
            .add_plugins(SpatialAudioPlugin)
            .add_plugins(EventSoundPlugin)
            .init_resource::<CosmicAmbiance>();
    }
}
