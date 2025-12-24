//! Leitmotif playback system

use bevy::prelude::*;
use std::collections::HashMap;

use super::leitmotif::Leitmotif;
use super::melody::{Melody, MelodyGenerator};
use super::AudioEngine;
use crate::core::TravelerId;

/// Leitmotif playback state
#[derive(Resource)]
pub struct LeitmotifPlayer {
    /// Current melodies per traveler
    melodies: HashMap<TravelerId, Melody>,
    /// Current note index per traveler
    note_indices: HashMap<TravelerId, usize>,
    /// Time until next note per traveler
    time_until_next: HashMap<TravelerId, f32>,
    /// Generator
    generator: MelodyGenerator,
}

impl Default for LeitmotifPlayer {
    fn default() -> Self {
        Self {
            melodies: HashMap::new(),
            note_indices: HashMap::new(),
            time_until_next: HashMap::new(),
            generator: MelodyGenerator::new(42),
        }
    }
}

impl LeitmotifPlayer {
    /// Start playing leitmotif for traveler
    #[allow(dead_code)]
    pub fn start(&mut self, traveler: TravelerId) {
        let leitmotif = match traveler {
            TravelerId::Archivist => Leitmotif::archivist(),
            TravelerId::Wanderer => Leitmotif::wanderer(),
            TravelerId::Keeper => Leitmotif::keeper(),
            TravelerId::Child => Leitmotif::child(),
            TravelerId::Other => Leitmotif::other(),
        };

        let melody = self.generator.generate(&leitmotif);
        self.melodies.insert(traveler, melody);
        self.note_indices.insert(traveler, 0);
        self.time_until_next.insert(traveler, 0.0);

        info!(target: "lightwatch::audio", "Leitmotif started for {:?}", traveler);
    }

    /// Stop playing leitmotif for traveler
    #[allow(dead_code)]
    pub fn stop(&mut self, traveler: TravelerId) {
        self.melodies.remove(&traveler);
        self.note_indices.remove(&traveler);
        self.time_until_next.remove(&traveler);
    }

    /// Update and get notes to play
    pub fn update(&mut self, delta: f32) -> Vec<(TravelerId, f32)> {
        let mut notes_to_play = Vec::new();

        for (traveler, melody) in &self.melodies {
            let time = self.time_until_next.get_mut(traveler).unwrap();
            *time -= delta;

            if *time <= 0.0 {
                let idx = self.note_indices.get_mut(traveler).unwrap();

                if *idx < melody.notes.len() {
                    if !melody.is_rest[*idx] {
                        notes_to_play.push((*traveler, melody.notes[*idx]));
                    }

                    // Get tempo for this traveler
                    let tempo = match traveler {
                        TravelerId::Archivist => 60.0,
                        TravelerId::Wanderer => 75.0,
                        TravelerId::Keeper => 55.0,
                        TravelerId::Child => 90.0,
                        TravelerId::Other => 45.0,
                    };

                    let beat_duration = 60.0 / tempo;
                    *time = melody.durations[*idx] * beat_duration;
                    *idx += 1;
                }
            }
        }

        notes_to_play
    }

    /// Check if traveler's melody is complete
    #[allow(dead_code)]
    pub fn is_complete(&self, traveler: TravelerId) -> bool {
        if let (Some(melody), Some(idx)) =
            (self.melodies.get(&traveler), self.note_indices.get(&traveler))
        {
            *idx >= melody.notes.len()
        } else {
            true
        }
    }
}

/// Update leitmotif playback
pub fn update_leitmotifs(
    time: Res<Time>,
    mut player: ResMut<LeitmotifPlayer>,
    mut engine: ResMut<AudioEngine>,
) {
    let notes = player.update(time.delta_seconds());

    for (_traveler, frequency) in notes {
        engine.play_note(frequency);
    }
}

/// Leitmotif plugin
pub struct LeitmotifPlugin;

impl Plugin for LeitmotifPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LeitmotifPlayer>()
            .add_systems(Update, update_leitmotifs);
    }
}
