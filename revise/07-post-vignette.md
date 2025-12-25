# Prompt 07: Implement Vignette Post-Processing

## Priority: MEDIUM
## Dependency: 06-post-chromatic-aberration (shared pipeline)
## Estimated Scope: Small

---

## Problem

Vignette effect is computed but not rendered. Would add focus and cinematic framing.

## Current State

- `src/post/materials.rs` - `VignetteMaterial` defined
- `src/post/dynamic.rs` - Vignette state computed:
  - Base intensity: 0.3
  - Pulses ±0.1 at phase transitions
  - Higher during emotional moments
- Shader: `shaders/post_vignette.wgsl` (24 lines)

## Requirements

1. **Add vignette to post-processing chain**:
   - After chromatic aberration (or combined)
   - Darken corners/edges of screen

2. **Connect to DynamicPostState**:
   - Read `vignette_intensity`
   - Apply to shader

3. **Shader implementation**:
   - Radial falloff from center
   - Smooth gradient (not harsh cutoff)
   - Configurable intensity and radius

## Files to Modify

- `src/post/mod.rs` - Add to chain
- `src/post/vignette.rs` - Create if needed
- `src/post/dynamic.rs` - Verify state updates

## Shader Logic

```wgsl
fn vignette(uv: vec2<f32>, intensity: f32, radius: f32) -> f32 {
    let center = vec2<f32>(0.5, 0.5);
    let dist = length(uv - center);
    let vig = smoothstep(radius, radius - 0.2, dist);
    return mix(1.0, vig, intensity);
}
```

## Success Criteria

- [ ] Screen edges/corners are darkened
- [ ] Vignette intensity varies with phase
- [ ] Effect is subtle but present
- [ ] Draws focus to center of screen
- [ ] Works with bloom without fighting

## Verification

1. Run with chromatic aberration from Prompt 06
2. Observe darkening at screen corners
3. Effect should pulse slightly at phase changes
4. Should feel cinematic without being heavy-handed
