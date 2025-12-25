# Prompt 05: Activate Traveler Shell Shader

## Priority: HIGH
## Dependency: 04-traveler-glow-shader
## Estimated Scope: Medium

---

## Problem

The outer translucent shell layer uses `StandardMaterial` instead of `TravelerShellMaterial` which provides refraction and transmission effects for a glass-like ethereal appearance.

## Current State

- `src/travelers/shader_material.rs` - `TravelerShellMaterial` defined but unused
- `src/travelers/materials.rs:39-64` - `create_shell_material()` returns StandardMaterial with:
  - 30% alpha
  - `specular_transmission: 0.6`
  - `ior: 1.5` (glass-like)

Current shell is semi-transparent but lacks the custom refraction effects.

## Requirements

1. **Create TravelerShellMaterial** with properties:
   - Base color with low alpha
   - Refraction strength (distorts background)
   - Transmission amount
   - Pulse influence (subtle)

2. **Assign shell material to outer mesh layer**:
   - Each traveler has core + shell + edge layers
   - Shell surrounds core at larger radius

3. **Implement shell shader** (`shaders/traveler_shell.wgsl`):
   - Sample background with UV offset for refraction
   - Blend with shell color based on view angle
   - Subtle pulse-driven opacity variation

4. **Update uniforms each frame**:
   - `time` for animation
   - `refraction_strength` (per traveler or global)

## Files to Modify

- `src/travelers/shader_material.rs` - Complete TravelerShellMaterial
- `src/shaders/sources.rs` - Add or verify shell shader
- Traveler spawn code - Assign shell material to shell mesh

## Success Criteria

- [ ] Shell mesh uses TravelerShellMaterial
- [ ] Background distortion visible through shell
- [ ] Glass-like ethereal appearance
- [ ] Shell enhances rather than obscures core glow
- [ ] Works with bloom without excessive brightness

## Technical Notes

Refraction in fragment shader:
```wgsl
// Offset UV based on normal for refraction
let refracted_uv = uv + normal.xy * refraction_strength;
let background = textureSample(background_texture, sampler, refracted_uv);
```

Note: May need to use Bevy's transmission features rather than custom refraction depending on render pipeline limitations.

## Verification

1. Run with glow shader from Prompt 04
2. Observe shells around travelers
3. Background should appear slightly distorted through shell
4. Move camera to see refraction change with angle
5. Shell should have glass-like quality
