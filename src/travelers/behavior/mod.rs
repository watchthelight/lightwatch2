//! Traveler behavior systems - pulse, sync, drift, anchor, orbit, grief

use bevy::prelude::*;

pub mod anchor;
pub mod drift;
pub mod grief;
pub mod orbit;
pub mod pulse;
pub mod sync;

pub use anchor::*;
pub use drift::*;
pub use grief::*;
pub use orbit::*;
pub use pulse::*;
pub use sync::*;

use crate::travelers::Traveler;

/// Add behavior components when traveler spawns
pub fn setup_traveler_behaviors(
    mut commands: Commands,
    travelers: Query<(Entity, &Traveler), Added<Traveler>>,
) {
    for (entity, traveler) in travelers.iter() {
        commands.entity(entity).insert((
            TravelerDrift::for_traveler(traveler.id),
            TravelerAnchor::for_traveler(traveler.id),
            TravelerOrbit::for_traveler(traveler.id),
        ));
    }
}

/// Plugin for traveler behaviors
pub struct TravelerBehaviorPlugin;

impl Plugin for TravelerBehaviorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                setup_traveler_behaviors,
                update_traveler_pulse,
                handle_sync_events,
                gradual_sync_during_connection,
                break_sync_during_acceptance,
                apply_traveler_drift,
                apply_anchor_behavior,
                apply_orbit_behavior,
                apply_grief_behavior,
                traveler_specific_grief,
            ),
        );
    }
}
