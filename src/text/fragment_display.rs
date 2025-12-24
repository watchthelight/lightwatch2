//! Fragment display system

use bevy::prelude::*;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use super::fragments::{get_traveler_fragments, traveler_display_name, TravelerFragment};
use super::{TextPosition, Transmission, TransmissionCommands, TransmissionQueue};
use crate::core::ExperienceClock;

/// Fragment display state
#[derive(Resource)]
pub struct FragmentState {
    /// Fragments already shown
    shown_fragments: Vec<usize>,
    /// Time since last fragment
    time_since_last: f32,
    /// Minimum time between fragments
    #[allow(dead_code)]
    pub min_interval: f32,
    /// Maximum time between fragments
    #[allow(dead_code)]
    pub max_interval: f32,
    /// RNG
    rng: ChaCha8Rng,
    /// Next fragment time
    next_fragment_at: f32,
}

impl Default for FragmentState {
    fn default() -> Self {
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let next = rng.gen_range(8.0..15.0);
        Self {
            shown_fragments: Vec::new(),
            time_since_last: 0.0,
            min_interval: 8.0,
            max_interval: 20.0,
            rng,
            next_fragment_at: next,
        }
    }
}

/// Display traveler fragments at appropriate times
pub fn display_fragments(
    time: Res<Time>,
    clock: Res<ExperienceClock>,
    mut state: ResMut<FragmentState>,
    mut queue: ResMut<TransmissionQueue>,
) {
    let elapsed = clock.elapsed();
    state.time_since_last += time.delta_seconds();

    // Only during active phases (after bang, before end)
    if elapsed < 12.0 || elapsed > 135.0 {
        return;
    }

    // Check if time for next fragment
    if state.time_since_last < state.next_fragment_at {
        return;
    }

    // Get available fragments
    let all_fragments = get_traveler_fragments();
    let available: Vec<(usize, &TravelerFragment)> = all_fragments
        .iter()
        .enumerate()
        .filter(|(idx, frag)| {
            !state.shown_fragments.contains(idx)
                && elapsed >= frag.phase_start
                && elapsed <= frag.phase_end
        })
        .collect();

    if available.is_empty() {
        return;
    }

    // Choose random fragment
    let choice_idx = state.rng.gen_range(0..available.len());
    let (idx, fragment) = available[choice_idx];

    // Format with quotes
    let text = format!("\"{}\"", fragment.text);

    queue.transmit_full(
        Transmission::new(text)
            .with_position(TextPosition::Center)
            .with_speed(10.0)
            .with_hold(4.0),
        0.0,
    );

    // Show attribution after main text
    queue.transmit_full(
        Transmission::new(format!("— {}", traveler_display_name(fragment.traveler)))
            .with_position(TextPosition::Center)
            .with_speed(15.0)
            .with_hold(3.0)
            .with_priority(-1), // Show after main text
        2.0,                    // Delay
    );

    state.shown_fragments.push(idx);
    state.time_since_last = 0.0;
    state.next_fragment_at = state.rng.gen_range(8.0..20.0);

    info!(target: "lightwatch::text", "Fragment displayed: {:?} - {}", fragment.traveler, fragment.text);
}
