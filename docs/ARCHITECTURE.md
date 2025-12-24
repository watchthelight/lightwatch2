# LIGHTWATCH: Architecture Overview

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                          LIGHTWATCH                                  │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐ │
│  │    Core     │  │   Visual    │  │    Audio    │  │  Narrative  │ │
│  │   Systems   │  │   Systems   │  │   Systems   │  │   Systems   │ │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘ │
│         │                │                │                │        │
│         └────────────────┴────────────────┴────────────────┘        │
│                                 │                                    │
│                          ┌──────┴──────┐                            │
│                          │  Event Bus  │                            │
│                          └──────┬──────┘                            │
│                                 │                                    │
│                          ┌──────┴──────┐                            │
│                          │    Clock    │                            │
│                          │   System    │                            │
│                          └─────────────┘                            │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

## Directory Structure

```
lightwatch/
├── Cargo.toml                 # Project manifest
├── Cargo.lock                 # Dependency lock
├── README.md                  # Project overview
├── CHANGELOG.md               # Version history
├── build.rs                   # Build script (optional)
│
├── src/
│   ├── main.rs               # Entry point
│   ├── lib.rs                # Library root
│   │
│   ├── core/                 # Core systems
│   │   ├── mod.rs
│   │   ├── clock.rs          # Timeline management
│   │   ├── state.rs          # State machine
│   │   ├── phase.rs          # Phase controller
│   │   ├── events.rs         # Event definitions
│   │   └── input.rs          # Input handling
│   │
│   ├── visual/               # Visual systems
│   │   ├── mod.rs
│   │   ├── camera/           # Camera systems
│   │   │   ├── mod.rs
│   │   │   ├── rig.rs
│   │   │   ├── behaviors.rs
│   │   │   ├── dof.rs
│   │   │   ├── shake.rs
│   │   │   └── transitions.rs
│   │   │
│   │   ├── traveler/         # Traveler rendering
│   │   │   ├── mod.rs
│   │   │   ├── components.rs
│   │   │   ├── geometry.rs
│   │   │   ├── materials.rs
│   │   │   ├── particles.rs
│   │   │   └── behaviors.rs
│   │   │
│   │   ├── environment/      # Environment rendering
│   │   │   ├── mod.rs
│   │   │   ├── starfield.rs
│   │   │   ├── nebula.rs
│   │   │   ├── dust.rs
│   │   │   ├── fog.rs
│   │   │   └── reflection.rs
│   │   │
│   │   ├── bang/             # Bang sequence
│   │   │   ├── mod.rs
│   │   │   ├── core.rs
│   │   │   ├── expansion.rs
│   │   │   ├── godrays.rs
│   │   │   ├── shockwave.rs
│   │   │   └── debris.rs
│   │   │
│   │   └── post/             # Post-processing
│   │       ├── mod.rs
│   │       ├── bloom.rs
│   │       ├── dof.rs
│   │       ├── aberration.rs
│   │       ├── grain.rs
│   │       └── tonemap.rs
│   │
│   ├── audio/                # Audio systems
│   │   ├── mod.rs
│   │   ├── synthesis.rs      # Sound generation
│   │   ├── leitmotifs.rs     # Traveler melodies
│   │   ├── spatial.rs        # 3D audio
│   │   └── events.rs         # Audio event handlers
│   │
│   └── narrative/            # Narrative systems
│       ├── mod.rs
│       ├── text.rs           # Text rendering
│       └── fragments.rs      # Transmission fragments
│
├── assets/                   # Runtime assets
│   ├── shaders/             # WGSL shaders
│   │   ├── traveler.wgsl
│   │   ├── nebula.wgsl
│   │   ├── particles.wgsl
│   │   └── post/
│   │       ├── bloom.wgsl
│   │       ├── dof.wgsl
│   │       └── grain.wgsl
│   │
│   └── fonts/               # Typography
│       └── mono.ttf
│
└── docs/                    # Documentation
    ├── VISION.md
    ├── TECH_STACK.md
    ├── TIMELINE.md
    ├── TRAVELERS.md
    ├── ARCHITECTURE.md
    └── ROADMAP.md
```

## ECS Architecture

LIGHTWATCH uses Bevy's Entity Component System architecture.

### Components

```rust
// Core identification
struct Traveler(TravelerId);
struct Phase(PhaseId);

// Transform
struct Position(Vec3);
struct Velocity(Vec3);
struct Scale(f32);

// Visual
struct Color(Color);
struct Luminosity(f32);
struct PulsePhase(f32);

// State
struct Alive(bool);
struct DeathProgress(f32);
struct ConnectionStrength(f32);

// Audio
struct AudioSource(Handle<AudioSink>);
struct Frequency(f32);
struct Volume(f32);
```

### Resources

```rust
// Time
struct Clock {
    elapsed: f32,          // 0.0 - 143.0
    phase: PhaseId,
    paused: bool,
}

// State
struct ExperienceState {
    current: State,
    previous: State,
}

// Camera
struct CameraRig {
    position: Vec3,
    target: Vec3,
    trauma: f32,
}

// Events
struct EventQueue {
    pending: Vec<LightwatchEvent>,
}
```

