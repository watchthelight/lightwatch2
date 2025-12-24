# LIGHTWATCH: Technology Stack

## Core Technologies

| Layer | Technology | Version |
|-------|------------|---------|
| Language | Rust | stable |
| Engine | Bevy | 0.14 |
| Rendering | Bevy PBR + Custom WGSL Shaders | — |
| Audio | bevy_kira_audio + Custom Synthesis | — |
| Build System | Cargo | — |
| Output | Native Binary | — |

## Why Rust + Bevy?

### Rust

- **Performance**: Zero-cost abstractions, no garbage collector
- **Safety**: Memory safety without runtime overhead
- **Expressiveness**: Pattern matching, traits, and algebraic types
- **Reliability**: If it compiles, it probably works

### Bevy

- **ECS Architecture**: Clean separation of data (Components), logic (Systems), and state (Resources)
- **First-class Shaders**: Native WGSL support with hot reloading
- **Modern Rendering**: HDR, bloom, tonemapping, MSAA built-in
- **Compute Shaders**: GPU-accelerated particle systems
- **Developer Experience**: Hot reloading, fast iteration

## Rendering Pipeline

```
┌─────────────────────────────────────────────────────────────┐
│                    LIGHTWATCH RENDERER                      │
├─────────────────────────────────────────────────────────────┤
│  Scene                                                      │
│  ├── Starfield (2000 points)                                │
│  ├── Nebula (raymarched background)                         │
│  ├── Dust Particles (10,000 GPU particles)                  │
│  ├── Travelers (5 entities, custom shaders)                 │
│  │   ├── Core Geometry (procedural meshes)                  │
│  │   ├── Aura Particles (orbiting)                          │
│  │   └── Trail Particles (movement)                         │
│  ├── Bang Effects (during Phase 2)                          │
│  │   ├── Core Explosion                                     │
│  │   ├── God Rays                                           │
│  │   ├── Shockwave                                          │
│  │   └── Debris (5000 particles)                            │
│  └── Volumetric Fog                                         │
├─────────────────────────────────────────────────────────────┤
│  Post-Processing                                            │
│  ├── Depth of Field (Bokeh)                                 │
│  ├── Bloom (HDR)                                            │
│  ├── Chromatic Aberration                                   │
│  ├── Film Grain                                             │
│  └── ACES Tonemapping                                       │
└─────────────────────────────────────────────────────────────┘
```

## Audio Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    AUDIO SYNTHESIS                          │
├─────────────────────────────────────────────────────────────┤
│  Per Traveler                                               │
│  ├── Base Oscillator (sine/triangle)                        │
│  ├── Frequency (unique per traveler)                        │
│  ├── Amplitude Envelope                                     │
│  └── Leitmotif Generator                                    │
├─────────────────────────────────────────────────────────────┤
│  Global                                                     │
│  ├── Spatial Audio (3D positioning)                         │
│  ├── Reverb (cathedral-scale)                               │
│  ├── Master Mix                                             │
│  └── Event-driven Triggers                                  │
│      ├── Bang                                               │
│      ├── Connection                                         │
│      ├── Grief (traveler death)                             │
│      └── Final Silence                                      │
└─────────────────────────────────────────────────────────────┘
```

## Build Targets

| Target | Platform | Notes |
|--------|----------|-------|
| Primary | macOS (ARM64) | Development platform |
| Secondary | macOS (x86_64) | Intel Mac support |
| Future | Windows | If needed |
| Future | Linux | If needed |
| Future | WebGPU | Browser deployment |

## Dependencies (Planned)

```toml
[dependencies]
bevy = "0.14"
bevy_kira_audio = "0.20"

[dev-dependencies]
bevy-inspector-egui = "0.25"  # Debug UI
```

## Performance Targets

| Metric | Target |
|--------|--------|
| Frame Rate | 60 FPS locked |
| Resolution | Native display |
| GPU Memory | < 512 MB |
| Binary Size | < 50 MB |
| Startup Time | < 2 seconds |
