use bevy::prelude::*;

pub use components::*;
use systems::*;

use crate::asset_management::states::app_states::AppStates;

mod components;
mod systems;

/// Plugin for managing food within the game world.
///
/// The `FoodPlugin` handles the creation and management of food items, setting up the necessary
/// components and systems to spawn food entities in the game world.
///
/// # Components
/// - `Food`: A marker component indicating that an entity is a food item.
///
/// # Systems
/// - `spawn_food`: Spawns food items in the game world and configures their initial properties and behavior.
pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Food>()
            .add_systems(OnEnter(AppStates::Running), spawn_food);
    }
}
