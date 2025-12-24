//! Text display configuration

#![allow(dead_code)]

use bevy::prelude::*;

/// Text display configuration
#[derive(Resource)]
pub struct TextConfig {
    /// Characters per second for typewriter
    pub chars_per_second: f32,
    /// Hold duration after complete
    pub hold_duration: f32,
    /// Fade out duration
    pub fade_duration: f32,
    /// Default font size
    pub font_size: f32,
    /// Text color
    pub text_color: Color,
    /// Glow color
    #[allow(dead_code)]
    pub glow_color: Color,
}

impl Default for TextConfig {
    fn default() -> Self {
        Self {
            chars_per_second: 12.0,
            hold_duration: 3.0,
            fade_duration: 1.5,
            font_size: 24.0,
            text_color: Color::srgba(0.7, 0.75, 0.8, 1.0),
            glow_color: Color::srgba(0.5, 0.6, 0.7, 0.3),
        }
    }
}

/// Screen position for text
#[derive(Clone, Copy, Debug, Default)]
pub enum TextPosition {
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    #[default]
    BottomCenter,
    BottomRight,
}

impl TextPosition {
    pub fn to_justify(&self) -> JustifyText {
        match self {
            TextPosition::TopLeft | TextPosition::CenterLeft | TextPosition::BottomLeft => {
                JustifyText::Left
            }
            TextPosition::TopCenter | TextPosition::Center | TextPosition::BottomCenter => {
                JustifyText::Center
            }
            TextPosition::TopRight | TextPosition::CenterRight | TextPosition::BottomRight => {
                JustifyText::Right
            }
        }
    }

    /// Get screen offset percentages
    pub fn to_offset(&self) -> (f32, f32) {
        match self {
            TextPosition::TopLeft => (-40.0, 40.0),
            TextPosition::TopCenter => (0.0, 40.0),
            TextPosition::TopRight => (40.0, 40.0),
            TextPosition::CenterLeft => (-40.0, 0.0),
            TextPosition::Center => (0.0, 0.0),
            TextPosition::CenterRight => (40.0, 0.0),
            TextPosition::BottomLeft => (-40.0, -40.0),
            TextPosition::BottomCenter => (0.0, -40.0),
            TextPosition::BottomRight => (40.0, -40.0),
        }
    }
}
