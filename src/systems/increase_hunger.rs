use bevy::log::trace;
use bevy::prelude::{Query, Res, Time};

use crate::components::Hunger;

/// Increases an entity's hunger over time.
///
/// This system increments the hunger level of each entity based on the `per_second`
/// rate specified in their `Hunger` component. The hunger level is capped at 100.0.
///
/// # Parameters
/// - `time`: Resource providing the delta time for the game.
/// - `hungers`: Query to fetch and modify the `Hunger` component of entities.
///
/// # Example
/// The `increase_hunger` system should be added to your Bevy app like this:
/// ```rust
/// use bevy::prelude::*;
/// use empire_of_wind::systems::increase_hunger;
/// use empire_of_wind::components::Hunger;
///
/// fn main() {
///     App::new()
///         .add_plugins(DefaultPlugins)
///         .add_system(Update, increase_hunger)
///         .run();
/// }
/// ```
pub fn increase_hunger(time: Res<Time>, mut hungers: Query<&mut Hunger>) {
    for mut hunger in &mut hungers {
        hunger.level += hunger.per_second * time.delta_seconds();
        if hunger.level >= 100.0 {
            hunger.level = 100.0;
        }
        trace!("Hunger: {}", hunger.level);
    }
}
