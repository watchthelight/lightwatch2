# LIGHTWATCH v1.0.0 - Full Codebase Audit

## Executive Summary

LIGHTWATCH has **extensive architecture with critical output gaps**. The codebase contains ~35,000 lines of sophisticated Rust/Bevy code, but several systems compute values internally without producing visible/audible output.

**Current State: 40% functional, 60% scaffolded**

---

## Critical Blockers (Experience Breaking)

### 1. AUDIO: 0% Output
- **18 audio files** with complete synthesis engine (oscillators, filters, envelopes, voices, reverb)
- `BangRumble`, `GriefDissonance`, `TransitionSound`, `CosmicAmbiance` - all `.trigger()` but `.sample()` never called
- `AudioEngine::fill_buffer()` generates samples to nowhere
- `bevy_kira_audio` declared in Cargo.toml but **completely unused**
- Leitmotif system triggers notes but no audio device output
- Spatial audio computes gain/pan/pitch but never applies them
- **Result: Complete silence**

### 2. TEXT: 0% Visible
- Full typewriter system with states (Typing → Holding → Fading → Complete)
- Queue system with priorities and delays
- 16 narrative fragments, signal detection, grief text, final messages
- **Missing: Font handle** - `TextStyle { ..default() }` has no font loaded
- **Result: All text entities spawn but nothing renders**

### 3. WINDOW: Not Resizable
- `resizable: false` in window config
- Scale factor forced to 1.0 (breaks high-DPI displays)

---

## Major Gaps (Visual Quality)

### 4. Traveler Shaders: Defined But Unused
- `TravelerGlowMaterial` (Fresnel rim, pulse, inner glow) - **NOT USED**
- `TravelerShellMaterial` (refraction/transmission) - **NOT USED**
- `TravelerEdgeMaterial` (glowing wireframe) - **NOT USED**
- Travelers use generic `StandardMaterial` instead
- **Result: Travelers appear as flat, boring geometric shapes**

### 5. Post-Processing: Computed But Not Rendered
- Chromatic Aberration - materials defined, values computed, **no render node**
- Film Grain - materials defined, values computed, **no render node**
- Vignette - materials defined, values computed, **no render node**
- **Result: No cinematic post-processing effects**

### 6. God Rays: Scaffolded Only
- State computed (light position, intensity, decay)
- Comment: "Full integration deferred to prompt 39"
- **Result: Bang lacks screen-space radial light effect**

### 7. Depth of Field: Computed But Not Rendered
- Focus distance tracked per phase
- Shader exists (`dof.wgsl`)
- **No render node integration**

---

## Partially Working Systems

### 8. Bang Sequence: 80% Working
- Core light expansion ✓
- Expansion rings (5 total) ✓
- Debris particles (5000, brief) ✓
- Shockwave torus ✓
- `bang_core.wgsl` is **placeholder** (returns white)
- God rays missing

### 9. Environment: 70% Working
- Starfield (2000 stars) ✓ - reveals after bang
- Nebula background ✓ - if shader compiles
- Dust (10,000 particles) - **hidden most of experience**
- Fog system - scaffolded only
- Reflection system - scaffolded only

### 10. Particles: 75% Working
- Aura particles ✓ (orbiting motes)
- Trail particles ✓ (but disabled during Acceptance phase)
- Debris ✓ (but despawns within 8 seconds)

### 11. Camera: 90% Working
- Rig system ✓
- Breathing motion ✓
- Shake system ✓
- Transitions ✓
- Behavior blending has issues (momentum killed during transitions)
- DOF not rendering

---

## Working Systems

- Experience clock and phase system ✓
- State machine (Loading → Ready → Running → Ending → Ended) ✓
- Event system (all events fire correctly) ✓
- Traveler lifecycle (spawn → active → grieving → fading → gone) ✓
- Input handling (click to start, dev controls) ✓
- Bloom (dynamic, phase-responsive) ✓
- Tonemapping (ACES Fitted) ✓
- Debug overlay (properly gated) ✓
- Hot reload (debug only) ✓

---

## File Statistics

| Module | Files | Lines | Status |
|--------|-------|-------|--------|
| audio/ | 18 | ~2,500 | 0% output |
| text/ | 12 | ~1,200 | 0% visible |
| travelers/ | 15 | ~2,800 | 50% (missing shaders) |
| bang/ | 6 | ~800 | 80% |
| camera/ | 9 | ~1,100 | 90% |
| environment/ | 6 | ~700 | 70% |
| post/ | 5 | ~500 | 30% |
| core/ | 15 | ~2,000 | 95% |
| shaders/ | 4 + 9 wgsl | ~600 | 40% connected |
| **TOTAL** | 97 | ~35,000 | **40%** |

---

## Root Causes

1. **Deferred Integration**: Many systems marked "will be integrated in prompt X" but never were
2. **Architecture Over Output**: Excellent internal design but missing "last mile" to rendering/audio
3. **Missing Render Nodes**: Post-processing has materials but no composition pipeline
4. **No Audio Backend**: Synthesis engine never connected to bevy_kira_audio or cpal
5. **Font Not Loaded**: Simple oversight - TextStyle needs explicit font handle

---

## Revision Strategy

The 20 prompts in this folder are ordered by:
1. **Critical fixes** (audio output, text visibility, window) - Prompts 01-03
2. **Visual foundations** (traveler shaders, post-processing) - Prompts 04-08
3. **Audio completion** (spatial, effects, ambiance) - Prompts 09-12
4. **Visual polish** (particles, behaviors, environment) - Prompts 13-16
5. **Integration & testing** (final polish, performance, release) - Prompts 17-20

Each prompt is designed to be completable in a single session and produce measurable improvement.
