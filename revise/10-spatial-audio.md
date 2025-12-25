# Prompt 10: Connect Spatial Audio to Output

## Priority: MEDIUM
## Dependency: 01-audio-output
## Estimated Scope: Medium

---

## Problem

Spatial audio computes gain, pan, and pitch for sources but never applies these values to actual audio output.

## Current State

- `src/audio/spatial.rs` - Complete implementation:
  - `calculate_attenuation()` - distance falloff
  - `calculate_panning()` - stereo from 3D position
  - `calculate_doppler()` - pitch shift from velocity
  - `SpatialAudioSource` stores: `computed_gain`, `computed_pan`, `computed_pitch`
- Values updated each frame but never applied

## Requirements

1. **Apply spatial values to audio mixing**:
   - Multiply source audio by `computed_gain`
   - Apply stereo pan using `computed_pan`
   - Pitch shift using `computed_pitch` (optional, computationally expensive)

2. **Attach SpatialAudioSource to travelers**:
   - Each traveler should have spatial source
   - Leitmotif plays from traveler's position

3. **Position-aware mixing**:
   - Close travelers louder
   - Distant travelers quieter
   - Stereo field matches visual positions

## Files to Modify

- `src/audio/spatial.rs` - Apply values to output
- `src/audio/output.rs` (from Prompt 01) - Read spatial values when mixing
- Traveler spawn - Add SpatialAudioSource components

## Stereo Panning

```rust
fn apply_pan(sample: f32, pan: f32) -> (f32, f32) {
    // pan: -1.0 = full left, +1.0 = full right
    let left = sample * (1.0 - pan.max(0.0));
    let right = sample * (1.0 + pan.min(0.0));
    (left, right)
}
```

## Success Criteria

- [ ] Travelers sound louder when camera is close
- [ ] Travelers sound quieter when far
- [ ] Left-positioned travelers audible more in left speaker
- [ ] Right-positioned travelers audible more in right speaker
- [ ] Audio feels positioned in 3D space

## Verification

1. Run with audio output from Prompt 01
2. Use dev controls to move camera (arrow keys)
3. Notice volume changes as camera moves
4. Notice stereo positioning as camera rotates
5. Audio should feel spatially connected to visuals
