# Changelog

All notable changes to LIGHTWATCH will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.9.0] - 2024-12-24

### Added
- `src/post/config.rs` - Post-processing configuration
  - PostProcessConfig: master control for all effects
  - BloomConfig: intensity, threshold, composite mode
  - ChromaticAberrationConfig: base and max intensity
  - FilmGrainConfig: intensity, brightness response
  - VignetteConfig: intensity, midpoint, softness
- `src/post/bloom.rs` - Dynamic bloom control
  - update_bloom_for_bang: intensity increases during bang peak
  - Smooth ramp up (3-4s) and decay (4-6s)
- `src/post/dynamic.rs` - Dynamic post-processing state
  - DynamicPostProcess: tracks current effect intensities
  - update_chromatic_aberration: spikes during bang
  - update_film_grain: stronger at start/end for "old film" feel
  - update_vignette: pulses at phase transitions
- `src/post/materials.rs` - Custom material definitions
  - ChromaticAberrationMaterial: RGB channel offset shader interface
  - FilmGrainMaterial: procedural noise with time
  - VignetteMaterial: radial darkening
- `assets/shaders/chromatic_aberration.wgsl` - CA shader
  - Edge-based intensity (squared falloff)
  - RGB channel separation
- `assets/shaders/film_grain.wgsl` - Grain shader
  - Hash-based procedural noise
  - Brightness-responsive grain reduction
- `assets/shaders/vignette.wgsl` - Vignette shader
  - Smooth radial falloff
  - Configurable midpoint and softness
- PostPlugin integrates all post-processing systems

### Notes
- Prompt 39-POST-PROCESSING complete
- **Phase 9 (Post-Processing) COMPLETE**
- 97.5% overall progress

## [0.8.1] - 2024-12-24

### Added
- `src/text/fragments.rs` - Traveler text fragments
  - TravelerFragment: text + traveler + phase timing
  - 15 fragments across 5 travelers with phase constraints
  - traveler_display_name(): THE ARCHIVIST, THE WANDERER, etc.
- `src/text/signal.rs` - Signal detection overlay
  - SignalConfig: reveal_interval (0.4s), reveal_order
  - SignalState: tracks detection progress
  - start_signal_detection: "SIGNAL DETECTED" at 0.5s
  - reveal_travelers: names appear one-by-one
- `src/text/fragment_display.rs` - Fragment display system
  - FragmentState: shown fragments, timing, seeded RNG
  - display_fragments: random selection within phase windows
  - Quoted text with traveler attribution
  - 8-20 second randomized intervals
- `src/text/grief.rs` - Grief text for Child
  - GriefTextState: tracks if grief shown
  - show_grief_text: triggers on TravelerFadedEvent
  - "the child fades first / as the youngest always do"
- `src/text/final_messages.rs` - Experience ending text
  - FinalMessageState: tracks message sequence
  - show_final_messages: starts at 130s
  - "They built beacons / that would outlast their stars"
  - "We receive their light / long after they are gone"
- FragmentsPlugin registers all narrative systems

### Notes
- Prompt 38-TEXT-FRAGMENTS complete
- **Phase 8 (Narrative) COMPLETE**
- 95% overall progress

## [0.8.0] - 2024-12-24

### Added
- `src/text/config.rs` - Text display configuration
  - TextConfig: chars_per_second, hold_duration, fade_duration
  - font_size (24.0), text_color, glow_color
  - TextPosition enum: 9 screen positions (TopLeft → BottomRight)
  - to_justify(): maps position to JustifyText
  - to_offset(): returns screen offset coordinates
- `src/text/transmission.rs` - Transmission component
  - TransmissionState: Typing, Holding, Fading, Complete
  - Transmission: full_text, revealed_chars, opacity, priority
  - with_position(), with_priority(), with_hold(), with_speed() builders
  - visible_text(): returns currently revealed substring
  - typing_complete(): checks if all characters revealed
- `src/text/queue.rs` - Transmission queue
  - TransmissionQueue: ordered display with priority sorting
  - enqueue(): adds transmission with optional delay
  - next(): returns ready transmission, respects minimum_gap
  - update(): decrements timers
- `src/text/typewriter.rs` - Typewriter effect
  - update_typewriter: character-by-character reveal
  - Typing→Holding→Fading→Complete state machine
  - Opacity fade during Fading state
  - cleanup_transmissions: despawns Complete entities
- `src/text/spawn.rs` - Spawning system
  - spawn_queued_transmissions: processes queue
  - Text2dBundle with position-based justify
  - update_transmission_queue: timer updates
- `src/text/api.rs` - Convenience API
  - TransmissionCommands trait: transmit(), transmit_at()
  - transmit_delayed(), transmit_full() for queue access
