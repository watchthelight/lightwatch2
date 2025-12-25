# Prompt 12: Enable Cosmic Ambiance

## Priority: MEDIUM
## Dependency: 01-audio-output
## Estimated Scope: Small

---

## Problem

`CosmicAmbiance` generates continuous background texture but `.sample()` is never called.

## Current State

- `src/audio/ambiance.rs` - Complete implementation:
  - Rumble layer (30Hz sine, LowPass 60Hz)
  - Shimmer layer (800Hz sine, HighPass 2000Hz)
  - Noise layer (white noise, BandPass 400Hz)
  - Mixed at: rumble 50%, shimmer 10%, noise 5%
  - Master volume: 0.15

## Requirements

1. **Sample ambiance continuously**:
   - Start after experience begins
   - Play throughout all phases
   - Fade during final silence (139s+)

2. **Mix with other audio**:
   - Low volume (0.15) to stay behind other sounds
   - Apply to both channels equally (non-spatial)

3. **Phase-responsive modulation**:
   - Could modulate filter cutoffs per phase
   - Could adjust mix ratios per phase
   - Keep subtle throughout

## Files to Modify

- `src/audio/ambiance.rs` - Remove dead_code allow
- `src/audio/output.rs` - Sample and mix ambiance
- Optional: Phase-responsive modulation in `src/audio/events.rs`

## Success Criteria

- [ ] Continuous low-level background sound
- [ ] Subtle texture that doesn't compete with other audio
- [ ] Fades to silence at experience end
- [ ] Provides sense of cosmic space

## Verification

1. Run with audio output
2. Listen for subtle background during quiet moments
3. Should provide "hum of the universe" texture
4. Confirm it fades at experience end (139s+)
