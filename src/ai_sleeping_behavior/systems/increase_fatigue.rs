use bevy::log::trace;
use bevy::prelude::{Query, Res, Time};

use crate::ai_sleeping_behavior::Fatigue;

/// Increases an entity's fatigue over time.
///
/// This system increments the fatigue level of each entity based on the `per_second`
/// rate specified in their `Fatigue` component. The fatigue level is capped at 100.0.
///
/// # Parameters
/// - `time`: Resource providing the delta time for the game.
/// - `fatigues`: Query to fetch and modify the `Fatigue` component of entities.
pub fn increase_fatigue(time: Res<Time>, mut fatigues: Query<&mut Fatigue>) {
    for mut fatigue in &mut fatigues {
        fatigue.level += fatigue.per_second * time.delta_seconds();
        if fatigue.level >= 100.0 {
            fatigue.level = 100.0;
        }
        trace!("Tiredness: {}", fatigue.level);
    }
}
