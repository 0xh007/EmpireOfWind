use bevy::asset::Assets;
use bevy::core::Name;
use bevy::pbr::{PbrBundle, StandardMaterial};
use bevy::prelude::{Color, Commands, default, Mesh, Meshable, ResMut, Sphere, Transform};
use bevy_xpbd_3d::components::{Friction, RigidBody};
use bevy_xpbd_3d::prelude::Collider;

use crate::food::Food;

/// System to spawn food entities in the game.
///
/// This system creates food entities with specified properties, including a mesh,
/// material, transform, and physical properties. The food entities are configured
/// to have a dynamic rigid body, friction, and a spherical collider for physics interactions.
///
/// # Parameters
/// - `commands`: Commands for spawning and configuring entities.
/// - `meshes`: Resource to store and manage meshes.
/// - `materials`: Resource to store and manage materials.
///
/// # Details
/// The food entities are created with the following characteristics:
/// - Name: "Food"
/// - Mesh: A red sphere with a radius of 0.2 units.
/// - Transform: Positioned at coordinates (13.167, 7.1885, 0.0).
/// - Physics: Dynamic rigid body, friction of 1.0, and a spherical collider.
pub fn spawn_food(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("Food"),
        Food,
        PbrBundle {
            mesh: meshes.add(Sphere::new(0.2).mesh().ico(5).unwrap()),
            material: materials.add(Color::RED),
            transform: Transform::from_xyz(13.167, 7.1885, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        Friction::new(1.0),
        Collider::sphere(0.2),
    ));
}
