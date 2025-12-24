//! Transmission API for easy text display

#![allow(dead_code)]

use bevy::prelude::*;

use super::{TextPosition, Transmission, TransmissionQueue};

/// Extension trait for easy transmission creation
pub trait TransmissionCommands {
    /// Display text as transmission
    fn transmit(&mut self, text: &str);

    /// Display text with position
    fn transmit_at(&mut self, text: &str, position: TextPosition);

    /// Display text with delay
    fn transmit_delayed(&mut self, text: &str, delay: f32);

    /// Display text with full options
    fn transmit_full(&mut self, transmission: Transmission, delay: f32);
}

impl TransmissionCommands for ResMut<'_, TransmissionQueue> {
    fn transmit(&mut self, text: &str) {
        self.enqueue(Transmission::new(text), 0.0);
    }

    fn transmit_at(&mut self, text: &str, position: TextPosition) {
        self.enqueue(Transmission::new(text).with_position(position), 0.0);
    }

    fn transmit_delayed(&mut self, text: &str, delay: f32) {
        self.enqueue(Transmission::new(text), delay);
    }

    fn transmit_full(&mut self, transmission: Transmission, delay: f32) {
        self.enqueue(transmission, delay);
    }
}
