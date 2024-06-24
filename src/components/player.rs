use bevy::prelude::*;

/// A marker component indicating that an entity is the main player.
///
/// This component is used to distinguish the main player entity within the game world.
///
/// # Example
///
/// ```
/// use bevy::prelude::*;
/// use empire_of_wind::components::Player;
///
/// fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
///     // Create an entity representing the player
///     commands.spawn((
///         Player,
///         PbrBundle {
///             mesh: meshes.add(Mesh::from(shape::Capsule {
///                 radius: 0.4,
///                 ..default()
///             })),
///             material: materials.add(Color::BLUE.into()),
///             transform: Transform::from_xyz(0.0, 1.0, 0.0),
///             ..default()
///         },
///         // Additional components specific to the player entity
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
pub struct Player;
