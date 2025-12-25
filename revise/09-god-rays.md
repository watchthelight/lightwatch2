# Prompt 09: Implement God Rays During Bang

## Priority: HIGH
## Dependency: 06-post-chromatic-aberration (shared pipeline)
## Estimated Scope: Medium

---

## Problem

God rays are scaffolded but marked "deferred to prompt 39". The bang sequence lacks the dramatic radial light effect that would make it visually impactful.

## Current State

- `src/bang/god_rays.rs` - State computed:
  - Light position projected to screen space
  - Intensity ramps during expansion (0 → 1.0)
  - Decay: 0.96, Samples: 50
  - Timeline: 3.0s - 6.0s
- Shader: `shaders/bang_godray.wgsl` (26 lines, loaded but unused)

## Requirements

1. **Create god ray render node**:
   - Sample from light source position toward each pixel
   - Accumulate brightness along ray
   - Blend with scene

2. **Integrate with bang timeline**:
   - Start at expansion phase (3.0s)
   - Peak at bang peak (4.0s)
   - Fade during settling (6.0s)

3. **Connect to GodRaysState**:
   - Read light screen position
   - Read intensity and decay
   - Apply during correct time window

## Shader Logic

```wgsl
fn god_rays(uv: vec2<f32>, light_pos: vec2<f32>, intensity: f32, decay: f32, samples: i32) -> f32 {
    let delta = (uv - light_pos) / f32(samples);
    var current_uv = uv;
    var illumination = 0.0;
    var weight = 1.0;

    for (var i = 0; i < samples; i++) {
        current_uv -= delta;
        let sample = textureSample(scene, sampler, current_uv).rgb;
        illumination += luminance(sample) * weight;
        weight *= decay;
    }

    return illumination * intensity;
}
```

## Success Criteria

- [ ] Radial light rays emanate from bang center during expansion
- [ ] Rays visible from 3-6 seconds
- [ ] Peak intensity at 4 seconds (bang peak)
- [ ] Smooth fade as bang settles
- [ ] Dramatic visual enhancement of bang sequence

## Verification

1. Run experience
2. During bang (3-6s), observe radial light rays from center
3. Rays should pulse/intensify at peak
4. Should add significant visual drama to explosion
