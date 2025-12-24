//! Traveler text fragments and display names

use crate::core::TravelerId;

/// Fragment attributed to a traveler
pub struct TravelerFragment {
    pub traveler: TravelerId,
    pub text: &'static str,
    pub phase_start: f32, // Can appear after this time
    pub phase_end: f32,   // Cannot appear after this time
}

/// All traveler fragments
pub fn get_traveler_fragments() -> Vec<TravelerFragment> {
    vec![
        // Archivist - memory keeper, deliberate
        TravelerFragment {
            traveler: TravelerId::Archivist,
            text: "We were here before the counting began",
            phase_start: 27.0,
            phase_end: 57.0,
        },
        TravelerFragment {
            traveler: TravelerId::Archivist,
            text: "Memory persists where light cannot",
            phase_start: 57.0,
            phase_end: 87.0,
        },
        TravelerFragment {
            traveler: TravelerId::Archivist,
            text: "Each ending contains its beginning",
            phase_start: 87.0,
            phase_end: 143.0,
        },
        // Wanderer - explorer, restless
        TravelerFragment {
            traveler: TravelerId::Wanderer,
            text: "Always further",
            phase_start: 27.0,
            phase_end: 57.0,
        },
        TravelerFragment {
            traveler: TravelerId::Wanderer,
            text: "The edge calls louder than the center",
            phase_start: 57.0,
            phase_end: 87.0,
        },
        TravelerFragment {
            traveler: TravelerId::Wanderer,
            text: "There is no arrival, only approach",
            phase_start: 87.0,
            phase_end: 143.0,
        },
        // Keeper - guardian, steady
        TravelerFragment {
            traveler: TravelerId::Keeper,
            text: "Hold what cannot be held",
            phase_start: 27.0,
            phase_end: 57.0,
        },
        TravelerFragment {
            traveler: TravelerId::Keeper,
            text: "Stillness is not absence",
            phase_start: 57.0,
            phase_end: 87.0,
        },
        TravelerFragment {
            traveler: TravelerId::Keeper,
            text: "We remain because we must",
            phase_start: 87.0,
            phase_end: 143.0,
        },
        // Child - first to fade, innocent
        TravelerFragment {
            traveler: TravelerId::Child,
            text: "Is this the first time or the last",
            phase_start: 27.0,
            phase_end: 57.0,
        },
        TravelerFragment {
            traveler: TravelerId::Child,
            text: "The dark is warm here",
            phase_start: 57.0,
            phase_end: 87.0,
        },
        TravelerFragment {
            traveler: TravelerId::Child,
            text: "I forget which way we came",
            phase_start: 87.0,
            phase_end: 110.0, // Before fading
        },
        // Other - distant, unknowable
        TravelerFragment {
            traveler: TravelerId::Other,
            text: "...",
            phase_start: 27.0,
            phase_end: 57.0,
        },
        TravelerFragment {
            traveler: TravelerId::Other,
            text: "The pattern recognizes itself",
            phase_start: 57.0,
            phase_end: 87.0,
        },
        TravelerFragment {
            traveler: TravelerId::Other,
            text: "We were never what you imagined",
            phase_start: 120.0,
            phase_end: 143.0,
        },
    ]
}

/// Get traveler display name
pub fn traveler_display_name(id: TravelerId) -> &'static str {
    match id {
        TravelerId::Archivist => "THE ARCHIVIST",
        TravelerId::Wanderer => "THE WANDERER",
        TravelerId::Keeper => "THE KEEPER",
        TravelerId::Child => "THE CHILD",
        TravelerId::Other => "THE OTHER",
    }
}
