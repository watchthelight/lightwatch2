# Changelog

All notable changes to LIGHTWATCH will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.1] - 2024-12-24

### Added
- `src/camera/behavior.rs` - Camera behavior system
  - CameraBehaviorState tracks current/previous with transitions
  - BehaviorParams for drift, approach, pullback tuning
  - Smooth ease_in_out_cubic blending between behaviors
  - Static, Drift, Approach, Pullback modes
- Behaviors respond to CameraBehaviorChangedEvent from phase controller

### Notes
- Prompt 12-CAMERA-BEHAVIORS complete
- 30% overall progress milestone

## [0.3.0] - 2024-12-24

### Added
- `src/camera/rig.rs` - Camera rig system
  - CameraRig component with position/rotation offsets
  - ExperienceCamera marker component
  - spawn_camera with HDR, bloom, ACES tonemapping
  - apply_rig_to_transform for frame updates
- `src/camera/breathing.rs` - Organic breathing motion
  - BreathingConfig with dual-frequency parameters
  - update_breathing system for subtle sway
- `src/camera/config.rs` - Camera configuration resource
- CameraPlugin fully implemented

### Notes
- Prompt 11-CAMERA-RIG complete
- Phase 3 (Camera) begun
- Camera breathes with organic dual-frequency motion

## [0.2.4] - 2024-12-24

### Added
- `src/core/input.rs` - Input handler system
  - InputConfig resource (enabled, hide_cursor)
  - handle_click_to_start (any mouse button)
  - Cursor hides during experience, restores on end
  - Extended dev controls: R reset, arrow scrubbing
  - F1 dev help display
  - Keys 7/8 for near-end phase jumps
- InputPlugin for centralized input handling

### Changed
- Removed redundant start_on_click from state.rs

### Notes
- Prompt 10-INPUT-HANDLER complete
- Phase 2 (Core Systems) complete!
- 25% overall progress milestone

## [0.2.3] - 2024-12-24

### Added
- `src/core/phase_controller.rs` - Experience orchestration
  - Moment struct for scheduled actions
  - MomentAction enum with all action types
  - 40+ moments across all 6 phases
  - process_moments system for event dispatch
  - reset_controller_on_restart for dev mode
- PhaseControllerPlugin for centralized moment scheduling

### Notes
- Prompt 09-PHASE-CONTROLLER complete
- Complete 143-second timeline defined
- All systems wired via event dispatch

## [0.2.2] - 2024-12-24

### Added
- `src/core/events.rs` - Complete event catalog
  - TravelerId enum (Archivist, Wanderer, Keeper, Child, Other)
  - Traveler events: Spawned, Pulse, Fading, Faded, Grief, Synced
  - Camera events: BehaviorChanged, Shake, Focus
  - Audio events: PlayNote, PlayLeitmotif, AudioLayer
  - Narrative events: DisplayText, HideText, SignalOverlay
  - Visual events: Bang, Glitch, Environment
  - MomentEvent for phase-specific triggers
- EventsPlugin for centralized event registration

### Changed
- TravelerId moved from travelers/mod.rs to core/events.rs
- travelers/mod.rs now re-exports TravelerId from core

### Notes
- Prompt 08-EVENT-BUS complete
- 18 event types registered
- Decoupled architecture for system communication

## [0.2.1] - 2024-12-24

### Added
- `src/core/state.rs` - Experience state machine
  - ExperienceState: Loading → Ready → Running → Ending → Ended
  - StateChangedEvent for state transitions
  - System sets: ReadySet, RunningSet, EndingSet
  - Run conditions: in_ready_state, in_running_state, etc.
  - EndingTimer for 3-second fade-out
- `src/core/ready_screen.rs` - "click to begin" overlay
  - Black screen with centered text
  - Despawns when experience starts

### Changed
- Clock now runs only in Running state
- State transitions logged via wide events

### Notes
- Prompt 07-STATE-MACHINE complete
- Click to start experience functionality working

## [0.2.0] - 2024-12-24

### Added
- `src/core/clock.rs` - Experience clock system
  - Phase enum with 6 phases (Signal, Bang, Awakening, Discovery, Connection, Acceptance)
  - ExperienceClock resource with 143-second timeline
  - Phase detection and transition tracking
  - PhaseChangedEvent for phase transitions
  - Dev-only pause, resume, time scaling, and jump controls
- Wide event logging for phase transitions

### Changed
- Debug overlay now shows real clock data (elapsed, phase, progress, running state)
- DebugOverlayState simplified (removed redundant phase/elapsed fields)

### Notes
- Prompt 06-CLOCK-SYSTEM complete
- Phase 2 (Core Systems) begun
- Clock integrates with TimeControl from prompt 04

## [0.1.4] - 2024-12-24

### Added
- `build.rs` - Build script passing TARGET env to binary
- `src/core/build_info.rs` - BuildInfo for version/target/profile at runtime
- `./dev` script - Quick debug run with dynamic linking
- `./dist` script - Build and package for distribution
- `.github/workflows/build.yml` - CI for Linux, macOS, Windows
- `dev` feature for Bevy dynamic linking

### Changed
- Enhanced `./lightwatch` script with --build, --debug, --dev, --help options
- Cargo.toml release profile: lto = "fat" for maximum optimization
- Added release-fast profile with thin LTO for faster builds
- BuildInfo::log_info() now used at startup

### Notes
- Prompt 05-BUILD-SYSTEM complete
- Phase 1 (Foundation) complete!
- Binary size: 24MB (down from 26MB with fat LTO)
- Single self-contained executable

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

[Unreleased]: https://github.com/watchthelight/lightwatch2/compare/v0.3.1...HEAD
[0.3.1]: https://github.com/watchthelight/lightwatch2/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/watchthelight/lightwatch2/compare/v0.2.4...v0.3.0
[0.2.4]: https://github.com/watchthelight/lightwatch2/compare/v0.2.3...v0.2.4
[0.2.3]: https://github.com/watchthelight/lightwatch2/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/watchthelight/lightwatch2/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/watchthelight/lightwatch2/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/watchthelight/lightwatch2/compare/v0.1.4...v0.2.0
[0.1.4]: https://github.com/watchthelight/lightwatch2/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/watchthelight/lightwatch2/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/watchthelight/lightwatch2/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/watchthelight/lightwatch2/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/watchthelight/lightwatch2/compare/v0.0.1...v0.1.0
[0.0.1]: https://github.com/watchthelight/lightwatch2/compare/v0.0.0...v0.0.1
[0.0.0]: https://github.com/watchthelight/lightwatch2/releases/tag/v0.0.0
