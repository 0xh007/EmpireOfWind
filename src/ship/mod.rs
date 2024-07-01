use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub use components::*;
use resources::*;
use systems::*;

use crate::asset_management::states::app_states::AppStates;

mod components;
mod resources;
mod systems;

/// Plugin for managing the ship entity within the game.
///
/// The `ShipPlugin` is responsible for loading and setting up the main ship entity in the game world.
/// It registers the necessary components, initializes resources, and sets up systems to handle ship spawning and asset loading.
///
/// # Components
/// - `Ship`: A marker component indicating that an entity is a ship.
///
/// # Resources
/// - `ShipAssets`: Contains handles to the ship assets used in the game.
///
/// # Systems
/// - `spawn_ship`: Spawns the ship entity with the necessary components and assets.
pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.configure_loading_state(
            LoadingStateConfig::new(AppStates::AssetLoading).load_collection::<ShipAssets>(),
        )
            .register_type::<Ship>()
            .add_systems(OnEnter(AppStates::Running), spawn_ship);
    }
}
