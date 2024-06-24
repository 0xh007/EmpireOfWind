use bevy::prelude::Component;

/// A marker component indicating that an entity is a ship.
///
/// This component is used to distinguish entities that represent ships within the game world.
///
/// # Example
///
/// ```
/// use bevy::prelude::*;
/// use empire_of_wind::components::Ship;
///
/// fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
///     // Load a ship model
///     let ship_handle = asset_server.load("models/ship.gltf#Scene0");
///
///     // Create an entity representing a ship
///     commands.spawn((
///         Ship,
///         SceneBundle {
///             scene: ship_handle,
///             ..default()
///         },
///         // Additional components specific to the ship entity
///     ));
/// }
///
/// fn main() {
///     App::new()
///         .add_plugins(DefaultPlugins)
///         .add_startup_system(setup)
///         .run();
/// }
/// ```
#[derive(Component)]
pub struct Ship;
