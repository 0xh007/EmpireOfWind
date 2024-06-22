use bevy::log::trace;
use bevy::prelude::{Query, Res, Time};

use crate::components::Fatigue;

/// Increases an entity's fatigue over time
pub fn increase_fatigue(time: Res<Time>, mut fatigues: Query<&mut Fatigue>) {
    for mut fatigue in &mut fatigues {
        fatigue.level += fatigue.per_second * time.delta_seconds();
        if fatigue.level >= 100.0 {
            fatigue.level = 100.0;
        }
        trace!("Tiredness: {}", fatigue.level);
    }
}
