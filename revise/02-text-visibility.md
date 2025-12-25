# Prompt 02: Fix Text Rendering - Load Font

## Priority: CRITICAL
## Dependency: None
## Estimated Scope: Small

---

## Problem

All text entities spawn correctly but nothing renders because no font is loaded. The `TextStyle` uses `..default()` which provides no font handle.

## Current State

- `src/text/spawn.rs:29-33`:
```rust
TextStyle {
    font_size: config.font_size,
    color: config.text_color,
    ..default()  // NO FONT!
}
```

- Text state machine works perfectly (Typing → Holding → Fading → Complete)
- Queue system works
- Positioning works
- Just no visible output

## Requirements

1. **Add a font asset** to the project:
   - Create `assets/fonts/` directory
   - Add a suitable monospace or clean sans-serif font
   - Suggested: JetBrains Mono, Source Code Pro, or Inter

2. **Load the font** in TextConfig or spawn system:
```rust
pub struct TextConfig {
    pub font: Handle<Font>,
    pub font_size: f32,
    // ...
}
```

3. **Apply font to all text spawns**:
   - `spawn_transmission()` in `src/text/spawn.rs`
   - Any other text spawning locations

4. **Verify all text types render**:
   - Signal detection ("SIGNAL DETECTED")
   - Traveler reveals ("• THE ARCHIVIST")
   - Narrative fragments
   - Grief text
   - Final messages

## Files to Modify

- `src/text/config.rs` - Add font handle to config
- `src/text/spawn.rs` - Use font from config
- `src/text/mod.rs` - Load font asset at startup
- Add: `assets/fonts/[fontname].ttf`

## Success Criteria

- [ ] Font file exists in assets/fonts/
- [ ] Font loads without errors at startup
- [ ] "SIGNAL DETECTED" visible at 0.5s
- [ ] Traveler names reveal with typewriter effect
- [ ] Narrative fragments display during experience
- [ ] Grief text appears when Child fades
- [ ] Final messages display at 130s+

## Technical Notes

Font loading in Bevy 0.14:
```rust
fn setup_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/my_font.ttf");
    commands.insert_resource(TextConfig {
        font,
        font_size: 24.0,
        // ...
    });
}
```

Using font in TextStyle:
```rust
TextStyle {
    font: config.font.clone(),
    font_size: config.font_size,
    color: config.text_color,
}
```

## Font Recommendations

For the contemplative/cosmic aesthetic:
- **JetBrains Mono** - Clean, readable, slightly technical
- **IBM Plex Mono** - Humanist monospace, warm
- **Space Mono** - Geometric, space-age feel
- **Inter** - If preferring sans-serif over monospace

## Verification

1. Run `cargo run`
2. Wait for "click to begin"
3. Click to start
4. Observe "SIGNAL DETECTED" in top-left
5. Watch traveler names reveal sequentially
6. Confirm text fades correctly after hold duration