- TransmissionPlugin and TextPlugin for easy registration

### Notes
- Prompt 37-TEXT-TRANSMISSION complete
- Phase 8 (Narrative) begun
- 92.5% overall progress

## [0.7.3] - 2024-12-24

### Added
- `src/audio/bang_sound.rs` - Bang rumble effect
  - BangRumble: layered sub-bass + mid + noise
  - Sub-bass at 30Hz, mid at 60Hz
  - Pitch drops over duration (0.5x by end)
  - Filter sweep: opens to 500Hz then closes
  - 6-second duration with ADSR envelope
- `src/audio/grief_sound.rs` - Grief dissonance effect
  - GriefDissonance: minor second cluster (A3, Bb3, B3)
  - Three triangle oscillators with staggered envelopes
  - Tremolo modulation at 4Hz
  - Triggered when Child fades
- `src/audio/silence.rs` - Strategic silence management
  - SilenceState: Normal, FadingToSilence, Silent, FadingFromSilence
  - SilenceManager: volume multiplier with cubic easing
  - fade_to_silence(), fade_from_silence() controls
- `src/audio/transitions.rs` - Phase transition sounds
  - TransitionSound: phase-specific frequencies
  - Signal→D4, Bang→A3, Awakening→E4, Discovery→G4
  - Connection→A4, Acceptance→B4, Ended→D5
- `src/audio/events.rs` - Event sound system
  - EventSoundConfig: durations and frequencies
  - EventSounds resource: bang_rumble, grief, transitions
  - handle_bang_events: triggers rumble on Expansion stage
  - handle_traveler_faded: triggers grief for Child
  - handle_phase_transitions: triggers transition sounds
  - EventSoundPlugin registers all event handlers

### Notes
- Prompt 36-AUDIO-EVENTS complete
- **Phase 7 (Audio) COMPLETE**
- 90% overall progress

## [0.7.2] - 2024-12-24

### Added
- `src/audio/spatial.rs` - 3D spatial audio system
  - SpatialAudioConfig: max_distance, reference_distance, rolloff, Doppler settings
  - AudioListener component: tracks camera for listener position
  - SpatialAudioSource component: computed gain, pan, pitch
  - calculate_attenuation: inverse distance falloff
  - calculate_panning: stereo positioning from 3D direction
  - calculate_doppler: pitch shift for moving sources
  - update_spatial_audio: processes all sources relative to listener
  - attach_listener_to_camera: auto-attaches listener to ExperienceCamera
  - SpatialAudioPlugin registers config and systems
- `src/audio/reverb.rs` - Schroeder reverb
  - CombFilter: 8 parallel delay lines with feedback (0.84)
  - AllpassFilter: 4 series diffusion filters (0.5)
  - Reverb: combines filters for cosmic spaciousness
  - Sample-rate scaled delays for consistency
- `src/audio/ambiance.rs` - Cosmic background ambiance
  - CosmicAmbiance resource: layered drone generator
  - Rumble: 30Hz sine → lowpass (60Hz cutoff)
  - Shimmer: 800Hz sine → highpass (2kHz cutoff)
  - Noise: white noise → bandpass (400Hz center)
  - Combined for cosmic void texture

### Notes
- Prompt 35-AUDIO-SPATIAL complete
- 87.5% overall progress

## [0.7.1] - 2024-12-24

### Added
- `src/audio/scale.rs` - D pentatonic scale system
  - A4 constant (440Hz), midi_to_freq conversion
  - D_PENTATONIC intervals: D, E, G, A, B (0, 2, 5, 7, 10 semitones)
  - ScaleDegree enum: Root, Second, Third, Fifth, Sixth
  - Scale struct with octave-aware frequency lookup
- `src/audio/leitmotif.rs` - Traveler leitmotif definitions
  - Contour enum: Ascending, Descending, Arch, Valley, Static
  - RhythmPattern: durations and rest positions
  - Leitmotif struct: preferred degrees, contour, octave range, tempo
  - Per-traveler leitmotifs:
    - Archivist: slow arch, lower register, 60 BPM
    - Wanderer: ascending, wide range, varied rhythm, 75 BPM
    - Keeper: static ostinato, steady, 55 BPM
    - Child: valley contour, high/playful, 90 BPM
    - Other: descending, sparse, wide leaps, 45 BPM
- `src/audio/melody.rs` - Procedural melody generator
  - MelodyGenerator with seeded RNG (ChaCha8)
  - Contour-based note selection
  - Interval tendency and preferred degree biasing
  - Octave wrapping and clamping
- `src/audio/leitmotif_player.rs` - Playback system
  - LeitmotifPlayer resource: melody/timing per traveler
  - start(), stop(): control traveler melodies
  - update(): returns notes to play based on tempo/rhythm
  - update_leitmotifs system integrates with AudioEngine
  - LeitmotifPlugin registers player and update system

