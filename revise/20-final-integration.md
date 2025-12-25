# Prompt 20: Final Integration and Testing

## Priority: HIGH
## Dependency: All previous prompts
## Estimated Scope: Large

---

## Problem

All systems need to work together cohesively. This prompt ensures complete integration and validates the full 143-second experience.

## Requirements

### Full Experience Validation

1. **Timeline Accuracy**:
   - [ ] Phase transitions at correct times (2s, 12s, 27s, 57s, 87s, 143s)
   - [ ] All moments trigger on schedule
   - [ ] No timing drift over 143 seconds

2. **Visual Continuity**:
   - [ ] Bang sequence visually dramatic (0-12s)
   - [ ] Travelers emerge with presence (12-27s)
   - [ ] Discovery phase has wonder (27-57s)
   - [ ] Connection phase feels warm (57-87s)
   - [ ] Acceptance phase is poignant (87-143s)
   - [ ] Fade to black at end (141-143s)

3. **Audio Continuity**:
   - [ ] Bang rumble during explosion
   - [ ] Phase transition tones audible
   - [ ] Leitmotifs play for travelers
   - [ ] Grief sound when Child fades
   - [ ] Cosmic ambiance throughout
   - [ ] Fade to silence at end

4. **Text Visibility**:
   - [ ] Signal detection sequence (0-2s)
   - [ ] Traveler reveals (0.5-3s)
   - [ ] Narrative fragments throughout
   - [ ] Grief text at 95s
   - [ ] Final messages at 130s+

5. **Post-Processing**:
   - [ ] Chromatic aberration during bang
   - [ ] Vignette throughout
   - [ ] Film grain for texture
   - [ ] Bloom responds to brightness
   - [ ] God rays during explosion

### Edge Cases

6. **State Transitions**:
   - [ ] Ready → Running on click
   - [ ] Running → Ending at 141s
   - [ ] Ending → Ended at 143s

7. **Traveler Lifecycle**:
   - [ ] All 5 spawn at correct times
   - [ ] Child fades first (95s)
   - [ ] Others fade in order
   - [ ] Grief triggers correctly

8. **Camera Behaviors**:
   - [ ] Drift during Awakening
   - [ ] Approach during Discovery
   - [ ] Static during Connection
   - [ ] Pullback during Acceptance

### Performance

9. **Frame Rate**:
   - [ ] 60 FPS minimum during normal phases
   - [ ] Acceptable during bang (may dip)
   - [ ] No memory leaks over 143 seconds

10. **Clean Shutdown**:
    - [ ] ESC exits cleanly
    - [ ] No error messages on exit
    - [ ] Audio stops cleanly

## Testing Procedure

1. **Full Run-Through**:
   - Start fresh
   - Do not use dev controls
   - Watch entire 143 seconds
   - Note any issues

2. **Phase-by-Phase**:
   - Use dev controls to jump to each phase
   - Verify all elements present
   - Check transitions

3. **Stress Test**:
   - Multiple runs without restart
   - Check for degradation
   - Verify memory stability

## Success Criteria

- [ ] Complete experience plays without errors
- [ ] All visual systems render correctly
- [ ] All audio systems produce sound
- [ ] All text displays properly
- [ ] Emotional arc feels complete
- [ ] Ready for release

## Final Checklist

Before marking complete:
- [ ] `cargo build --release` succeeds
- [ ] `cargo clippy` has no warnings
- [ ] `cargo test` passes (if tests exist)
- [ ] Full 143-second playthrough successful
- [ ] Experience is emotionally compelling
- [ ] Window is resizable
- [ ] Works on target platforms

## Verification

Run the complete experience 3 times:
1. First run: Note any issues
2. Second run: Verify issues fixed
3. Third run: Final validation

Document any remaining issues for future work.
