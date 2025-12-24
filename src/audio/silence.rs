//! Silence manager - strategic use of silence

use bevy::prelude::*;

/// Silence state
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum SilenceState {
    /// Normal audio
    #[default]
    Normal,
    /// Fading to silence
    FadingToSilence,
    /// Complete silence
    Silent,
    /// Fading from silence
    FadingFromSilence,
}

/// Silence manager - strategic use of silence
#[derive(Resource)]
pub struct SilenceManager {
    pub state: SilenceState,
    /// Current volume multiplier
    pub volume_multiplier: f32,
    /// Target volume
    target_volume: f32,
    /// Fade duration
    fade_duration: f32,
    /// Time in current state
    time_in_state: f32,
}

impl Default for SilenceManager {
    fn default() -> Self {
        Self {
            state: SilenceState::Normal,
            volume_multiplier: 1.0,
            target_volume: 1.0,
            fade_duration: 2.0,
            time_in_state: 0.0,
        }
    }
}

impl SilenceManager {
    /// Fade to silence
    #[allow(dead_code)]
    pub fn fade_to_silence(&mut self, duration: f32) {
        self.state = SilenceState::FadingToSilence;
        self.target_volume = 0.0;
        self.fade_duration = duration;
        self.time_in_state = 0.0;
    }

    /// Fade from silence
    #[allow(dead_code)]
    pub fn fade_from_silence(&mut self, duration: f32) {
        self.state = SilenceState::FadingFromSilence;
        self.target_volume = 1.0;
        self.fade_duration = duration;
        self.time_in_state = 0.0;
    }

    /// Update silence state
    pub fn update(&mut self, delta_time: f32) {
        self.time_in_state += delta_time;

        match self.state {
            SilenceState::Normal => {
                self.volume_multiplier = 1.0;
            }
            SilenceState::FadingToSilence => {
                let progress = (self.time_in_state / self.fade_duration).min(1.0);
                self.volume_multiplier = 1.0 - ease_in_out_cubic(progress);

                if progress >= 1.0 {
                    self.state = SilenceState::Silent;
                    self.time_in_state = 0.0;
                }
            }
            SilenceState::Silent => {
                self.volume_multiplier = 0.0;
            }
            SilenceState::FadingFromSilence => {
                let progress = (self.time_in_state / self.fade_duration).min(1.0);
                self.volume_multiplier = ease_in_out_cubic(progress);

                if progress >= 1.0 {
                    self.state = SilenceState::Normal;
                    self.time_in_state = 0.0;
                }
            }
        }
    }
}

fn ease_in_out_cubic(t: f32) -> f32 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powf(3.0) / 2.0
    }
}
