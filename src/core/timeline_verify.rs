//! Timeline verification - Ensures phases transition at correct times

use bevy::prelude::*;

use super::{ExperienceClock, Phase};

/// Timeline verification resource
#[derive(Resource, Default)]
pub struct TimelineVerification {
    /// Phases entered with their timestamps
    phases_entered: Vec<(Phase, f32)>,
    /// Last logged phase
    last_phase: Option<Phase>,
    /// Verification logged
    logged: bool,
}

/// Verify timeline events occur at correct times
pub fn verify_timeline(
    clock: Res<ExperienceClock>,
    mut verify: ResMut<TimelineVerification>,
) {
    let elapsed = clock.elapsed();
    let phase = clock.phase();

    // Record phase entries
    if verify.last_phase != Some(phase) {
        verify.last_phase = Some(phase);
        verify.phases_entered.push((phase, elapsed));

        // Verify timing
        let expected = match phase {
            Phase::Signal => 0.0,
            Phase::Bang => 2.0,
            Phase::Awakening => 12.0,
            Phase::Discovery => 27.0,
            Phase::Connection => 57.0,
            Phase::Acceptance => 87.0,
            Phase::Ended => 143.0,
        };

        let delta = (elapsed - expected).abs();
        if delta > 0.5 {
            warn!(
                target: "lightwatch::timeline",
                "Phase {:?} entered at {:.1}s, expected {:.1}s (delta: {:.1}s)",
                phase, elapsed, expected, delta
            );
        } else {
            debug!(
                target: "lightwatch::timeline",
                "Phase {:?} entered on time at {:.1}s",
                phase, elapsed
            );
        }
    }
}

/// Log final verification summary
pub fn log_timeline_verification(
    clock: Res<ExperienceClock>,
    mut verify: ResMut<TimelineVerification>,
) {
    if clock.elapsed() >= 143.0 && !verify.logged {
        verify.logged = true;

        debug!(target: "lightwatch::timeline", "=== Timeline Verification ===");
        for (phase, time) in &verify.phases_entered {
            debug!(target: "lightwatch::timeline", "  {:?}: {:.1}s", phase, time);
        }
        info!(target: "lightwatch::timeline", "Experience complete at {:.1}s", clock.elapsed());
    }
}
