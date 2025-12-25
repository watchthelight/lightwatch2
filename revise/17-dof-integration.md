# Prompt 17: Integrate Depth of Field

## Priority: LOW
## Dependency: 06-post-chromatic-aberration (shared pipeline)
## Estimated Scope: Medium

---

## Problem

Depth of Field values are computed per-phase but the shader is not connected to the render pipeline.

## Current State

- `src/camera/dof.rs` - Focus distance computed:
  - Signal: 15.0, Bang: 5.0, Awakening: 12.0
  - Discovery: 10.0, Connection: 8.0
  - Acceptance: 25.0, Ended: 50.0
- `assets/shaders/dof.wgsl` - 69 lines, Bokeh-based blur
- Comment: "Will be fully integrated in Prompt 39"

## Requirements

1. **Add DOF to post-processing chain**:
   - Render scene with depth buffer
   - Apply DOF blur based on depth vs focus distance

2. **Connect to DofState**:
   - Read `focus_distance`, `aperture`, `focal_length`
   - Calculate circle of confusion per pixel

3. **Bokeh effect**:
   - Blur amount varies with distance from focus
   - Hexagonal or circular bokeh shape
   - Smooth falloff at focus boundary

## Technical Approach

```wgsl
fn circle_of_confusion(depth: f32, focus: f32, aperture: f32, focal_length: f32) -> f32 {
    let coc = abs(depth - focus) * aperture * focal_length / (depth * focus);
    return clamp(coc, 0.0, max_blur);
}
```

## Success Criteria

- [ ] Objects at focus distance are sharp
- [ ] Objects far from focus are blurred
- [ ] Blur amount matches phase-appropriate focus
- [ ] Transitions smoothly between focus distances
- [ ] Performance acceptable (DOF is expensive)

## Verification

1. Run with post-processing pipeline
2. During Discovery (27s+), near travelers sharp, far things blurry
3. During Acceptance (87s+), wider focus, more in focus
4. Smooth transitions between phases
