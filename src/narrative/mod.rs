//! Narrative systems: Text rendering, Transmission fragments

use bevy::prelude::*;

use crate::text;

/// Narrative plugin for text and transmissions
pub struct NarrativePlugin;

impl Plugin for NarrativePlugin {
    fn build(&self, app: &mut App) {
        // Add text transmission and fragment plugins
        app.add_plugins(text::TextPlugin);
    }
}
