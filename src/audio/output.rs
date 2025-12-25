//! Audio output system - connects synthesis to speakers via cpal

use std::sync::{Arc, Mutex};

use bevy::prelude::*;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use super::bang_sound::BangRumble;
use super::grief_sound::GriefDissonance;
use super::reverb::Reverb;
use super::spatial::SpatialAudioSource;
use super::transitions::TransitionSound;
use super::{BiquadFilter, FilterType, Oscillator, Waveform};
use crate::core::Phase;

/// Trigger commands sent from Bevy to audio thread
#[derive(Clone)]
pub enum AudioTrigger {
    BangRumble,
    GriefDissonance,
    PhaseTransition(Phase),
    /// Start fading ambiance to silence
    FadeAmbiance { duration: f32 },
}

/// Spatial data for a single audio source
#[derive(Clone, Default)]
pub struct SpatialSourceData {
    pub gain: f32,
    pub pan: f32, // -1 left, +1 right
    pub pitch: f32,
}

/// Shared spatial mix data between Bevy and audio thread
#[derive(Default)]
pub struct SpatialMixData {
    /// Per-traveler spatial data
    pub travelers: [SpatialSourceData; 5], // One per TravelerId
    /// Master spatial influence (weighted average of active travelers)
    pub master_pan: f32,
    pub master_gain: f32,
}

/// Resource for sharing spatial data with audio thread
#[derive(Resource)]
pub struct SharedSpatialData {
    pub data: Arc<Mutex<SpatialMixData>>,
}

/// Audio state owned by the audio thread
struct AudioState {
    bang_rumble: BangRumble,
    grief: GriefDissonance,
    transitions: TransitionSound,
    ambiance: AmbianceGenerator,
    reverb: Reverb,
    master_volume: f32,
    sample_rate: f32,
    /// Cached spatial data
    cached_pan: f32,
    cached_gain: f32,
}

impl AudioState {
    fn new(sample_rate: f32) -> Self {
        let mut reverb = Reverb::new(sample_rate);
        reverb.mix = 0.25; // 25% wet for cosmic space feel

        Self {
            bang_rumble: BangRumble::new(sample_rate),
            grief: GriefDissonance::new(),
            transitions: TransitionSound::new(),
            ambiance: AmbianceGenerator::new(sample_rate),
            reverb,
            master_volume: 0.7,
            sample_rate,
            cached_pan: 0.0,
            cached_gain: 1.0,
        }
    }

    fn update_spatial(&mut self, spatial_data: &SpatialMixData) {
        self.cached_pan = spatial_data.master_pan;
        self.cached_gain = spatial_data.master_gain.max(0.3); // Minimum gain
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
                AudioTrigger::FadeAmbiance { duration } => {
                    self.ambiance.start_fade(duration);
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

        // Apply reverb for cosmic space feel
        sample = self.reverb.process(sample);

        // Master volume and soft clip
        sample *= self.master_volume;
        soft_clip(sample)
    }

    /// Generate stereo sample with spatial panning
    fn generate_stereo_sample(&mut self) -> (f32, f32) {
        let mono = self.generate_sample();

        // Apply spatial panning
        // pan: -1.0 = full left, +1.0 = full right
        // Using constant-power panning for natural sound
        let pan = self.cached_pan.clamp(-1.0, 1.0);
        let angle = (pan + 1.0) * std::f32::consts::FRAC_PI_4; // 0 to PI/2
        let left = mono * angle.cos();
        let right = mono * angle.sin();

        // Apply spatial gain
        let gain = self.cached_gain;
        (left * gain, right * gain)
    }
}

/// Cosmic ambiance generator for the audio thread
struct AmbianceGenerator {
    rumble: Oscillator,
    shimmer: Oscillator,
    noise: Oscillator,
    rumble_filter: BiquadFilter,
    shimmer_filter: BiquadFilter,
    noise_filter: BiquadFilter,
    volume: f32,
    target_volume: f32,
    fade_speed: f32,
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
            target_volume: 0.15,
            fade_speed: 0.0,
            active: true,
        }
    }

    fn start_fade(&mut self, duration: f32) {
        self.target_volume = 0.0;
        if duration > 0.0 {
            self.fade_speed = self.volume / duration;
        } else {
            self.fade_speed = 1.0;
        }
    }

    fn sample(&mut self, sample_rate: f32) -> f32 {
        if !self.active {
            return 0.0;
        }

        // Handle fading
        if self.fade_speed > 0.0 {
            let dt = 1.0 / sample_rate;
            self.volume -= self.fade_speed * dt;
            if self.volume <= 0.0 {
                self.volume = 0.0;
                self.fade_speed = 0.0;
                self.active = false;
                return 0.0;
            }
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

    let spatial_data = world.resource::<SharedSpatialData>();
    let spatial = spatial_data.data.clone();

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

    // Counter for periodic spatial updates
    let mut spatial_update_counter = 0u32;

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

                // Periodically update spatial data (every ~1024 samples)
                spatial_update_counter += 1;
                if spatial_update_counter >= 1024 {
                    spatial_update_counter = 0;
                    if let Ok(spatial_data) = spatial.try_lock() {
                        audio_state.update_spatial(&spatial_data);
                    }
                }

                // Generate stereo audio
                for frame in data.chunks_mut(channels) {
                    let (left, right) = audio_state.generate_stereo_sample();
                    if channels >= 2 {
                        frame[0] = left;
                        frame[1] = right;
                        // Fill remaining channels with average
                        for channel in frame.iter_mut().skip(2) {
                            *channel = (left + right) * 0.5;
                        }
                    } else {
                        // Mono fallback
                        frame[0] = (left + right) * 0.5;
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

    info!(target: "lightwatch::audio", "Audio output initialized with spatial stereo");
}

/// System to sync spatial audio data from ECS to audio thread
pub fn sync_spatial_audio_data(
    shared: Res<SharedSpatialData>,
    sources: Query<(&SpatialAudioSource, &crate::travelers::Traveler)>,
) {
    let Ok(mut data) = shared.data.try_lock() else {
        return;
    };

    let mut total_gain = 0.0;
    let mut weighted_pan = 0.0;
    let mut active_count = 0.0;

    for (source, traveler) in sources.iter() {
        // Store per-traveler spatial data
        let idx = traveler.id as usize;
        if idx < data.travelers.len() {
            data.travelers[idx] = SpatialSourceData {
                gain: source.computed_gain,
                pan: source.computed_pan,
                pitch: source.computed_pitch,
            };
        }

        // Accumulate for master spatial
        if source.computed_gain > 0.01 {
            total_gain += source.computed_gain;
            weighted_pan += source.computed_pan * source.computed_gain;
            active_count += 1.0;
        }
    }

    // Calculate master spatial (weighted average)
    if active_count > 0.0 {
        data.master_gain = (total_gain / active_count).clamp(0.3, 1.0);
        data.master_pan = (weighted_pan / total_gain).clamp(-1.0, 1.0);
    } else {
        data.master_gain = 1.0;
        data.master_pan = 0.0;
    }
}

/// Audio output plugin
pub struct AudioOutputPlugin;

impl Plugin for AudioOutputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AudioTriggerQueue>()
            .insert_resource(SharedSpatialData {
                data: Arc::new(Mutex::new(SpatialMixData::default())),
            })
            .add_systems(Startup, init_audio_output)
            .add_systems(Update, sync_spatial_audio_data);
    }
}
