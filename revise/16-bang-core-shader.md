# Prompt 16: Complete Bang Core Shader

## Priority: MEDIUM
## Dependency: None
## Estimated Scope: Medium

---

## Problem

`bang_core.wgsl` is a placeholder that just returns white. The bang core should have dynamic color, intensity, and visual complexity.

## Current State

- `assets/shaders/bang_core.wgsl` - 18 lines, returns fixed white
- Comment: "Will be expanded in Prompt 28"
- `src/bang/core.rs` - Computes intensity, temperature, color
- Values computed but shader ignores them

## Requirements

1. **Implement proper bang core shader**:
   - Accept uniforms: time, intensity, temperature
   - White-hot at peak, cooling to amber/red
   - Pulsing/flickering effect
   - Radial glow from center

2. **Connect to BangCore component values**:
   - Pass computed values as uniforms
   - Update each frame during bang

3. **Visual effects**:
   - Core should pulse with intensity
   - Color should shift with temperature
   - Radial falloff for glow effect
   - Optional: Noise for organic look

## Shader Features

```wgsl
// Temperature to color (blackbody approximation)
fn temperature_to_color(temp: f32) -> vec3<f32> {
    // 1.0 = white hot, 0.0 = dark red
    let r = 1.0;
    let g = pow(temp, 0.5);
    let b = pow(temp, 1.5);
    return vec3<f32>(r, g, b);
}

// Radial glow
fn radial_glow(uv: vec2<f32>, intensity: f32) -> f32 {
    let dist = length(uv - 0.5) * 2.0;
    return pow(1.0 - dist, 2.0) * intensity;
}
```

## Success Criteria

- [ ] Bang core has dynamic color (white → amber → red)
- [ ] Intensity drives brightness (exceeds 1.0 for bloom)
- [ ] Radial glow emanates from center
- [ ] Core feels like violent cosmic event

## Verification

1. Run experience
2. Watch bang sequence (2-10s)
3. Core should start white-hot and cool
4. Should trigger intense bloom
5. Should feel like universe-creating explosion
