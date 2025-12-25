# Prompt 06: Implement Chromatic Aberration Post-Processing

## Priority: HIGH
## Dependency: None
## Estimated Scope: Medium

---

## Problem

Chromatic aberration is computed but never rendered. The material exists, values are calculated per-phase, but no render node applies the effect to the screen.

## Current State

- `src/post/materials.rs` - `ChromaticAberrationMaterial` defined
- `src/post/dynamic.rs` - `DynamicPostState` computes chromatic intensity:
  - Base: 0.002
  - During bang (3-6s): ramps to 0.015
  - Returns to base after settle
- Shader: `shaders/post_chromatic.wgsl` (23 lines)

Values computed each frame but never applied to rendering.

## Requirements

1. **Create post-processing render pipeline**:
   - Render main scene to texture
   - Apply chromatic aberration as full-screen pass
   - Output to screen

2. **Implement render node** or use Bevy's post-processing:
   - Bevy 0.14 approach: Custom post-process node
   - Alternative: 2D quad with material sampling scene texture

3. **Connect to DynamicPostState**:
   - Read `chromatic_intensity` from state
   - Apply to shader uniform

4. **Verify shader**:
   - RGB channel separation based on distance from center
   - Intensity controls separation amount

## Files to Create/Modify

- `src/post/mod.rs` - Add post-processing pipeline
- `src/post/chromatic.rs` - Render node for chromatic aberration
- `src/post/materials.rs` - Ensure material properly configured
- `src/shaders/sources.rs` - Verify shader

## Shader Logic

```wgsl
// Chromatic aberration
fn chromatic_aberration(uv: vec2<f32>, intensity: f32) -> vec3<f32> {
    let center = vec2<f32>(0.5, 0.5);
    let offset = (uv - center) * intensity;

    let r = textureSample(scene, sampler, uv + offset).r;
    let g = textureSample(scene, sampler, uv).g;
    let b = textureSample(scene, sampler, uv - offset).b;

    return vec3<f32>(r, g, b);
}
```

## Success Criteria

- [ ] Chromatic aberration visible during bang (3-6s)
- [ ] Effect intensity matches computed values
- [ ] RGB separation visible at screen edges
- [ ] Effect subtle during normal phases
- [ ] No performance regression

## Verification

1. Run `cargo run`
2. Start experience
3. During bang (3-6s), observe color fringing at screen edges
4. Effect should be strongest at peak, then fade
5. Compare bang visual impact with/without effect
