use bevy::prelude::*;

pub use components::*;
use systems::*;

use crate::asset_management::states::app_states::AppStates;

mod components;
mod systems;

/// Plugin for managing ship items within the game.
///
/// The `ShipItemsPlugin` is responsible for setting up various interactive items within the ship,
/// such as furniture. It registers the necessary components and sets up systems to handle the
/// spawning of these items.
///
/// # Components
/// - `SleepArea`: A marker component for designating areas where characters can sleep.
///
/// # Systems
/// - `spawn_furniture`: Spawns furniture items such as beds in the game world.
pub struct ShipItemsPlugin;

impl Plugin for ShipItemsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SleepArea>()
            .add_systems(OnEnter(AppStates::Running), spawn_furniture);
    }
}
