# Changelog

All notable changes to LIGHTWATCH will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.3] - 2024-12-24

### Added
- `src/core/logging.rs` - Wide event logging system with structured context
  - WideEvent struct with key-value fields
  - WideValue enum for typed context values
  - `wide_event!` macro for event creation
- `src/core/debug_overlay.rs` - In-game debug overlay
  - Shows FPS, elapsed time, phase, traveler/particle counts
  - F3 to toggle visibility
  - Visible by default in debug builds
- `src/core/hot_reload.rs` - Hot reload configuration
  - HotReloadConfig resource
  - F5 manual reload trigger (debug only)
- `src/core/time_control.rs` - Development time control
  - Space to pause/resume
  - [ and ] to adjust playback speed
  - Number keys 1-6, 0 to jump to phases
- FrameTimeDiagnosticsPlugin for FPS tracking
- Tracing subscriber with structured log output

### Changed
- Updated `src/core/mod.rs` with new development modules
- Updated `src/main.rs` with tracing initialization

### Notes
- Prompt 04-DEVELOPMENT-TOOLS complete
- Debug tools only active in debug builds
- Release builds remain optimized and quiet
- Binary size: 26MB

## [0.1.2] - 2024-12-24

### Added
- 9 placeholder WGSL shaders embedded in binary:
  - `traveler_glow.wgsl` - Pulsing emissive effect
  - `nebula.wgsl` - Cosmic background
  - `star_twinkle.wgsl` - Flickering starfield
  - `post_chromatic.wgsl` - Chromatic aberration
  - `post_grain.wgsl` - Film grain
  - `post_vignette.wgsl` - Vignette effect
  - `bang_core.wgsl` - Explosion core
  - `bang_shockwave.wgsl` - Expanding ring distortion
  - `bang_godray.wgsl` - Volumetric light beams
- `src/shaders/sources.rs` - Embedded shader sources via include_str!
- `src/shaders/loader.rs` - Shader loading system with ShaderHandles resource
- `src/shaders/material.rs` - TravelerGlowMaterial custom material
- `ShadersPlugin` for shader and material registration

### Notes
- Prompt 03-ASSET-PIPELINE complete
- All shaders compiled into binary (no runtime file loading)
- Binary size: 25MB (reasonable)
- ShaderHandles resource provides access to all shader handles

## [0.1.1] - 2024-12-24

### Added
- `src/core/window.rs` - Window configuration with ESC close and F11 fullscreen toggle
- `src/core/renderer.rs` - CinematicCameraBundle with HDR, ACES tonemapping, bloom
- `src/core/exposure.rs` - Dynamic exposure control tied to experience phases
- Render pipeline configuration with high-performance GPU preference
- 4x MSAA anti-aliasing

### Changed
- Updated `src/main.rs` to use new window and render configuration
- Updated `src/core/mod.rs` to integrate new modules

### Notes
- Prompt 02-WINDOW-RENDERER complete
- HDR enabled with ACES Filmic tonemapping
- VSync active via AutoVsync present mode
- Window fixed at 1920x1080, non-resizable

## [0.1.0] - 2024-12-24

### Added
- `Cargo.toml` with Bevy 0.14 and all dependencies
- `src/main.rs` with Bevy app skeleton and window configuration
- `src/lib.rs` with module re-exports and prelude
- `src/core/mod.rs` - CorePlugin stub
- `src/camera/mod.rs` - CameraPlugin stub
- `src/travelers/mod.rs` - TravelersPlugin stub with TravelerId enum
- `src/environment/mod.rs` - EnvironmentPlugin stub
- `src/bang/mod.rs` - BangPlugin stub
- `src/audio/mod.rs` - AudioPlugin stub
- `src/narrative/mod.rs` - NarrativePlugin stub
- `src/post/mod.rs` - PostPlugin stub
- `src/shaders/mod.rs` - Shader utilities module
- `assets/shaders/` directory for WGSL shaders
- `lightwatch` executable script
- `.gitignore` for build artifacts
- `Cargo.lock` for reproducible builds

### Notes
- Prompt 01-PROJECT-SCAFFOLD complete
- `cargo check` passes
- `cargo build --release` succeeds
- Window opens with black background at 1920x1080
- All module stubs compile

## [0.0.1] - 2024-12-24

### Added
- `docs/VISION.md` - Project vision and philosophy
- `docs/TECH_STACK.md` - Technology stack documentation
- `docs/TIMELINE.md` - 143-second phase breakdown
- `docs/TRAVELERS.md` - Five traveler character documentation
- `docs/ARCHITECTURE.md` - ECS architecture overview
- `docs/ROADMAP.md` - 40-prompt development roadmap
- Expanded README.md with full project overview

### Notes
- Framework prompt (00-FRAMEWORK) complete
- All foundational documentation in place

## [0.0.0] - 2024-12-24

### Added
- Initial project structure
- README.md with project placeholder
- CHANGELOG.md for version tracking

### Notes
- This marks the beginning of LIGHTWATCH development
- A 143-second real-time art piece built with Bevy (Rust)

[Unreleased]: https://github.com/watchthelight/lightwatch2/compare/v0.1.3...HEAD
[0.1.3]: https://github.com/watchthelight/lightwatch2/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/watchthelight/lightwatch2/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/watchthelight/lightwatch2/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/watchthelight/lightwatch2/compare/v0.0.1...v0.1.0
[0.0.1]: https://github.com/watchthelight/lightwatch2/compare/v0.0.0...v0.0.1
[0.0.0]: https://github.com/watchthelight/lightwatch2/releases/tag/v0.0.0
