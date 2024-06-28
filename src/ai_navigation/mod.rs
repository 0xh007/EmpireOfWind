use bevy::prelude::*;

pub use components::*;
pub use systems::*;

mod components;
mod constants;
mod systems;
mod utils;

/// Plugin for managing AI navigation behavior within the game.
///
/// The AiNavigationPlugin provides functionality for handling the navigation
/// behaviors of entities. It registers the necessary components and sets up
/// systems to manage navigation towards food and sleep areas.
///
/// # Components
/// - SeekFoodBehavior: Enables an entity to navigate towards the nearest `Food` target,
///   defining the movement speed.
/// - SeekSleepAreaBehavior: Enables an entity to navigate towards the nearest `SleepArea` target,
///   defining the movement speed.
/// - NavigationPath: Stores a sequence of points that define a navigation path in 3D space.
///
/// # Systems
/// - navigate_to_nearest: Manages the navigation of entities towards the nearest target of a specified type.
pub struct AiNavigationPlugin;

impl Plugin for AiNavigationPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<SeekFoodBehavior>()
            .register_type::<SeekSleepAreaBehavior>()
            .register_type::<NavigationPath>()
            .add_systems(PreUpdate,
                         (
                             navigate_to_nearest::<SleepArea>,
                             navigate_to_nearest::<Food>,
                         ),
            );
    }
}