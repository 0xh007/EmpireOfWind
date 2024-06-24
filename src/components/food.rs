use bevy::prelude::*;

/// Marker component for something edible.
///
/// The `Food` component is a marker component used to identify entities that can be consumed
/// by other entities. This component does not have any fields or behavior on its own.
/// It is typically used in systems where entities search for and interact with food items.
///
/// # Usage
///
/// ## Example: Spawning a Food Entity with Additional Components
///
/// ```rust
/// use bevy::prelude::*;
/// use bevy::pbr::{PbrBundle, StandardMaterial};
/// use bevy::asset::Assets;
/// use empire_of_wind::components::Food;
///
/// fn spawn_food(
///     mut commands: Commands,
///     mut meshes: ResMut<Assets<Mesh>>,
///     mut materials: ResMut<Assets<StandardMaterial>>,
/// ) {
///     commands.spawn((
///         Food,
///         PbrBundle {
///             mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
///             material: materials.add(StandardMaterial {
///                 base_color: Color::RED,
///                 ..Default::default()
///             }),
///             transform: Transform::from_xyz(0.0, 0.0, 0.0),
///             ..Default::default()
///         },
///         // Other components such as collider, physics body...
///     ));
/// }
///
/// fn main() {
///     App::new()
///         .add_startup_system(spawn_food.system())
///         .run();
/// }
/// ```
#[derive(Component, Debug, Clone)]
pub struct Food;
