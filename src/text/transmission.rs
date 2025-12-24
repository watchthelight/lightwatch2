//! Transmission text component

use bevy::prelude::*;

use super::TextPosition;

/// Transmission display state
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum TransmissionState {
    /// Characters appearing
    #[default]
    Typing,
    /// Fully displayed, holding
    Holding,
    /// Fading out
    Fading,
    /// Complete, ready for removal
    Complete,
}

/// Transmission text component
#[derive(Component)]
pub struct Transmission {
    /// Full text content
    pub full_text: String,
    /// Currently revealed character count
    pub revealed_chars: usize,
    /// Current state
    pub state: TransmissionState,
    /// Time in current state
    pub state_time: f32,
    /// Characters per second (can override config)
    pub chars_per_second: f32,
    /// Hold duration (can override config)
    pub hold_duration: f32,
    /// Fade duration (can override config)
    pub fade_duration: f32,
    /// Current opacity
    pub opacity: f32,
    /// Position on screen
    pub position: TextPosition,
    /// Priority (higher = displayed first)
    pub priority: i32,
}

impl Transmission {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            full_text: text.into(),
            revealed_chars: 0,
            state: TransmissionState::Typing,
            state_time: 0.0,
            chars_per_second: 12.0,
            hold_duration: 3.0,
            fade_duration: 1.5,
            opacity: 1.0,
            position: TextPosition::BottomCenter,
            priority: 0,
        }
    }

    #[allow(dead_code)]
    pub fn with_position(mut self, position: TextPosition) -> Self {
        self.position = position;
        self
    }

    #[allow(dead_code)]
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

    #[allow(dead_code)]
    pub fn with_hold(mut self, duration: f32) -> Self {
        self.hold_duration = duration;
        self
    }

    #[allow(dead_code)]
    pub fn with_speed(mut self, chars_per_second: f32) -> Self {
        self.chars_per_second = chars_per_second;
        self
    }

    /// Get currently visible text
    pub fn visible_text(&self) -> &str {
        &self.full_text[..self.revealed_chars.min(self.full_text.len())]
    }

    /// Check if typing is complete
    pub fn typing_complete(&self) -> bool {
        self.revealed_chars >= self.full_text.len()
    }
}
