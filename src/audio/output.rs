//! Audio output system - connects synthesis to speakers via cpal

use std::sync::{Arc, Mutex};

use bevy::prelude::*;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use super::bang_sound::BangRumble;
use super::grief_sound::GriefDissonance;
use super::transitions::TransitionSound;
use super::{BiquadFilter, FilterType, Oscillator, Waveform};
use crate::core::Phase;

/// Trigger commands sent from Bevy to audio thread
#[derive(Clone)]
pub enum AudioTrigger {
    BangRumble,
    GriefDissonance,
    PhaseTransition(Phase),
}

/// Audio state owned by the audio thread
struct AudioState {
    bang_rumble: BangRumble,
    grief: GriefDissonance,
    transitions: TransitionSound,
    ambiance: AmbianceGenerator,
    master_volume: f32,
    sample_rate: f32,
}

impl AudioState {
    fn new(sample_rate: f32) -> Self {
        Self {
            bang_rumble: BangRumble::new(sample_rate),
            grief: GriefDissonance::new(),
            transitions: TransitionSound::new(),
            ambiance: AmbianceGenerator::new(sample_rate),
            master_volume: 0.7,
            sample_rate,
        }
    }

    fn process_triggers(&mut self, triggers: &mut Vec<AudioTrigger>) {
        for trigger in triggers.drain(..) {
            match trigger {
                AudioTrigger::BangRumble => {
                    self.bang_rumble.trigger();
                }
                AudioTrigger::GriefDissonance => {
                    self.grief.trigger();
                }
                AudioTrigger::PhaseTransition(phase) => {
                    self.transitions.trigger_for_phase(phase);
                }
            }
        }
    }

    fn generate_sample(&mut self) -> f32 {
        let dt = 1.0 / self.sample_rate;

        let mut sample = 0.0;

        // Event sounds
        sample += self.bang_rumble.sample(self.sample_rate, dt);
        sample += self.grief.sample(self.sample_rate, dt);
        sample += self.transitions.sample(self.sample_rate, dt);

        // Ambiance
        sample += self.ambiance.sample(self.sample_rate);

        // Master volume and soft clip
        sample *= self.master_volume;
        soft_clip(sample)
    }
}

/// Simplified ambiance generator for the audio thread
struct AmbianceGenerator {
    rumble: Oscillator,
    shimmer: Oscillator,
    noise: Oscillator,
    rumble_filter: BiquadFilter,
    shimmer_filter: BiquadFilter,
    noise_filter: BiquadFilter,
    volume: f32,
    active: bool,
}

impl AmbianceGenerator {
    fn new(sample_rate: f32) -> Self {
        Self {
            rumble: Oscillator::new(Waveform::Sine, 30.0),
            shimmer: Oscillator::new(Waveform::Sine, 800.0),
            noise: Oscillator::new(Waveform::Noise, 0.0),
            rumble_filter: BiquadFilter::new(FilterType::LowPass, 60.0, 0.7, sample_rate),
            shimmer_filter: BiquadFilter::new(FilterType::HighPass, 2000.0, 2.0, sample_rate),
            noise_filter: BiquadFilter::new(FilterType::BandPass, 400.0, 0.5, sample_rate),
            volume: 0.15,
            active: true,
        }
    }

    fn sample(&mut self, sample_rate: f32) -> f32 {
        if !self.active {
            return 0.0;
        }

        let rumble = self.rumble.sample(sample_rate);
        let rumble_filtered = self.rumble_filter.process(rumble) * 0.5;

        let shimmer = self.shimmer.sample(sample_rate);
        let shimmer_filtered = self.shimmer_filter.process(shimmer) * 0.1;

        let noise = self.noise.sample(sample_rate);
        let noise_filtered = self.noise_filter.process(noise) * 0.05;

        (rumble_filtered + shimmer_filtered + noise_filtered) * self.volume
    }
}

/// Soft clipping function for gentle limiting
fn soft_clip(x: f32) -> f32 {
    if x.abs() < 0.5 {
        x
    } else {
        x.signum() * (1.0 - (-2.0 * (x.abs() - 0.5)).exp()) * 0.5 + x.signum() * 0.5
    }
}

/// Shared trigger queue between Bevy and audio thread
#[derive(Resource, Default)]
pub struct AudioTriggerQueue {
    triggers: Arc<Mutex<Vec<AudioTrigger>>>,
}

impl AudioTriggerQueue {
    pub fn send(&self, trigger: AudioTrigger) {
        if let Ok(mut queue) = self.triggers.lock() {
            queue.push(trigger);
        }
    }
}

/// NonSend resource holding the audio stream (keeps it alive)
/// cpal::Stream is not Send+Sync on macOS, so we use NonSend
pub struct AudioOutput {
    #[allow(dead_code)]
    stream: cpal::Stream,
}

/// Initialize audio output (exclusive system for NonSend resource)
pub fn init_audio_output(world: &mut World) {
    let trigger_queue = world.resource::<AudioTriggerQueue>();
    let triggers = trigger_queue.triggers.clone();

    let host = cpal::default_host();

    let Some(device) = host.default_output_device() else {
        warn!(target: "lightwatch::audio", "No audio output device available");
        return;
    };

    let Ok(config) = device.default_output_config() else {
        warn!(target: "lightwatch::audio", "No default audio config available");
        return;
    };

    info!(
        target: "lightwatch::audio",
        "Audio device: {}, Sample rate: {}, Channels: {}",
        device.name().unwrap_or_default(),
        config.sample_rate().0,
        config.channels()
    );

    let sample_rate = config.sample_rate().0 as f32;
    let channels = config.channels() as usize;

    // Audio state owned by the audio thread
    let state = Arc::new(Mutex::new(AudioState::new(sample_rate)));

    let stream = device
        .build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let Ok(mut audio_state) = state.lock() else {
                    for sample in data.iter_mut() {
                        *sample = 0.0;
                    }
                    return;
                };

                // Process any pending triggers
                if let Ok(mut queue) = triggers.try_lock() {
                    audio_state.process_triggers(&mut queue);
                }

                // Generate audio
                for frame in data.chunks_mut(channels) {
                    let sample = audio_state.generate_sample();
                    for channel in frame.iter_mut() {
                        *channel = sample;
                    }
                }
            },
            |err| {
                error!(target: "lightwatch::audio", "Audio stream error: {}", err);
            },
            None,
        )
        .expect("Failed to build audio stream");

    stream.play().expect("Failed to start audio stream");

    world.insert_non_send_resource(AudioOutput { stream });

    info!(target: "lightwatch::audio", "Audio output initialized");
}

/// Audio output plugin
pub struct AudioOutputPlugin;

impl Plugin for AudioOutputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AudioTriggerQueue>()
            .add_systems(Startup, init_audio_output);
    }
}
