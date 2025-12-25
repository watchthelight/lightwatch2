# Prompt 03: Fix Window Configuration

## Priority: HIGH
## Dependency: None
## Estimated Scope: Small

---

## Problem

Window is not resizable and forces scale factor 1.0, breaking high-DPI displays.

## Current State

`src/core/window.rs`:
```rust
Window {
    resizable: false,  // Should be true
    resolution: WindowResolution::new(1920., 1080.)
        .with_scale_factor_override(1.0),  // Breaks high-DPI
    // ...
}
```

## Requirements

1. **Enable window resizing**:
   - Set `resizable: true`
   - Handle aspect ratio gracefully (letterboxing or camera adjustment)

2. **Fix high-DPI support**:
   - Remove `with_scale_factor_override(1.0)`
   - Let system determine appropriate scale factor

3. **Optional: Add fullscreen toggle for release builds**:
   - Currently F11 is debug-only
   - Consider making it available in release

4. **Ensure UI scales with window**:
   - Text positioning uses absolute coordinates
   - May need to convert to viewport-relative positioning

## Files to Modify

- `src/core/window.rs` - Fix window settings
- `src/text/config.rs` - Make positions viewport-relative (optional)
- `src/core/ready_screen.rs` - Ensure "click to begin" scales

## Success Criteria

- [ ] Window can be resized by dragging edges
- [ ] Text and UI remain readable on 4K/Retina displays
- [ ] Experience remains visually correct at different window sizes
- [ ] No UI elements clip off screen edges

## Technical Notes

Viewport-relative positioning:
```rust
// Instead of absolute coordinates
let offset = Vec2::new(-40.0, 40.0);

// Use window-relative
fn get_position(window: &Window, position: TextPosition) -> Vec2 {
    let half_width = window.width() / 2.0;
    let half_height = window.height() / 2.0;
    let margin = 50.0;

    match position {
        TextPosition::TopLeft => Vec2::new(-half_width + margin, half_height - margin),
        // ...
    }
}
```

## Verification

1. Run `cargo run`
2. Drag window edges to resize
3. Confirm 3D scene scales appropriately
4. Confirm text remains positioned correctly
5. Test on high-DPI display if available
