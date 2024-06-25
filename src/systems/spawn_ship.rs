use bevy::prelude::{Commands, default, Res, SceneBundle};
use bevy::core::Name;

use crate::prelude::*;

/// Spawns the main ship entity in the game world.
///
/// This function utilizes preloaded ship assets to create and configure the ship entity.
/// The ship entity is assigned various components to integrate it into the game's
/// entity-component system.
///
/// # Parameters
///
/// * `commands`: The Commands resource is used to spawn and configure entities.
/// * `ship_assets`: A reference to the ShipAssets resource, which contains preloaded
/// assets for the ship entity.
///
/// # Components
///
/// * `Ship`: A custom component that identifies the entity as the main ship.
/// * `Name`: Assigns a name to the entity ("Ship").
/// * `SceneBundle`: Combines the scene asset and other necessary data for the ship entity.
pub fn spawn_ship(mut commands: Commands, ship_assets: Res<ShipAssets>) {
    commands.spawn((
        Ship,
        Name::new("Ship"),
        SceneBundle {
            scene: ship_assets.ship.clone(),
            ..default()
        },
    ));
}
