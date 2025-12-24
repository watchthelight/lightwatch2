//! Experience clock - the 143-second heartbeat of LIGHTWATCH

use bevy::prelude::*;

/// The total duration of the LIGHTWATCH experience
pub const EXPERIENCE_DURATION: f32 = 143.0;

/// Experience timeline phases
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Phase {
    #[default]
    Signal,     // 0-2s: Detection
    Bang,       // 2-12s: Overwhelming awe
    Awakening,  // 12-27s: Quiet emergence
    Discovery,  // 27-57s: Wonder, search
    Connection, // 57-87s: Warmth, unity
    Acceptance, // 87-143s: Loss, peace, silence
    Ended,      // 143s+: Experience complete
}

impl Phase {
    /// Get the phase for a given elapsed time
    pub fn from_elapsed(elapsed: f32) -> Self {
        match elapsed {
            t if t < 2.0 => Phase::Signal,
            t if t < 12.0 => Phase::Bang,
            t if t < 27.0 => Phase::Awakening,
            t if t < 57.0 => Phase::Discovery,
            t if t < 87.0 => Phase::Connection,
            t if t < EXPERIENCE_DURATION => Phase::Acceptance,
            _ => Phase::Ended,
        }
    }

    /// Get phase start time
    pub fn start_time(&self) -> f32 {
        match self {
            Phase::Signal => 0.0,
            Phase::Bang => 2.0,
            Phase::Awakening => 12.0,
            Phase::Discovery => 27.0,
            Phase::Connection => 57.0,
            Phase::Acceptance => 87.0,
            Phase::Ended => EXPERIENCE_DURATION,
        }
    }

    /// Get phase end time
    pub fn end_time(&self) -> f32 {
        match self {
            Phase::Signal => 2.0,
            Phase::Bang => 12.0,
            Phase::Awakening => 27.0,
            Phase::Discovery => 57.0,
            Phase::Connection => 87.0,
            Phase::Acceptance => EXPERIENCE_DURATION,
            Phase::Ended => f32::INFINITY,
        }
    }

    /// Get phase duration
    pub fn duration(&self) -> f32 {
        self.end_time() - self.start_time()
    }

    /// Get progress within this phase (0.0 to 1.0)
    pub fn progress(&self, elapsed: f32) -> f32 {
        let start = self.start_time();
        let duration = self.duration();
        ((elapsed - start) / duration).clamp(0.0, 1.0)
    }

    /// Get display name
    pub fn name(&self) -> &'static str {
        match self {
            Phase::Signal => "signal",
            Phase::Bang => "bang",
            Phase::Awakening => "awakening",
            Phase::Discovery => "discovery",
            Phase::Connection => "connection",
            Phase::Acceptance => "acceptance",
            Phase::Ended => "ended",
        }
    }
}

/// The master clock for the experience
#[derive(Resource)]
pub struct ExperienceClock {
    /// Elapsed time since experience started
    elapsed: f32,
    /// Is the experience running?
    running: bool,
    /// Time scale (1.0 = normal, 2.0 = double speed)
    time_scale: f32,
    /// Current phase
    current_phase: Phase,
    /// Previous phase (for detecting transitions)
    previous_phase: Phase,
    /// Has the experience started?
    started: bool,
}

impl Default for ExperienceClock {
    fn default() -> Self {
        Self {
            elapsed: 0.0,
            running: false,
            time_scale: 1.0,
            current_phase: Phase::Signal,
            previous_phase: Phase::Signal,
            started: false,
        }
    }
}

impl ExperienceClock {
    /// Start the experience
    pub fn start(&mut self) {
        if !self.started {
            self.started = true;
            self.running = true;
            self.elapsed = 0.0;
            info!(target: "lightwatch::clock", "Experience started");
        }
    }

    /// Pause the clock (dev only)
    #[cfg(debug_assertions)]
    pub fn pause(&mut self) {
        self.running = false;
    }

    /// Resume the clock (dev only)
    #[cfg(debug_assertions)]
    pub fn resume(&mut self) {
        self.running = true;
    }

    /// Toggle pause (dev only)
    #[cfg(debug_assertions)]
    pub fn toggle_pause(&mut self) {
        self.running = !self.running;
    }