### Notes
- Prompt 34-AUDIO-LEITMOTIFS complete
- 85% overall progress

## [0.7.0] - 2024-12-24

### Added
- `src/audio/oscillator.rs` - Waveform oscillator generators
  - Waveform enum: Sine, Saw, Triangle, Square, Noise
  - Oscillator struct with phase accumulation
  - sample(): generates next sample at given sample rate
  - white_noise(): xorshift-based noise generation
  - set_frequency(): runtime frequency adjustment
- `src/audio/filter.rs` - Biquad filter implementation
  - FilterType enum: LowPass, HighPass, BandPass
  - BiquadFilter with real-time coefficient calculation
  - process(): IIR filter processing
  - set_cutoff(), set_resonance(): parameter modulation
- `src/audio/envelope.rs` - ADSR envelope generator
  - EnvelopeStage: Idle, Attack, Decay, Sustain, Release
  - Envelope with configurable ADSR times
  - trigger(), release(): note control
  - process(): returns current envelope level
  - is_active(): check if envelope is sounding
- `src/audio/voice.rs` - Combined synth voice
  - Voice: oscillator + filter + amplitude envelope + filter envelope
  - trigger(): start note with frequency
  - release(): begin release phase
  - process(): generates filtered, enveloped sample
  - Filter envelope modulates cutoff frequency
- `src/audio/engine.rs` - Polyphonic audio engine
  - AudioEngine resource with voice management
  - play_note(): voice stealing and allocation
  - release_note(): release by frequency
  - fill_buffer(): generate audio buffer
  - 16 voice polyphony, 44100 Hz sample rate
- AudioSynthesisPlugin initializes AudioEngine resource

### Notes
- Prompt 33-AUDIO-SYNTHESIS complete
- Phase 7 (Audio) begun
- 82.5% overall progress

## [0.6.4] - 2024-12-24

### Added
- `src/bang/debris.rs` - 5000 debris particles system
  - DebrisConfig: 5000 particles, velocity range, decay, lifetimes
  - DebrisParticle component: velocity, age, lifetime, is_seed
  - DebrisState: spawn tracking
  - spawn_debris: burst at peak with spherical distribution
  - update_debris: velocity decay, temperature cooling, size shrink
  - debris_temperature_color: white -> amber -> red -> dark gradient
  - 5 seed particles persist longer with pulsing effect
  - transform_seeds_to_travelers: converts seeds to TravelerSpawnMarker
  - reset_debris_state: cleanup for restart
  - DebrisPlugin registers all systems

### Notes
- Prompt 32-BANG-DEBRIS complete
- Phase 6 (The Bang) complete!
- 80% overall progress

## [0.6.3] - 2024-12-24

### Added
- `src/bang/shockwave.rs` - Expanding torus shockwave system
  - Shockwave component: radius, thickness, refraction, lifetime
  - ShockwaveConfig: spawn_time (4s at peak), speed, max_radius
  - ShockwaveState: tracks if spawned
  - create_shockwave_mesh: procedural torus (64x16 segments)
  - spawn_shockwave: creates at bang peak with refractive material
  - update_shockwave: expanding radius, thinning, camera pass detection
  - Increased emissive when passing through camera position
  - reset_shockwave_state: cleanup for restart
  - ShockwavePlugin registers all systems

### Notes
- Prompt 31-BANG-SHOCKWAVE complete
- 77.5% overall progress

## [0.6.2] - 2024-12-24

### Added
- `assets/shaders/god_rays.wgsl` - Screen-space radial blur shader
  - Raymarch from pixel toward light source
  - Accumulate samples with decay factor
  - Distance-based falloff from center
- `src/bang/god_rays.rs` - God rays system
  - GodRayConfig: decay, density, samples, exposure settings
  - GodRayState: light_position, intensity tracking
  - update_god_ray_intensity: follows bang timeline
  - interpolate_god_rays: smooth intensity transitions
  - update_light_screen_position: world-to-screen projection
  - GodRaysPlugin (render integration deferred to post-processing)

### Notes
- Prompt 30-BANG-GOD-RAYS complete
- 75% overall progress

## [0.6.1] - 2024-12-24

### Added
- `src/bang/expansion.rs` - Expansion rings system
  - ExpansionRing component: spawn_time, expansion, index
  - ExpansionConfig: 5 rings, 0.2s interval, 3s duration, 80m radius
  - RingSpawnState tracker for spawning coordination
  - create_ring_mesh: procedural ring geometry (64 segments)
  - spawn_expansion_rings: creates rings during expansion phase
  - update_expansion_rings: eased expansion, cooling color, fading opacity
  - reset_ring_state: cleans up after bang completes
  - ExpansionPlugin registers all systems

