//! Traveler identity - who they are and what defines them

#![allow(dead_code)]

use bevy::prelude::*;

use crate::core::TravelerId;

/// Core identity component for a traveler
#[derive(Component, Debug)]
pub struct Traveler {
    /// Which traveler is this?
    pub id: TravelerId,
    /// Display name
    pub name: &'static str,
    /// Time when this traveler spawned
    pub spawn_time: f32,
    /// Time when this traveler should fade (if known)
    pub fade_time: Option<f32>,
}

impl Traveler {
    pub fn new(id: TravelerId, spawn_time: f32) -> Self {
        let (name, fade_time) = match id {
            TravelerId::Archivist => ("THE ARCHIVIST", Some(120.0)),
            TravelerId::Wanderer => ("THE WANDERER", Some(105.0)),
            TravelerId::Keeper => ("THE KEEPER", Some(112.0)),
            TravelerId::Child => ("THE CHILD", Some(95.0)), // First to die
            TravelerId::Other => ("THE OTHER", Some(130.0)), // Last to die
        };

        Self {
            id,
            name,
            spawn_time,
            fade_time,
        }
    }
}

/// Static traveler definitions
pub struct TravelerDef {
    pub id: TravelerId,
    pub geometry: TravelerGeometry,
    pub color: TravelerColor,
    pub rhythm: TravelerRhythm,
    pub spawn_position: Vec3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TravelerGeometry {
    Icosahedron,  // Archivist - 20 faces, complex
    Tetrahedron,  // Wanderer - 4 faces, simple/sharp
    Cube,         // Keeper - 6 faces, stable
    Octahedron,   // Child - 8 faces, delicate
    Dodecahedron, // Other - 12 faces, alien
}

#[derive(Debug, Clone, Copy)]
pub struct TravelerColor {
    pub base: Color,
    pub evolved: Color,
    pub final_state: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct TravelerRhythm {
    pub base_hz: f32,
    pub variance: f32,
}

impl TravelerDef {
    pub fn all() -> Vec<Self> {
        vec![
            Self::archivist(),
            Self::wanderer(),
            Self::keeper(),
            Self::child(),
            Self::other(),
        ]
    }

    pub fn archivist() -> Self {
        Self {
            id: TravelerId::Archivist,
            geometry: TravelerGeometry::Icosahedron,
            color: TravelerColor {
                base: Color::srgb(0.91, 0.64, 0.27), // Amber
                evolved: Color::srgb(0.42, 0.36, 0.58), // Violet
                final_state: Color::srgb(0.91, 0.64, 0.27),
            },
            rhythm: TravelerRhythm {
                base_hz: 0.14,
                variance: 0.02,
            },
            spawn_position: Vec3::new(-2.0, 0.0, -5.0),
        }
    }

    pub fn wanderer() -> Self {
        Self {
            id: TravelerId::Wanderer,
            geometry: TravelerGeometry::Tetrahedron,
            color: TravelerColor {
                base: Color::srgb(0.31, 0.80, 0.77), // Cyan
                evolved: Color::srgb(0.31, 0.80, 0.77),
                final_state: Color::srgb(0.31, 0.80, 0.77),
            },
            rhythm: TravelerRhythm {
                base_hz: 0.11,
                variance: 0.03,
            },
            spawn_position: Vec3::new(3.0, 1.0, -6.0),
        }
    }

    pub fn keeper() -> Self {
        Self {
            id: TravelerId::Keeper,
            geometry: TravelerGeometry::Cube,
            color: TravelerColor {
                base: Color::srgb(0.83, 0.46, 0.18), // Orange
                evolved: Color::srgb(0.83, 0.46, 0.18),
                final_state: Color::srgb(0.83, 0.46, 0.18),
            },
            rhythm: TravelerRhythm {
                base_hz: 0.08,
                variance: 0.01,
            },
            spawn_position: Vec3::new(-1.0, -1.5, -4.0),
        }
    }

    pub fn child() -> Self {
        Self {
            id: TravelerId::Child,
            geometry: TravelerGeometry::Octahedron,
            color: TravelerColor {
                base: Color::srgb(0.96, 0.94, 0.91), // White
                evolved: Color::srgb(0.96, 0.94, 0.91),
                final_state: Color::srgb(0.96, 0.94, 0.91),
            },
            rhythm: TravelerRhythm {
                base_hz: 0.18,
                variance: 0.04,
            },
            spawn_position: Vec3::new(1.5, 0.5, -3.0),
        }
    }

    pub fn other() -> Self {
        Self {
            id: TravelerId::Other,
            geometry: TravelerGeometry::Dodecahedron,
            color: TravelerColor {
                base: Color::srgb(0.42, 0.36, 0.58), // Violet
                evolved: Color::srgb(0.42, 0.36, 0.58),
                final_state: Color::srgb(0.42, 0.36, 0.58),
            },
            rhythm: TravelerRhythm {
                base_hz: 0.06,
                variance: 0.01,
            },
            spawn_position: Vec3::new(0.0, 2.0, -10.0), // Distant
        }
    }

    pub fn get(id: TravelerId) -> Self {
        match id {
            TravelerId::Archivist => Self::archivist(),
            TravelerId::Wanderer => Self::wanderer(),
            TravelerId::Keeper => Self::keeper(),
            TravelerId::Child => Self::child(),
            TravelerId::Other => Self::other(),
        }
    }
}
