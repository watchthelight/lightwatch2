# Prompt 08: Implement Film Grain Post-Processing

## Priority: MEDIUM
## Dependency: 06-post-chromatic-aberration (shared pipeline)
## Estimated Scope: Small

---

## Problem

Film grain is computed but not rendered. Adds cinematic texture and organic feel.

## Current State

- `src/post/materials.rs` - `FilmGrainMaterial` defined
- `src/post/dynamic.rs` - Grain state computed:
  - Base intensity with 0.5 response
  - Higher at start (×1.5) and end (×1.5-2.0)
- Shader: `shaders/post_grain.wgsl` (24 lines)

## Requirements

1. **Add film grain to post-processing chain**:
   - After chromatic aberration and vignette
   - Overlay noise pattern on final image

2. **Implement animated grain**:
   - Noise pattern changes each frame
   - Grain follows screen pixels, not world

3. **Connect to DynamicPostState**:
   - Read `grain_intensity`
   - Apply time-varying seed for animation

## Shader Logic

```wgsl
fn random(uv: vec2<f32>, seed: f32) -> f32 {
    return fract(sin(dot(uv + seed, vec2<f32>(12.9898, 78.233))) * 43758.5453);
}

fn film_grain(color: vec3<f32>, uv: vec2<f32>, intensity: f32, time: f32) -> vec3<f32> {
    let noise = random(uv * 1000.0, time) * 2.0 - 1.0;
    return color + noise * intensity;
}
```

## Success Criteria

- [ ] Subtle grain visible over entire image
- [ ] Grain animates (different pattern each frame)
- [ ] Intensity higher at start and end of experience
- [ ] Doesn't obscure important visuals
- [ ] Adds organic/filmic quality

## Verification

1. Run with previous post-processing effects
2. Look for subtle noise texture over image
3. Should be most visible on flat/dark areas
4. Intensity should vary with phase