### Systems (Execution Order)

```
┌─────────────────────────────────────────────────────────────┐
│                      FRAME UPDATE                            │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  1. Input Systems                                            │
│     └── handle_input                                         │
│                                                              │
│  2. Time Systems                                             │
│     ├── update_clock                                         │
│     └── detect_phase_transitions                             │
│                                                              │
│  3. Event Systems                                            │
│     ├── emit_phase_events                                    │
│     └── process_events                                       │
│                                                              │
│  4. Simulation Systems                                       │
│     ├── update_traveler_positions                            │
│     ├── update_traveler_states                               │
│     ├── calculate_connections                                │
│     └── process_deaths                                       │
│                                                              │
│  5. Audio Systems                                            │
│     ├── update_synthesis                                     │
│     ├── update_spatial_audio                                 │
│     └── trigger_audio_events                                 │
│                                                              │
│  6. Visual Systems                                           │
│     ├── update_camera                                        │
│     ├── update_particles                                     │
│     ├── update_environment                                   │
│     └── update_post_processing                               │
│                                                              │
│  7. Render (Bevy handles this)                               │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

## State Machine

```
┌─────────┐    click    ┌─────────┐   t=143s   ┌─────────┐
│  Idle   │────────────▶│ Running │───────────▶│Complete │
└─────────┘             └─────────┘            └─────────┘
     │                       │                      │
     │                       │ ESC                  │
     │                       ▼                      │
     │                  ┌─────────┐                 │
     └──────────────────│ Paused  │◀────────────────┘
                        └─────────┘
                             │
                             │ ESC (hold)
                             ▼
                        ┌─────────┐
                        │  Quit   │
                        └─────────┘
```

## Event System

Events drive communication between systems.

```rust
enum LightwatchEvent {
    // State events
    ExperienceStarted,
    ExperiencePaused,
    ExperienceResumed,
    ExperienceCompleted,

    // Phase events
    PhaseChanged { from: PhaseId, to: PhaseId },

    // Traveler events
    TravelerSpawned(TravelerId),
    TravelerConnected(TravelerId, TravelerId),
    TravelerDisconnected(TravelerId, TravelerId),
    TravelerDying(TravelerId),
    TravelerDead(TravelerId),

    // Bang events
    BangStarted,
    BangPeaked,
    BangEnded,

    // Audio events
    HarmonyAchieved,
    HarmonyLost,
    SilenceBegun,
}
```

## Rendering Pipeline

```
┌─────────────────────────────────────────────────────────────┐
│                     RENDER STAGES                            │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  Stage 1: Background                                         │
│  ├── Clear to black                                          │
│  └── Render nebula (full-screen raymarched quad)            │
│                                                              │
│  Stage 2: Environment                                        │
│  ├── Render starfield (point sprites)                       │
│  └── Render dust particles (GPU instanced)                  │
│                                                              │
│  Stage 3: Bang (if active)                                   │
│  ├── Render explosion core                                   │
│  ├── Render shockwave                                        │
│  ├── Render debris particles                                 │
│  └── Render god rays                                         │
│                                                              │
│  Stage 4: Travelers                                          │
│  ├── Render core geometry (PBR)                             │
│  ├── Render aura particles                                   │
│  └── Render trail particles                                  │
│                                                              │
│  Stage 5: Volumetrics                                        │
│  └── Apply volumetric fog                                    │
│                                                              │
│  Stage 6: Post-Processing                                    │
│  ├── Depth of field                                          │
│  ├── Bloom (HDR)                                            │
│  ├── Chromatic aberration                                    │
│  ├── Film grain                                              │
│  └── ACES tonemapping                                        │
│                                                              │
│  Stage 7: UI (if any)                                        │
│  └── Render text overlays                                    │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

## Plugin Structure

Each major system is implemented as a Bevy plugin:

```rust
pub struct LightwatchPlugins;

impl PluginGroup for LightwatchPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            // Core
            .add(ClockPlugin)
            .add(StatePlugin)
            .add(PhasePlugin)
            .add(EventPlugin)
            .add(InputPlugin)
            // Visual
            .add(CameraPlugin)
            .add(TravelerPlugin)
            .add(EnvironmentPlugin)
            .add(BangPlugin)
            .add(PostProcessingPlugin)
            // Audio
            .add(AudioPlugin)
            // Narrative
            .add(NarrativePlugin)
    }
}
```

## Data Flow

```
Input ──▶ Clock ──▶ Phase ──▶ Events ──▶ Systems ──▶ Render
                      │                      ▲
                      │                      │
                      └──────────────────────┘
                         (phase parameters)
```

All systems read from the Clock to get current time and phase. The Phase system provides interpolated parameters (0.0-1.0) for smooth transitions. Events allow decoupled communication between systems.
