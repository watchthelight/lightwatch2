# LIGHTWATCH: Timeline & Phases

## Overview

LIGHTWATCH runs for exactly **143 seconds**. This duration is symbolic:
- 143 seconds ≈ 14.3 billion years (the age of our universe)
- Each second represents ~100 million years
- The entire history of existence, compressed into a moment

## Phase Structure

```
Time (seconds)
0         12        27        57        87        143
├─────────┼─────────┼─────────┼─────────┼─────────┤
│ Signal  │  Bang   │Awakening│Discovery│Connection│ Acceptance
│  2s     │  10s    │  15s    │  30s    │   30s   │    56s
└─────────┴─────────┴─────────┴─────────┴─────────┘
```

## Phase Details

### Phase 1: Signal (0-2s)

**Duration**: 2 seconds
**Emotion**: Detection, anticipation

The moment before everything. A faint signal detected from across the cosmos.

| Parameter | Start | End |
|-----------|-------|-----|
| Screen brightness | 0% | 5% |
| Audio | Silence | Faint hum |
| Camera | Static | Slight drift |

**Visual Elements**:
- Black screen with subtle noise
- Single point of light appearing
- Gradual lens flare buildup

---

### Phase 2: Bang (2-12s)

**Duration**: 10 seconds
**Emotion**: Overwhelming awe, birth

The origin. Everything explodes into existence.

| Parameter | Start | End |
|-----------|-------|-----|
| Brightness | 5% | 400% (HDR) |
| Camera shake | 0.0 | 1.0 (max) |
| Particle count | 0 | 5000 |
| God ray intensity | 0.0 | 1.0 |

**Visual Elements**:
- Central explosion (white → gold → colors)
- Expanding shockwave
- God rays piercing outward
- Debris particles flying
- Screen near whiteout at peak (t=4s)

**Audio Elements**:
- Massive sub-bass impact
- Rising harmonic overtones
- Crystalline scatter sounds

---

### Phase 3: Awakening (12-27s)

**Duration**: 15 seconds
**Emotion**: Quiet emergence, newness

The travelers emerge from the aftermath. Disoriented, separate, alone.

| Parameter | Start | End |
|-----------|-------|-----|
| Brightness | 400% | 100% |
| Traveler visibility | 0% | 100% |
| Traveler separation | max | medium |
| Ambient audio | Chaos | Calm |

**Visual Elements**:
- Bang effects fading
- Five lights becoming visible
- Each traveler pulsing at their unique frequency
- Nebula background emerging
- Starfield revealing

**Audio Elements**:
- Reverb tail of Bang
- Individual traveler tones fading in
- Dissonant, unsynced frequencies

---

### Phase 4: Discovery (27-57s)

**Duration**: 30 seconds
**Emotion**: Wonder, search, hope

The travelers sense each other. They begin to move, to search, to reach.

| Parameter | Start | End |
|-----------|-------|-----|
| Traveler movement | Slow drift | Purposeful |
| Camera | Wide | Following |
| Particle trails | Faint | Visible |
| Harmonic sync | 0% | 30% |

**Visual Elements**:
- Travelers drifting toward center
- Trail particles forming
- Auras beginning to pulse
- Near-misses and approaches

**Audio Elements**:
- Individual leitmotifs playing
- Occasional harmonic coincidences
- Building anticipation

---

### Phase 5: Connection (57-87s)

**Duration**: 30 seconds
**Emotion**: Warmth, unity, love

They found each other. For a brief cosmic moment, they dance together.

| Parameter | Start | End |
|-----------|-------|-----|
| Traveler proximity | Medium | Close |
| Harmonic sync | 30% | 100% |
| Warmth (color temp) | Neutral | Warm |
| Particle interaction | Separate | Intertwined |

**Visual Elements**:
- Travelers orbiting common center
- Particles flowing between them
- Colors blending
- Synchronized pulsing
- Peak beauty moment (t=72s)

**Audio Elements**:
- Full harmony achieved
- Pentatonic melody emerges
- Warm, resonant chords
- Spatial audio creates sphere of sound

---

### Phase 6: Acceptance (87-143s)

**Duration**: 56 seconds
**Emotion**: Loss, peace, entropy

One by one, the lights go out. This is not tragedy—it is the natural order.

| Sub-phase | Time | Event |
|-----------|------|-------|
| Child fades | 92-102s | First light dims |
| Keeper fades | 105-115s | Second light dims |
| Wanderer fades | 118-128s | Third light dims |
| Archivist fades | 130-138s | Fourth light dims |
| Other remains | 138-143s | Final light, alone |
| Silence | 143s | Complete darkness |

**Visual Elements**:
- Each traveler's light slowly dimming
- Particles scattering, dispersing
- Colors desaturating
- Stars appearing as travelers fade
- Final fade to black

**Audio Elements**:
- Harmony breaking down note by note
- Each voice silencing
- Reverb carrying away last sounds
- 5 seconds of pure silence at end

---

## Death Order & Reasoning

| Order | Traveler | Reason |
|-------|----------|--------|
| 1st | Child | Youngest, burned brightest |
| 2nd | Keeper | Gave everything to others |
| 3rd | Wanderer | Finally stopped searching |
| 4th | Archivist | Finished recording |
| 5th | Other | Alien, unknowable, watches alone |

---

## Timing Constants (for code)

```rust
pub const PHASE_SIGNAL_START: f32 = 0.0;
pub const PHASE_SIGNAL_END: f32 = 2.0;
pub const PHASE_BANG_START: f32 = 2.0;
pub const PHASE_BANG_END: f32 = 12.0;
pub const PHASE_AWAKENING_START: f32 = 12.0;
pub const PHASE_AWAKENING_END: f32 = 27.0;
pub const PHASE_DISCOVERY_START: f32 = 27.0;
pub const PHASE_DISCOVERY_END: f32 = 57.0;
pub const PHASE_CONNECTION_START: f32 = 57.0;
pub const PHASE_CONNECTION_END: f32 = 87.0;
pub const PHASE_ACCEPTANCE_START: f32 = 87.0;
pub const PHASE_ACCEPTANCE_END: f32 = 143.0;
pub const EXPERIENCE_DURATION: f32 = 143.0;
```