    /// Is the clock running?
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Has the experience started?
    pub fn has_started(&self) -> bool {
        self.started
    }

    /// Get elapsed time
    pub fn elapsed(&self) -> f32 {
        self.elapsed
    }

    /// Get remaining time
    pub fn remaining(&self) -> f32 {
        (EXPERIENCE_DURATION - self.elapsed).max(0.0)
    }

    /// Get progress through entire experience (0.0 to 1.0)
    pub fn progress(&self) -> f32 {
        (self.elapsed / EXPERIENCE_DURATION).clamp(0.0, 1.0)
    }

    /// Get current phase
    pub fn phase(&self) -> Phase {
        self.current_phase
    }

    /// Get progress within current phase (0.0 to 1.0)
    pub fn phase_progress(&self) -> f32 {
        self.current_phase.progress(self.elapsed)
    }

    /// Did we just transition to a new phase?
    pub fn phase_just_changed(&self) -> bool {
        self.current_phase != self.previous_phase
    }

    /// Get previous phase (for transition detection)
    pub fn previous_phase(&self) -> Phase {
        self.previous_phase
    }

    /// Set time scale (dev only)
    #[cfg(debug_assertions)]
    pub fn set_time_scale(&mut self, scale: f32) {
        self.time_scale = scale.clamp(0.1, 10.0);
    }

    /// Get current time scale
    pub fn time_scale(&self) -> f32 {
        self.time_scale
    }

    /// Jump to specific time (dev only)
    #[cfg(debug_assertions)]
    pub fn jump_to(&mut self, time: f32) {
        self.elapsed = time.clamp(0.0, EXPERIENCE_DURATION + 1.0);
        self.update_phase();
        info!(
            target: "lightwatch::clock",
            "Jumped to {:.2}s ({})",
            self.elapsed,
            self.current_phase.name()
        );
    }

    /// Update the clock - call every frame
    pub fn tick(&mut self, delta: f32) {
        if !self.running {
            return;
        }

        // Advance time
        self.elapsed += delta * self.time_scale;

        // Clamp at end
        if self.elapsed >= EXPERIENCE_DURATION {
            self.elapsed = EXPERIENCE_DURATION;
            self.running = false;
        }

        // Update phase
        self.update_phase();
    }

    fn update_phase(&mut self) {
        self.previous_phase = self.current_phase;
        self.current_phase = Phase::from_elapsed(self.elapsed);
    }
}

/// System to update the experience clock
pub fn update_clock(
    mut clock: ResMut<ExperienceClock>,
    time: Res<Time>,
    time_control: Res<super::TimeControl>,
) {
    // Apply time control overrides (dev only)
    #[cfg(debug_assertions)]
    {
        if clock.has_started() && time_control.paused != !clock.is_running() {
            if time_control.paused {
                clock.pause();
            } else {
                clock.resume();
            }
        }

        if time_control.speed != clock.time_scale() {
            clock.set_time_scale(time_control.speed);
        }

        if let Some(target_time) = time_control.scrub_position {
            clock.jump_to(target_time);
        }
    }

    // Suppress unused warning in release
    let _ = &time_control;

    // Tick the clock
    clock.tick(time.delta_seconds());
}

/// Clear scrub position after applying
#[cfg(debug_assertions)]
pub fn clear_scrub_position(mut time_control: ResMut<super::TimeControl>) {
    time_control.scrub_position = None;
}

/// Event fired when phase changes
#[derive(Event, Debug, Clone)]
pub struct PhaseChangedEvent {
    pub from: Phase,
    pub to: Phase,
    pub elapsed: f32,
}

/// System to emit phase change events
pub fn emit_phase_changes(
    clock: Res<ExperienceClock>,
    mut events: EventWriter<PhaseChangedEvent>,
) {
    if clock.phase_just_changed() {
        let event = PhaseChangedEvent {
            from: clock.previous_phase(),
            to: clock.phase(),
            elapsed: clock.elapsed(),
        };

        events.send(event.clone());

        crate::wide_event!("phase_changed")
            .with_str("from", event.from.name())
            .with_str("to", event.to.name())
            .with_f32("elapsed", event.elapsed)
            .emit(event.elapsed);
    }
}
