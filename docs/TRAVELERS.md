# LIGHTWATCH: The Five Travelers

## Overview

The travelers are ancient beings who found each other across cosmic distances. Each is unique in form, color, frequency, and purpose. They are not characters in a traditional sense—they are presences, lights in the void.

---

## The Travelers

### Archivist

**Role**: The Organizer, The Recorder

| Attribute | Value |
|-----------|-------|
| Geometry | Icosahedron (20 faces) |
| Color | Amber (#FFB347) |
| Frequency | 0.14 Hz |
| Death Order | 4th (130-138s) |

**Character**:
The Archivist remembers. Every moment, every connection, every loss—recorded in light. They organized the others, gave structure to chaos, made meaning from randomness. They die when there is nothing left to record.

**Visual Signature**:
- Complex geometry reflects their analytical nature
- Amber glow pulses in mathematical patterns
- Particles orbit in organized layers
- Trail forms geometric patterns

**Audio Signature**:
- Clear, bell-like tones
- Major key leitmotif
- Precise, measured rhythm
- Voice fades last among the original four

---

### Wanderer

**Role**: The Searcher, The Restless

| Attribute | Value |
|-----------|-------|
| Geometry | Tetrahedron (4 faces) |
| Color | Cyan (#00CED1) |
| Frequency | 0.11 Hz |
| Death Order | 3rd (118-128s) |

**Character**:
The Wanderer never stopped moving. Always seeking, always hoping for something beyond the next horizon. They searched the cosmos for meaning and found it in the others. They die when they finally stop searching.

**Visual Signature**:
- Sharp, angular geometry
- Cyan trails stretch behind them
- Erratic, searching movement patterns
- Particles stream like a comet tail

**Audio Signature**:
- Wavering, searching tones
- Minor key with suspended notes
- Irregular rhythm, never quite settling
- Voice carries longing

---

### Keeper

**Role**: The Listener, The Nurturer

| Attribute | Value |
|-----------|-------|
| Geometry | Cube (6 faces) |
| Color | Orange (#FF6B35) |
| Frequency | 0.08 Hz |
| Death Order | 2nd (105-115s) |

**Character**:
The Keeper held space for others. They listened when others spoke, gave warmth when others grew cold. Stable, grounding, always present. They die because they gave everything to the others, leaving nothing for themselves.

**Visual Signature**:
- Stable, grounded geometry
- Warm orange glow, steady
- Slow, deliberate movement
- Particles gather close, protective

**Audio Signature**:
- Deep, resonant tones
- Drone-like foundation
- Slowest rhythm of all
- Voice is the foundation others harmonize against

---

### Child

**Role**: The Youngest, The Brightest

| Attribute | Value |
|-----------|-------|
| Geometry | Octahedron (8 faces) |
| Color | White (#FFFFFF → warm white) |
| Frequency | 0.18 Hz |
| Death Order | 1st (92-102s) |

**Character**:
The Child burned brightest. Newest to existence, full of wonder and energy. They experienced everything with intensity—joy, curiosity, connection. They die first because the brightest lights burn fastest.

**Visual Signature**:
- Pure, balanced geometry
- Brilliant white that shifts warm
- Quick, excited movement
- Particles scatter energetically
- Highest luminosity of all travelers

**Audio Signature**:
- High, clear tones
- Simple, pure melody
- Fastest rhythm
- Voice like a chime, innocent

---

### Other

**Role**: The Alien, The Unknown

| Attribute | Value |
|-----------|-------|
| Geometry | Dodecahedron (12 faces) |
| Color | Violet (#8B5CF6) |
| Frequency | 0.06 Hz |
| Death Order | 5th (last, 138-143s) |

**Character**:
The Other is unknowable. They joined the group but remained apart—not by choice, but by nature. They watched, participated in their own way, but could never fully connect. They die last because they were never fully part of what ended.

**Visual Signature**:
- Complex, otherworldly geometry
- Deep violet, almost ultraviolet
- Slowest movement, most deliberate
- Particles orbit in strange, non-Euclidean patterns
- Slight dimensional shimmer

**Audio Signature**:
- Unusual harmonics, slightly dissonant
- Frequencies that don't quite fit
- Slowest rhythm, almost outside time
- Voice is more felt than heard
- Remains silent during Connection peak

---

## Relationships

```
                    Archivist
                   (Organizer)
                       │
           ┌──────────┼──────────┐
           │          │          │
        Wanderer    Child      Keeper
       (Searcher) (Brightest) (Listener)
           │          │          │
           └──────────┼──────────┘
                      │
                   Other ─ ─ ─ (observes from outside)
                 (Unknown)
```

**Connection Dynamics**:
- Archivist organizes the group, creates structure
- Keeper provides foundation for others to exist
- Child brings energy that draws others together
- Wanderer searched and found them all
- Other watches, participates, but remains fundamentally separate

---

## Code Reference

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TravelerId {
    Archivist,
    Wanderer,
    Keeper,
    Child,
    Other,
}

impl TravelerId {
    pub fn color(&self) -> Color {
        match self {
            Self::Archivist => Color::rgb(1.0, 0.702, 0.278),  // Amber
            Self::Wanderer => Color::rgb(0.0, 0.808, 0.820),   // Cyan
            Self::Keeper => Color::rgb(1.0, 0.420, 0.208),     // Orange
            Self::Child => Color::rgb(1.0, 1.0, 1.0),          // White
            Self::Other => Color::rgb(0.545, 0.361, 0.965),    // Violet
        }
    }

    pub fn frequency(&self) -> f32 {
        match self {
            Self::Archivist => 0.14,
            Self::Wanderer => 0.11,
            Self::Keeper => 0.08,
            Self::Child => 0.18,
            Self::Other => 0.06,
        }
    }

    pub fn death_time(&self) -> (f32, f32) {
        match self {
            Self::Child => (92.0, 102.0),
            Self::Keeper => (105.0, 115.0),
            Self::Wanderer => (118.0, 128.0),
            Self::Archivist => (130.0, 138.0),
            Self::Other => (138.0, 143.0),
        }
    }

    pub fn geometry_faces(&self) -> u32 {
        match self {
            Self::Archivist => 20,  // Icosahedron
            Self::Wanderer => 4,    // Tetrahedron
            Self::Keeper => 6,      // Cube
            Self::Child => 8,       // Octahedron
            Self::Other => 12,      // Dodecahedron
        }
    }
}
```

---

## Visual Reference

| Traveler | Shape | Color Hex | Hz | Dies |
|----------|-------|-----------|-----|------|
| Archivist | Icosahedron | #FFB347 | 0.14 | 4th |
| Wanderer | Tetrahedron | #00CED1 | 0.11 | 3rd |
| Keeper | Cube | #FF6B35 | 0.08 | 2nd |
| Child | Octahedron | #FFFFFF | 0.18 | 1st |
| Other | Dodecahedron | #8B5CF6 | 0.06 | 5th |
