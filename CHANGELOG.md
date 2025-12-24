# Changelog

All notable changes to LIGHTWATCH will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/watchthelight/lightwatch2/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/watchthelight/lightwatch2/compare/v0.0.1...v0.1.0
[0.0.1]: https://github.com/watchthelight/lightwatch2/compare/v0.0.0...v0.0.1
[0.0.0]: https://github.com/watchthelight/lightwatch2/releases/tag/v0.0.0
