use bevy::prelude::*;

/// Marker component for a place where a character can sleep.
///
/// The `SleepArea` component is used to designate areas or objects within the game world
/// where characters are allowed to sleep. It is typically added to entities like beds or
/// sleeping quarters.
///
/// # Usage
///
/// ## Example: Adding a Sleep Area to an Object
///
/// ```rust
/// use bevy::prelude::*;
/// use empire_of_wind::components::SleepArea;
///
/// fn spawn_bed(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
///     commands.spawn((
///         PbrBundle {
///             mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
///             material: materials.add(StandardMaterial {
///                 base_color: Color::rgb(0.2, 0.7, 0.2),
///                 ..default()
///             }),
///             transform: Transform::from_translation(Vec3::new(0.0, 0.5, 0.0)),
///             ..default()
///         },
///         SleepArea,
///     ));
/// }
/// ```
#[derive(Component, Debug, Clone)]
pub struct SleepArea;
