//! Wide event logging system - structured context for every event

#![allow(dead_code)]

use bevy::prelude::*;
use std::collections::HashMap;
use std::time::Instant;

/// A structured wide event - carries all context in fields
#[derive(Debug, Clone)]
pub struct WideEvent {
    /// Event name (snake_case)
    pub name: String,
    /// Timestamp since experience start
    pub elapsed_secs: f32,
    /// Arbitrary context fields
    pub context: HashMap<String, WideValue>,
    /// When this event was created
    pub created_at: Instant,
}

/// Values that can be stored in wide event context
#[derive(Debug, Clone)]
pub enum WideValue {
    String(String),
    Float(f32),
    Int(i64),
    Bool(bool),
    Vec3(Vec3),
}

impl WideEvent {
    /// Create a new wide event
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            elapsed_secs: 0.0,
            context: HashMap::new(),
            created_at: Instant::now(),
        }
    }

    /// Add string context
    pub fn with_str(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.context
            .insert(key.into(), WideValue::String(value.into()));
        self
    }

    /// Add float context
    pub fn with_f32(mut self, key: impl Into<String>, value: f32) -> Self {
        self.context.insert(key.into(), WideValue::Float(value));
        self
    }

    /// Add int context
    pub fn with_i64(mut self, key: impl Into<String>, value: i64) -> Self {
        self.context.insert(key.into(), WideValue::Int(value));
        self
    }

    /// Add bool context
    pub fn with_bool(mut self, key: impl Into<String>, value: bool) -> Self {
        self.context.insert(key.into(), WideValue::Bool(value));
        self
    }

    /// Add Vec3 context
    pub fn with_vec3(mut self, key: impl Into<String>, value: Vec3) -> Self {
        self.context.insert(key.into(), WideValue::Vec3(value));
        self
    }

    /// Emit the event (log with structured format)
    pub fn emit(mut self, elapsed: f32) -> Self {
        self.elapsed_secs = elapsed;

        let context_str: String = self
            .context
            .iter()
            .map(|(k, v)| format!("{}={:?}", k, v))
            .collect::<Vec<_>>()
            .join(" ");

        debug!(
            target: "lightwatch::events",
            "[{:.2}s] {} {}",
            self.elapsed_secs,
            self.name,
            context_str
        );

        self
    }
}

/// Convenience macro for creating wide events
#[macro_export]
macro_rules! wide_event {
    ($name:expr) => {
        $crate::core::logging::WideEvent::new($name)
    };
}