### Notes
- Prompt 29-BANG-EXPANSION complete
- 72.5% overall progress

## [0.6.0] - 2024-12-24

### Added
- `src/bang/core.rs` - Bang core light explosion effect
  - BangCore component: intensity, expansion, temperature
  - BangConfig: precise timeline (start 2s, peak 4s, complete 10s)
  - spawn_bang_core: ico sphere with emissive/additive material
  - update_bang_core: phase-based animations
    - Point of light (2.0-2.5s)
    - Building intensity (2.5-3.0s)
    - Explosive expansion (3.0-4.0s) with ease_out_expo
    - Deceleration/cooling (4.0-6.0s)
    - Settling to void (6.0-10.0s)
  - temperature_to_color: white -> amber -> red gradient
  - BangCorePlugin registers all systems

### Notes
- Prompt 28-BANG-CORE complete
- Phase 6 (The Bang) begun
- 70% overall progress

## [0.5.4] - 2024-12-24

### Added
- `src/environment/reflection.rs` - Reflection plane system
  - ReflectionConfig: height, size, max_opacity, blur, falloff
  - ReflectionState for smooth opacity interpolation
  - ReflectionPlane marker component
  - spawn_reflection_plane with metallic/glass-like material
  - update_reflection_for_phase: fade in during Connection, out during Acceptance
  - interpolate_reflection: lerps opacity, updates material/visibility
  - ReflectionPlugin registers all systems

### Notes
- Prompt 27-REFLECTION-PLANE complete
- Phase 5 (Environment) complete!
- 67.5% overall progress

## [0.5.3] - 2024-12-24

### Added
- `src/environment/fog.rs` - Volumetric fog system
  - FogConfig with phase-specific colors
  - PhaseColors: warm bang, cool acceptance, amber connection
  - FogState for smooth color/density interpolation
  - setup_fog adds FogSettings to ExperienceCamera
  - update_fog_for_phase: targets based on clock phase
  - interpolate_fog: lerps color and density smoothly
  - FogPulseEvent for temporary fog overrides
  - FogPlugin registers all systems

### Notes
- Prompt 26-VOLUMETRIC-FOG complete
- 65% overall progress

## [0.5.2] - 2024-12-24

### Added
- `src/environment/dust.rs` - 10,000 dust particle system
  - DustParticle with depth, opacity, velocity, noise seed
  - DustConfig: 10k particles, 20-150m depth range
  - Spherical distribution with vertical flattening
  - Opacity decreases with distance for depth perception
  - animate_dust: drift + noise-based motion with position wrapping
  - update_dust_visibility: phase-driven fade
  - DustLayer component for 4-tier depth organization
  - DustAssets caches shared mesh/material
  - DustPlugin registers all systems

### Notes
- Prompt 25-NEBULA-DUST complete
- 62.5% overall progress

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

[Unreleased]: https://github.com/watchthelight/lightwatch2/compare/v0.9.0...HEAD
[0.9.0]: https://github.com/watchthelight/lightwatch2/compare/v0.8.1...v0.9.0
[0.8.1]: https://github.com/watchthelight/lightwatch2/compare/v0.8.0...v0.8.1
[0.8.0]: https://github.com/watchthelight/lightwatch2/compare/v0.7.3...v0.8.0
[0.7.3]: https://github.com/watchthelight/lightwatch2/compare/v0.7.2...v0.7.3
[0.7.2]: https://github.com/watchthelight/lightwatch2/compare/v0.7.1...v0.7.2
[0.7.1]: https://github.com/watchthelight/lightwatch2/compare/v0.7.0...v0.7.1
[0.7.0]: https://github.com/watchthelight/lightwatch2/compare/v0.6.4...v0.7.0
[0.6.4]: https://github.com/watchthelight/lightwatch2/compare/v0.6.3...v0.6.4
[0.6.3]: https://github.com/watchthelight/lightwatch2/compare/v0.6.2...v0.6.3
[0.6.2]: https://github.com/watchthelight/lightwatch2/compare/v0.6.1...v0.6.2
[0.6.1]: https://github.com/watchthelight/lightwatch2/compare/v0.6.0...v0.6.1
[0.6.0]: https://github.com/watchthelight/lightwatch2/compare/v0.5.4...v0.6.0
[0.5.4]: https://github.com/watchthelight/lightwatch2/compare/v0.5.3...v0.5.4
[0.5.3]: https://github.com/watchthelight/lightwatch2/compare/v0.5.2...v0.5.3
[0.5.2]: https://github.com/watchthelight/lightwatch2/compare/v0.5.1...v0.5.2
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
