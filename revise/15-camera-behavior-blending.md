# Prompt 15: Fix Camera Behavior Blending

## Priority: MEDIUM
## Dependency: None
## Estimated Scope: Small

---

## Problem

Camera behavior transitions kill momentum. When transitioning between behaviors (Drift → Approach → Pullback), blend becomes 0.0, causing jerky movement.

## Current State

- `src/camera/behavior.rs`:
  - Blend factor drops to 0 during transition period
  - Approach/Pullback only apply when `state.current == behavior`
  - Transition interpolates blend from 0 → 1, but behavior offset also lerps to 0

## Issues

```rust
// Line 165: Only applies when current == Approach
// During transition, current is still old behavior, so this doesn't run
// Meanwhile, old behavior is lerping to 0
// Result: Camera stops moving during transition
```

## Requirements

1. **Smooth behavior crossfade**:
   - Old behavior fades out while new fades in
   - Maintain total movement throughout transition
   - No sudden stops or jerks

2. **Interpolate between behavior offsets**:
   - Compute offset for both old and new behavior
   - Blend based on transition progress

3. **Or simplify to immediate switch**:
   - If blending is too complex, instant switch is acceptable
   - As long as positions are continuous

## Files to Modify

- `src/camera/behavior.rs` - Fix blend logic

## Success Criteria

- [ ] Camera moves smoothly through behavior transitions
- [ ] No jerky stops during Drift → Approach transition
- [ ] No jerky stops during Approach → Pullback transition
- [ ] Camera motion feels organic throughout

## Verification

1. Run experience
2. Watch camera during phase transitions (12s, 27s, 57s, 87s)
3. Camera should move smoothly without sudden stops
4. Compare to before fix - should feel more fluid
