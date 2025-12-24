//! Transmission queue for ordered display

use bevy::prelude::*;

use super::Transmission;

/// Queued transmission request
pub struct QueuedTransmission {
    #[allow(dead_code)]
    pub text: String,
    pub delay: f32,
    pub transmission: Transmission,
}

/// Transmission queue for ordered display
#[derive(Resource, Default)]
pub struct TransmissionQueue {
    queue: Vec<QueuedTransmission>,
    /// Time until next can start
    time_until_next: f32,
    /// Minimum gap between transmissions
    pub minimum_gap: f32,
}

impl TransmissionQueue {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            queue: Vec::new(),
            time_until_next: 0.0,
            minimum_gap: 0.5,
        }
    }

    /// Add transmission to queue
    pub fn enqueue(&mut self, transmission: Transmission, delay: f32) {
        self.queue.push(QueuedTransmission {
            text: transmission.full_text.clone(),
            delay,
            transmission,
        });

        // Sort by priority (higher first), then by order added
        self.queue
            .sort_by(|a, b| b.transmission.priority.cmp(&a.transmission.priority));
    }

    /// Get next transmission if ready
    pub fn next(&mut self) -> Option<Transmission> {
        if self.time_until_next > 0.0 {
            return None;
        }

        if let Some(queued) = self.queue.first() {
            if queued.delay <= 0.0 {
                let queued = self.queue.remove(0);
                self.time_until_next = self.minimum_gap;
                return Some(queued.transmission);
            }
        }

        None
    }

    /// Update queue timers
    pub fn update(&mut self, delta: f32) {
        self.time_until_next = (self.time_until_next - delta).max(0.0);

        for queued in &mut self.queue {
            queued.delay = (queued.delay - delta).max(0.0);
        }
    }

    /// Clear all queued transmissions
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.queue.clear();
    }

    /// Number of queued transmissions
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}
