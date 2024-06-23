use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Marker component to designate an entity that should influence the navigation mesh.
///
/// This component is used to mark entities that will affect the navigation mesh generation or modification.
/// Entities with this component are processed to include navigation mesh affector properties.
///
/// # Example
///
/// ```
/// use bevy::prelude::*;
/// use empire_of_wind::components::nav_mesh_marker::NavMeshMarker;
///
/// fn setup(mut commands: Commands) {
///     commands.spawn((
///         NavMeshMarker,
///         Transform::default(),
///         GlobalTransform::default(),
///     ));
/// }
///
/// fn main() {
///     App::new()
///         .add_plugins(DefaultPlugins)
///         .add_startup_system(setup.system())
///         .run();
/// }
/// ```
///
/// This example demonstrates how to add the `NavMeshMarker` component to an entity in a Bevy app.
#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct NavMeshMarker;

