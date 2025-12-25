# Prompt 11: Sample Event Sounds Into Output

## Priority: MEDIUM
## Dependency: 01-audio-output
## Estimated Scope: Medium

---

## Problem

Event sounds (BangRumble, GriefDissonance, TransitionSound) trigger correctly but `.sample()` is never called, so they produce no audio.

## Current State

- `src/audio/events.rs` - Systems call `.trigger()`:
  - Bang expansion → `BangRumble.trigger()`
  - Child fades → `GriefDissonance.trigger()`
  - Phase changes → `TransitionSound.trigger_for_phase()`

- Each sound generator has `.sample()` marked `#[allow(dead_code)]`

## Requirements

1. **Call `.sample()` on active event sounds**:
   - Check `active` flag each frame
   - Generate samples into output buffer
   - Advance internal time

2. **Mix event sounds with main audio**:
   - BangRumble: Sub-bass (30Hz) + mid (60Hz) + noise
   - GriefDissonance: Dissonant cluster (A3, Bb3, B3) + tremolo
   - TransitionSound: Single tone per phase

3. **Respect duration and envelopes**:
   - BangRumble: 6 seconds with filter sweep
   - GriefDissonance: 4 seconds with staggered releases
   - TransitionSound: Quick envelope per tone

## Files to Modify

- `src/audio/events.rs` - Add sampling system
- `src/audio/output.rs` - Mix event sounds
- Remove `#[allow(dead_code)]` from sample methods

## Mixing

```rust
fn sample_event_sounds(
    sounds: &mut EventSounds,
    sample_rate: f32,
    delta_time: f32,
) -> f32 {
    let bang = sounds.bang_rumble.sample(sample_rate, delta_time);
    let grief = sounds.grief.sample(sample_rate, delta_time);
    let transition = sounds.transitions.sample(sample_rate, delta_time);

    bang + grief + transition
}
```

## Success Criteria

- [ ] Deep rumble audible during bang expansion (3-6s)
- [ ] Dissonant grief sound when Child fades (95s)
- [ ] Transition tones at each phase boundary
- [ ] Sounds have proper attack/decay envelopes
- [ ] No clipping when multiple sounds overlap

## Verification

1. Run with audio output
2. Listen for rumble during bang sequence
3. Listen for grief sound at 95s (dev skip with 6 key)
4. Notice transition tones at 2s, 12s, 27s, 57s, 87s
