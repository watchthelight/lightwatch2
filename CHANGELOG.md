# Changelog

All notable changes to LIGHTWATCH will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.1] - 2024-12-24

### Added
- `assets/shaders/nebula.wgsl` - Raymarched nebula shader
  - 3D gradient noise with quintic interpolation
  - fbm (4 octaves) for volumetric cloud density
  - raymarch_nebula with front-to-back compositing (24 steps)
  - Drift animation for slow cosmic motion
  - Amber/violet two-color gradient
- `src/environment/nebula.rs` - Nebula system
  - NebulaMaterial with color1/color2, time, intensity, drift, noise scale
  - NebulaConfig resource for runtime adjustment
  - update_nebula_intensity: phase-driven (fade in after bang, out during acceptance)
  - spawn_nebula_background: large quad at z=-500
  - NebulaPlugin registers material and systems

### Notes
- Prompt 24-NEBULA-RAYMARCHED complete
- 60% overall progress

## [0.5.0] - 2024-12-24

### Added
- `src/environment/starfield.rs` - 2000-star field system
  - Star component with brightness, twinkle frequency, phase offset
  - StarfieldConfig resource (2000 stars, 50-200m distance range)
  - Fibonacci sphere distribution for even star placement
  - spawn_starfield creates stars hidden at scale zero
  - reveal_stars gradually shows stars by distance after bang (8s start)
  - fade_in_stars smoothly increases opacity post-reveal
  - update_stars applies twinkle animation and material updates
  - fade_stars_during_acceptance fades stars in second half of Acceptance
  - StarfieldPlugin and StarfieldAssets for mesh/material caching
- StarMarker component for efficient querying

### Notes
- Prompt 23-STARFIELD complete
- Phase 5 (Environment) begun
- 57.5% overall progress

## [0.4.6] - 2024-12-24

### Added
- `src/travelers/behavior/` - Complete behavior system
  - pulse.rs: rhythmic pulsing with per-traveler shape curves
  - sync.rs: synchronization during Connection, desync during Acceptance
  - drift.rs: organic Perlin-like movement with phase multipliers
  - anchor.rs: formation positions during Connection phase
  - orbit.rs: orbital motion with tilted planes
  - grief.rs: tremor, pulse disruption, per-traveler responses
- TravelerBehaviorPlugin registers all behavior systems
- TravelerDrift, TravelerAnchor, TravelerOrbit components
- Character-specific pulse shapes (Archivist=smooth, Child=quick, Other=subtle)
- Soft bounds pushing travelers back if too far from center

### Notes
- Prompt 22-TRAVELER-BEHAVIORS complete
- Phase 4 (Traveler Rendering) complete!
- 55% overall progress

## [0.4.5] - 2024-12-24

### Added
- `src/travelers/particles/trails.rs` - Motion trail particle system
  - TravelerTrail component with per-traveler configuration
  - PositionHistory tracks positions with timestamps
  - TrailParticle with spawn_time, fade_duration, size
  - setup_traveler_trails: adds trail components on spawn
  - update_position_history: tracks movement with min_distance threshold
  - spawn_trail_particles: creates particles at recent positions
  - update_trail_particles: handles fade, shrink, and cleanup
  - control_trail_activation: enables trails during movement phases
  - TrailMeshCache for shared trail mesh
- Traveler-specific trail configurations:
  - Archivist: 20 points, 1.5s fade
  - Wanderer: 30 points, 2.0s fade (longer trails)
  - Keeper: 15 points, 1.0s fade
  - Child: 25 points, 0.8s fade (quick)
  - Other: 40 points, 3.0s fade (persistent)

### Notes
- Prompt 21-TRAVELER-PARTICLES-TRAILS complete
- 52.5% overall progress

## [0.4.4] - 2024-12-24

### Added
- `src/travelers/particles/aura.rs` - Orbiting aura particle system
  - TravelerAura component with per-traveler configuration
  - AuraParticle with orbital mechanics (angle, radius, tilt, phase)
  - spawn_aura_particles: spawns particles as children of travelers
  - animate_aura_particles: orbital motion with pulse influence
  - control_particle_density: adjusts visibility based on pulse
  - fade_aura_with_traveler: fades particles during traveler fade
- `src/travelers/particles/mod.rs` - TravelerParticlesPlugin
- Traveler-specific aura configurations:
  - Archivist: 60 particles, amber, steady orbit
  - Wanderer: 40 particles, cyan, fast/erratic
  - Keeper: 30 particles, deep orange, slow/steady
  - Child: 80 particles, white, fast/playful
  - Other: 20 particles, indigo, very slow/sparse

### Notes
- Prompt 20-TRAVELER-PARTICLES-AURA complete
- 50% overall progress

## [0.4.3] - 2024-12-24

### Added
- Custom WGSL shaders for ethereal traveler rendering
  - `traveler_glow.wgsl`: Fresnel rim, inner glow, pulse, grief
  - `traveler_shell.wgsl`: translucent shell with iridescence
  - `traveler_edge.wgsl`: glowing wireframe with pulse
- `src/travelers/shader_material.rs` - Custom material types
  - TravelerGlowMaterial with Fresnel power, rim color
  - TravelerShellMaterial with refraction, IOR
  - TravelerEdgeMaterial with glow intensity
  - update_shader_time system
  - sync_pulse_to_shader_materials system
  - TravelerShaderPlugin

### Notes
- Prompt 19-TRAVELER-SHADERS complete
- 47.5% overall progress

## [0.4.2] - 2024-12-24

