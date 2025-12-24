# LIGHTWATCH: Development Roadmap

## Overview

LIGHTWATCH is built through 40 sequential prompts, each adding a specific capability. This document tracks progress and provides an overview of the development journey.

## Progress Legend

- [ ] Not started
- [~] In progress
- [x] Complete

---

## Phase 1: Foundation (Prompts 01-05)

Building the Rust/Bevy project scaffold and development environment.

| # | Prompt | Status | Description |
|---|--------|--------|-------------|
| 01 | PROJECT-SCAFFOLD | [x] | Cargo.toml, directory structure, build script |
| 02 | WINDOW-RENDERER | [x] | Native window, HDR, MSAA |
| 03 | ASSET-PIPELINE | [x] | Asset loading, embedded assets, shaders |
| 04 | DEVELOPMENT-TOOLS | [x] | Hot reload, debug overlay, logging |
| 05 | BUILD-SYSTEM | [x] | Release optimization, single binary |

**Milestone**: Running window with development tools

---

## Phase 2: Core Systems (Prompts 06-10)

The backbone of the experience—timing, state, and events.

| # | Prompt | Status | Description |
|---|--------|--------|-------------|
| 06 | CLOCK-SYSTEM | [x] | 143-second timeline, phase detection |
| 07 | STATE-MACHINE | [x] | Experience states, phase enum |
| 08 | EVENT-BUS | [x] | Custom events, wide event emission |
| 09 | PHASE-CONTROLLER | [x] | Phase transitions, lerped params |
| 10 | INPUT-HANDLER | [x] | Click to start, disable after |

**Milestone**: Time-driven experience that responds to input

---

## Phase 3: Camera (Prompts 11-15)

The viewer's eye into the cosmic experience.

| # | Prompt | Status | Description |
|---|--------|--------|-------------|
| 11 | CAMERA-RIG | [x] | Perspective camera, breathing motion |
| 12 | CAMERA-BEHAVIORS | [x] | Drift, approach, pullback modes |
| 13 | CAMERA-DOF | [x] | Bokeh depth of field shader |
| 14 | CAMERA-SHAKE | [x] | Trauma-based shake system |
| 15 | CAMERA-TRANSITIONS | [x] | Cinematic sequences |

**Milestone**: Cinematic camera that breathes and responds

---

## Phase 4: Traveler Rendering (Prompts 16-22)

Bringing the five travelers to visual life.

| # | Prompt | Status | Description |
|---|--------|--------|-------------|
| 16 | TRAVELER-COMPONENTS | [x] | ECS components, spawn system |
| 17 | TRAVELER-GEOMETRY | [x] | Procedural platonic solid meshes |
| 18 | TRAVELER-MATERIALS | [x] | PBR materials, emissive glow |
| 19 | TRAVELER-SHADERS | [x] | Custom WGSL shaders |
| 20 | TRAVELER-PARTICLES-AURA | [x] | Orbiting particles |
| 21 | TRAVELER-PARTICLES-TRAILS | [x] | Movement trails |
| 22 | TRAVELER-BEHAVIORS | [x] | Rhythm, sync, grief animations |

**Milestone**: Five unique travelers with full visual treatment

---

## Phase 5: Environment (Prompts 23-27)

The cosmic backdrop for the travelers' story.

| # | Prompt | Status | Description |
|---|--------|--------|-------------|
| 23 | STARFIELD | [x] | 2000 stars with twinkle |
| 24 | NEBULA-RAYMARCHED | [x] | Full-screen raymarched background |
| 25 | NEBULA-DUST | [x] | 10,000 GPU dust particles |
| 26 | VOLUMETRIC-FOG | [x] | Exponential height fog |
| 27 | REFLECTION-PLANE | [x] | Planar reflection surface |

**Milestone**: Immersive cosmic environment

---

## Phase 6: The Bang (Prompts 28-32)

The explosive origin moment.

| # | Prompt | Status | Description |
|---|--------|--------|-------------|
| 28 | BANG-CORE | [x] | Central explosion effect |
| 29 | BANG-EXPANSION | [x] | Radial expansion animation |
| 30 | BANG-GOD-RAYS | [x] | Screen-space volumetric rays |
| 31 | BANG-SHOCKWAVE | [x] | Expanding torus distortion |
| 32 | BANG-DEBRIS | [x] | 5000 debris particles |

**Milestone**: Overwhelming visual spectacle for origin

---

## Phase 7: Audio (Prompts 33-36)

The voices of the travelers.

| # | Prompt | Status | Description |
|---|--------|--------|-------------|
| 33 | AUDIO-SYNTHESIS | [x] | Oscillators, filters, envelopes |
| 34 | AUDIO-LEITMOTIFS | [x] | Pentatonic melodies per traveler |
| 35 | AUDIO-SPATIAL | [x] | 3D audio positioning, reverb |
| 36 | AUDIO-EVENTS | [x] | Bang, grief, silence triggers |

**Milestone**: Emotional audio landscape

---

## Phase 8: Narrative (Prompts 37-38)

Transmission fragments and text.

| # | Prompt | Status | Description |
|---|--------|--------|-------------|
| 37 | TEXT-TRANSMISSION | [x] | Text rendering system |
| 38 | TEXT-FRAGMENTS | [x] | Signal overlay, transmission text |

**Milestone**: Narrative layer complete

---

## Phase 9: Post-Processing (Prompt 39)

Final visual polish.

| # | Prompt | Status | Description |
|---|--------|--------|-------------|
| 39 | POST-PROCESSING | [x] | Bloom, chromatic aberration, grain, ACES |

**Milestone**: AAA visual quality

---

## Phase 10: Integration (Prompt 40)

Final assembly and shipping.

| # | Prompt | Status | Description |
|---|--------|--------|-------------|
| 40 | FINAL-INTEGRATION | [x] | Polish, testing, executable build |

**Milestone**: Shippable experience

---

## Version Milestones

| Version | Prompts | Description |
|---------|---------|-------------|
| v0.1.0 | 01-05 | Foundation complete |
| v0.2.0 | 06-10 | Core systems complete |
| v0.3.0 | 11-15 | Camera complete |
| v0.4.0 | 16-22 | Travelers complete |
| v0.5.0 | 23-27 | Environment complete |
| v0.6.0 | 28-32 | Bang complete |
| v0.7.0 | 33-36 | Audio complete |
| v0.8.0 | 37-38 | Narrative complete |
| v0.9.0 | 39 | Post-processing complete |
| v1.0.0 | 40 | Release |

---

## Current Status

**Last completed prompt**: 40-FINAL-INTEGRATION
**Status**: COMPLETE
**Overall progress**: 40/40 (100%)

---

*LIGHTWATCH v1.0.0 Released*

*They built beacons that would outlast their stars.*
*We receive their light long after they are gone.*
