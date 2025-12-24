# LIGHTWATCH

A 143-second real-time art piece. A meditation on connection, loss, and the vastness of time.

## The Experience

Five travelers—ancient beings who found each other across cosmic distances—left beacons broadcasting their final transmissions. You receive this signal 14.3 billion years later.

You witness:
- **Connection** — The warmth of finding others in infinite darkness
- **Loss** — The gradual dimming of lights, one by one
- **Silence** — The peace that comes after

## Technology

| Component | Technology |
|-----------|------------|
| Language | Rust |
| Engine | Bevy 0.14 |
| Rendering | PBR + Custom WGSL Shaders |
| Audio | bevy_kira_audio + Synthesis |
| Output | Native Binary |

## Building

```bash
cargo build --release
./target/release/lightwatch
```

## Documentation

| Document | Description |
|----------|-------------|
| [Vision](docs/VISION.md) | Project philosophy and goals |
| [Tech Stack](docs/TECH_STACK.md) | Technology decisions |
| [Timeline](docs/TIMELINE.md) | The 143-second journey |
| [Travelers](docs/TRAVELERS.md) | The five beings |
| [Architecture](docs/ARCHITECTURE.md) | ECS structure |
| [Roadmap](docs/ROADMAP.md) | Development progress |

## Timeline

```
0s        2s        12s       27s       57s       87s        143s
├─────────┼─────────┼─────────┼─────────┼──────────┼──────────┤
│ Signal  │  Bang   │Awakening│Discovery│Connection│Acceptance│
```

## The Travelers

| Name | Shape | Color | Frequency |
|------|-------|-------|-----------|
| Archivist | Icosahedron | Amber | 0.14 Hz |
| Wanderer | Tetrahedron | Cyan | 0.11 Hz |
| Keeper | Cube | Orange | 0.08 Hz |
| Child | Octahedron | White | 0.18 Hz |
| Other | Dodecahedron | Violet | 0.06 Hz |

## License

All rights reserved.

---

*"They found each other across infinite distances. For a moment, they were not alone."*