### Added
- `src/travelers/materials.rs` - PBR material system
  - create_core_material: metallic, emissive
  - create_shell_material: translucent, specular transmission
  - create_edge_material: unlit, 2x emissive glow
  - TravelerLayer enum (Core, Shell, Edge)
  - TravelerMaterialCache for handle caching
  - PulsingMaterial component for dynamic emissive
  - update_pulsing_materials system
  - evolve_materials_for_phase (Connection/Acceptance color shift)
  - apply_grief_to_materials (desaturation during grief)

### Notes
- Prompt 18-TRAVELER-MATERIALS complete
- 45% overall progress

## [0.4.1] - 2024-12-24

### Added
- `src/travelers/geometry.rs` - Procedural Platonic solid mesh generation
  - generate_icosahedron (Archivist - 20 faces)
  - generate_tetrahedron (Wanderer - 4 faces)
  - generate_cube (Keeper - 6 faces)
  - generate_octahedron (Child - 8 faces)
  - generate_dodecahedron (Other - 12 faces)
  - Proper normals and spherical UV projection
  - Subtle vertex noise for organic feel
  - TravelerLayers: core, shell (1.1x), edges (wireframe)
  - TravelerMeshCache for handle caching

### Notes
- Prompt 17-TRAVELER-GEOMETRY complete
- 42.5% overall progress

## [0.4.0] - 2024-12-24

### Added
- `src/travelers/identity.rs` - Traveler identity system
  - Traveler component with id, name, spawn/fade times
  - TravelerDef with static definitions for all 5 travelers
  - TravelerGeometry: Icosahedron, Tetrahedron, Cube, Octahedron, Dodecahedron
  - TravelerColor with base, evolved, final_state
  - TravelerRhythm with frequency and variance
- `src/travelers/state.rs` - Traveler state components
  - TravelerState: Spawning, Active, Grieving, Fading, Gone
  - TravelerVisibility with smooth opacity transitions
  - TravelerPulse for rhythm tracking
  - TravelerGrief for mourning state
- `src/travelers/spawn.rs` - Spawn system
  - TravelerBundle for entity creation
  - handle_traveler_spawns from events
  - TravelerRegistry resource
- `src/travelers/lifecycle.rs` - Lifecycle systems
  - Visibility updates, spawn finalization
  - Fading and death handling
  - Grief events and decay

### Notes
- Prompt 16-TRAVELER-COMPONENTS complete
- Phase 4 (Traveler Rendering) begun
- 40% overall progress

## [0.3.4] - 2024-12-24

### Added
- `src/camera/transitions.rs` - Cinematic camera transition system
  - CameraTransition with position, rotation, duration, easing
  - EasingFunction enum (7 functions: Linear, EaseIn/Out, Cubic, Quart, Expo)
  - ActiveTransition resource for state tracking
  - TransitionPresets for phase-specific choreography:
    - awakening_drift (8s)
    - discovery_approach (20s)
    - connection_settle (10s)
    - acceptance_pullback (50s)
  - TriggerTransitionEvent for manual control

### Notes
- Prompt 15-CAMERA-TRANSITIONS complete
- Phase 3 (Camera) complete!
- 37.5% overall progress

## [0.3.3] - 2024-12-24

### Added
- `src/camera/shake.rs` - Trauma-based shake system
  - CameraShake with trauma accumulation and decay
  - Perlin noise (fbm_noise) for organic motion
  - ShakePresets: BANG_PEAK, GRIEF, SUBTLE, IMPACT
  - Affects both position and rotation
  - Responds to CameraShakeEvent

### Notes
- Prompt 14-CAMERA-SHAKE complete
- 35% overall progress

## [0.3.2] - 2024-12-24

### Added
- `src/camera/dof.rs` - Depth of field system
  - DepthOfFieldSettings with focus distance, aperture, blur
  - Phase-driven focus (Bang=5, Connection=8, Acceptance=25)
  - Smooth focus interpolation
  - CameraFocusEvent handling
- `assets/shaders/dof.wgsl` - Bokeh DOF shader (placeholder)
  - 13-sample circular bokeh kernel
  - Circle of confusion calculation

### Notes
- Prompt 13-CAMERA-DOF complete
- DOF render node deferred to post-processing prompt

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

[Unreleased]: https://github.com/watchthelight/lightwatch2/compare/v0.5.1...HEAD
[0.5.1]: https://github.com/watchthelight/lightwatch2/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/watchthelight/lightwatch2/compare/v0.4.6...v0.5.0
[0.4.6]: https://github.com/watchthelight/lightwatch2/compare/v0.4.5...v0.4.6
[0.4.5]: https://github.com/watchthelight/lightwatch2/compare/v0.4.4...v0.4.5
[0.4.4]: https://github.com/watchthelight/lightwatch2/compare/v0.4.3...v0.4.4
[0.4.3]: https://github.com/watchthelight/lightwatch2/compare/v0.4.2...v0.4.3
[0.4.2]: https://github.com/watchthelight/lightwatch2/compare/v0.4.1...v0.4.2
[0.4.1]: https://github.com/watchthelight/lightwatch2/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/watchthelight/lightwatch2/compare/v0.3.4...v0.4.0
[0.3.4]: https://github.com/watchthelight/lightwatch2/compare/v0.3.3...v0.3.4
[0.3.3]: https://github.com/watchthelight/lightwatch2/compare/v0.3.2...v0.3.3
[0.3.2]: https://github.com/watchthelight/lightwatch2/compare/v0.3.1...v0.3.2
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
