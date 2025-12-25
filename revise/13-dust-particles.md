# Prompt 13: Fix Dust Particle Visibility

## Priority: MEDIUM
## Dependency: None
## Estimated Scope: Small

---

## Problem

10,000 dust particles spawn and update every frame but are hidden most of the experience, wasting performance for no visual benefit.

## Current State

- `src/environment/dust.rs`:
  - 10,000 particles spawned at startup
  - Start with `Visibility::Hidden`
  - Only visible during Awakening phase
  - Update positions every frame even when hidden

## Requirements

1. **Expand dust visibility window**:
   - Start revealing after bang settles (8s+)
   - Keep visible through Connection phase
   - Fade during Acceptance

2. **Or reduce to lazy spawning**:
   - Don't spawn all 10,000 at startup
   - Spawn progressively during reveal
   - Despawn when no longer needed

3. **Phase-responsive density**:
   - Sparse during Awakening (emerging)
   - Dense during Discovery/Connection
   - Fading during Acceptance

## Option A: Extended Visibility

```rust
fn update_dust_visibility(phase: Phase) -> bool {
    matches!(phase,
        Phase::Awakening | Phase::Discovery | Phase::Connection
    )
}
```

## Option B: Progressive Spawn

```rust
// Spawn 500 particles per second during reveal
// Despawn during fade-out
```

## Success Criteria

- [ ] Dust visible during more of the experience
- [ ] Performance acceptable (no frame drops from 10k particles)
- [ ] Dust adds to cosmic atmosphere
- [ ] Proper fade transitions

## Verification

1. Run experience
2. After bang (8s+), dust should become visible
3. Should remain visible through Connection (87s)
4. Should fade during Acceptance
5. Check FPS during peak dust visibility
