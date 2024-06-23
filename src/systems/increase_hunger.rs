use bevy::prelude::{Query, Res, Time};
use bevy::log::trace;
use crate::components::Hunger;

/// Increases an entity's hunger over time
pub fn increase_hunger(time: Res<Time>, mut hungers: Query<&mut Hunger>) {
    for mut hunger in &mut hungers {
        hunger.level += hunger.per_second * time.delta_seconds();
        if hunger.level >= 100.0 {
            hunger.level = 100.0;
        }
        trace!("Hunger: {}", hunger.level);
    }
}
