# Prompt 04: Activate Traveler Glow Shader

## Priority: HIGH
## Dependency: None
## Estimated Scope: Medium

---

## Problem

Travelers use generic `StandardMaterial` instead of the custom `TravelerGlowMaterial` that provides Fresnel rim lighting, pulse effects, and inner glow. This makes travelers appear flat and boring.

## Current State

- `src/travelers/shader_material.rs` - Defines `TravelerGlowMaterial` with:
  - `base_color`, `pulse_intensity`, `time`, `pulse_phase`
  - Shader: `shaders/traveler_glow.wgsl`
  - Plugin registered but material never assigned to entities

- `src/travelers/materials.rs` - Creates `StandardMaterial` for travelers
- `src/shaders/sources.rs` - `traveler_glow.wgsl` embedded (66 lines)

Travelers spawn with StandardMaterial, ignoring the custom shader entirely.

## Requirements

1. **Locate where traveler meshes are created** and materials assigned

2. **Replace StandardMaterial with TravelerGlowMaterial** for traveler cores:
   - Create TravelerGlowMaterial per traveler with correct color
   - Assign to core mesh entity

3. **Update material uniforms each frame**:
   - Set `time` from elapsed seconds
   - Set `pulse_intensity` from `TravelerPulse` component
   - Set `pulse_phase` per traveler for variation

4. **Verify shader compiles and renders**:
   - Check `traveler_glow.wgsl` for any issues
   - Ensure Fresnel effect is visible from all angles
   - Confirm pulse causes brightness variation

## Files to Modify

- `src/travelers/shader_material.rs` - May need uniform update system
- `src/travelers/spawn.rs` or wherever meshes are created
- `src/shaders/sources.rs` - Verify shader code

## Shader Features (from traveler_glow.wgsl)

```wgsl
// Expected features:
- Fresnel rim lighting (bright edges based on view angle)
- Pulse-driven brightness modulation
- Inner glow (core always emits regardless of lighting)
- Per-traveler color
```

## Success Criteria

- [ ] Travelers use TravelerGlowMaterial instead of StandardMaterial
- [ ] Fresnel rim is visible (edges glow brighter when viewed edge-on)
- [ ] Pulse causes visible brightness changes
- [ ] Each traveler has distinct color matching their identity
- [ ] Glow visible even from distance

## Color Reference

```rust
Archivist: Amber (0.91, 0.64, 0.27)
Wanderer: Cyan (0.31, 0.80, 0.77)
Keeper: Orange (0.83, 0.46, 0.18)
Child: White (0.96, 0.94, 0.91)
Other: Violet (0.42, 0.36, 0.58)
```

## Verification

1. Run `cargo run`
2. Start experience, wait for travelers to spawn (12s+)
3. Observe Archivist - should have amber glow with bright rim
4. Move camera (dev controls) to see Fresnel effect from different angles
5. Watch for pulse brightness variation
6. Compare to boring flat appearance before fix
