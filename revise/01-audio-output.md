# Prompt 01: Connect Audio Synthesis to Output

## Priority: CRITICAL
## Dependency: None
## Estimated Scope: Large

---

## Problem

The audio synthesis engine is complete but produces no sound. `AudioEngine::fill_buffer()` generates samples that go nowhere. `bevy_kira_audio` is declared in Cargo.toml but unused.

## Current State

- `src/audio/engine.rs` - Complete voice management, `fill_buffer()` implemented
- `src/audio/oscillator.rs` - Sine, Saw, Triangle, Square, Noise waveforms
- `src/audio/envelope.rs` - ADSR envelope generator
- `src/audio/filter.rs` - Biquad lowpass/highpass/bandpass
- `src/audio/voice.rs` - Polyphonic voice with oscillator + filter + envelopes
- `src/audio/reverb.rs` - Schroeder reverb (never applied)

All synthesis code works. No audio output exists.

## Requirements

1. **Create audio output system** that:
   - Maintains a real-time audio buffer
   - Calls `AudioEngine::fill_buffer()` each frame
   - Outputs samples to the audio device

2. **Choose one approach**:
   - **Option A**: Use `bevy_kira_audio` with a streaming source
   - **Option B**: Use `cpal` directly for raw audio output
   - **Option C**: Use Bevy's built-in audio with procedural source

3. **Integrate existing synthesis**:
   - Connect `EventSounds` (BangRumble, GriefDissonance, TransitionSound) to output
   - Connect `CosmicAmbiance` to output
   - Connect `LeitmotifPlayer` notes to output

4. **Apply effects chain**:
   - Reverb (already implemented, just needs connection)
   - SilenceManager volume multiplier

## Files to Modify

- `src/audio/mod.rs` - Add audio output plugin
- `src/audio/engine.rs` - Ensure fill_buffer is called
- `src/audio/events.rs` - Sample event sounds into buffer
- `src/audio/ambiance.rs` - Sample ambiance into buffer
- New file: `src/audio/output.rs` - Audio device output system

## Success Criteria

- [ ] Audio plays through speakers
- [ ] Bang rumble audible during expansion phase (3-6s)
- [ ] Grief dissonance audible when Child fades
- [ ] Phase transition tones play on each phase change
- [ ] Cosmic ambiance provides background texture
- [ ] Leitmotif melodies play for active travelers

## Technical Notes

For `bevy_kira_audio` approach:
```rust
// Example: Streaming procedural audio
use bevy_kira_audio::{AudioPlugin, AudioSource, AudioControl};

fn create_streaming_source(engine: &mut AudioEngine) -> impl AudioSource {
    // Create a streaming source that calls engine.fill_buffer()
}
```

For `cpal` approach:
```rust
// Example: Direct audio output
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn setup_audio_stream(engine: Arc<Mutex<AudioEngine>>) {
    let host = cpal::default_host();
    let device = host.default_output_device().unwrap();
    // Build stream that pulls from engine
}
```

## Verification

1. Run `cargo run`
2. Click to start experience
3. Listen for bang rumble at 3 seconds
4. Listen for phase transition tones
5. Confirm audio continues through experience
