# Prompt 18: Apply Reverb to Audio Output

## Priority: LOW
## Dependency: 01-audio-output
## Estimated Scope: Small

---

## Problem

Reverb is fully implemented but never applied to the audio signal chain.

## Current State

- `src/audio/reverb.rs` - Complete Schroeder reverb:
  - 8 parallel comb filters
  - 4 series allpass filters
  - Mix control (0.3 = 30% wet)
  - Process method ready to use

## Requirements

1. **Add reverb to output chain**:
   - After mixing all sources
   - Before final output to device

2. **Configure for cosmic space**:
   - Long decay (cosmic emptiness)
   - High diffusion (smooth tails)
   - Moderate mix (30% wet)

3. **Phase-responsive reverb** (optional):
   - More reverb during Connection (warmth)
   - Less during Bang (clarity)
   - Fade out during silence (139s+)

## Files to Modify

- `src/audio/output.rs` - Add reverb processing
- `src/audio/reverb.rs` - Remove dead_code allow

## Signal Chain

```
Sources → Mix → Reverb → Output
         ↓
   [bang, grief, transitions, leitmotif, ambiance]
```

## Success Criteria

- [ ] Audio has spatial depth
- [ ] Reverb tails audible on transients
- [ ] Sounds feel like they exist in cosmic space
- [ ] Mix appropriate (not overwhelming)

## Verification

1. Run with full audio
2. Listen to transition tones - should have reverb tail
3. Listen to bang rumble - should have depth
4. Overall audio should feel spacious
