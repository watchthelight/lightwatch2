# Prompt 14: Enable Trail Particles During Acceptance

## Priority: LOW
## Dependency: None
## Estimated Scope: Small

---

## Problem

Trail particles are disabled during Acceptance phase, making travelers feel static during the longest and most emotional phase.

## Current State

- `src/travelers/particles/trails.rs`:
  - Trails active during Discovery, Connection
  - Disabled during Acceptance
  - Makes fading travelers look less ethereal

## Requirements

1. **Enable trails during Acceptance**:
   - Keep trails on for active/fading travelers
   - May want faster fade for ethereal feel

2. **Adjust trail properties for Acceptance**:
   - Shorter duration (more ephemeral)
   - Lower opacity (ghostly)
   - Possibly different color (cooler tones)

3. **Trail fading with traveler**:
   - As traveler opacity decreases, trails should too
   - Trails should linger briefly after traveler fades

## Files to Modify

- `src/travelers/particles/trails.rs` - Extend phase mask
- Possibly `src/travelers/particles/mod.rs`

## Success Criteria

- [ ] Trails visible during Acceptance phase
- [ ] Trails enhance ethereal feeling of fading
- [ ] Trails fade gracefully with traveler opacity
- [ ] No performance issues from extended trails

## Verification

1. Jump to Acceptance phase (dev key 6)
2. Observe travelers - should have motion trails
3. Watch as travelers fade - trails should enhance ghostly feel
4. Compare to previous static appearance
