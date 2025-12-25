# Prompt 19: Production Polish and Cleanup

## Priority: MEDIUM
## Dependency: All previous prompts
## Estimated Scope: Medium

---

## Problem

Several debug/dev features could interfere with production experience. Some logging is not gated. Code has `#[allow(dead_code)]` that can be removed.

## Current State

- Timeline verification logs every frame if slow (not gated)
- Some systems log excessively in release builds
- `#[allow(dead_code)]` on now-used functions
- Unused shaders still loaded

## Requirements

1. **Gate timeline verification logging**:
   - `src/core/timeline_verify.rs` - Only log in debug builds
   - Or reduce log level to debug/trace

2. **Review all logging statements**:
   - Info level for important events only
   - Debug level for development info
   - Reduce release build verbosity

3. **Clean up dead_code allows**:
   - Remove from functions now being used
   - Keep only for intentionally unused scaffolding

4. **Remove unused shader loading**:
   - Don't load shaders that aren't used
   - Or verify all loaded shaders are used

5. **Performance review**:
   - Check for any debug-only overhead
   - Verify release builds are optimized

## Files to Review

- `src/core/timeline_verify.rs` - Gate logging
- `src/core/logging.rs` - Review log levels
- `src/shaders/loader.rs` - Review what's loaded
- All files with `#[allow(dead_code)]`

## Success Criteria

- [ ] Release builds have minimal console output
- [ ] No warnings in release build
- [ ] No unused code warnings
- [ ] Clean `cargo build --release` output
- [ ] Performance acceptable on target hardware

## Verification

1. `cargo build --release` - no warnings
2. Run release build - minimal console output
3. FPS stable throughout experience
4. Memory usage stable
