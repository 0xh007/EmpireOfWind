/// This module provides a system for spawning furniture items on the ship.
/// Currently, it includes the creation of a bed object.
///
/// Note: This system is expected to be deprecated as the game evolves and more
/// sophisticated systems for spawning and managing furniture are developed.

use bevy::asset::Assets;
use bevy::core::Name;
use bevy::math::{Quat, Vec3};
use bevy::pbr::{PbrBundle, StandardMaterial};
use bevy::prelude::{default, Color, Commands, Cuboid, Mesh, ResMut, Transform};
use bevy_xpbd_3d::components::{Friction, RigidBody};
use bevy_xpbd_3d::prelude::Collider;
use bevy::color::palettes::css::BLUE;


use crate::ship_items::SleepArea;

/// Spawns a bed entity in the game world.
///
/// This function creates a simple bed entity using Bevy's PBR (Physically Based Rendering)
/// components and some physics properties. The bed is positioned at a specific location on the ship
/// and is marked as a `SleepArea` where characters can rest.
///
/// # Parameters
///
/// * `commands`: The Commands resource is used to spawn and configure entities.
/// * `meshes`: A mutable reference to the Assets resource containing Mesh objects.
/// * `materials`: A mutable reference to the Assets resource containing StandardMaterial objects.
///
/// # Components
///
/// * `Name`: Assigns a name to the entity ("Bed").
/// * `PbrBundle`: Combines mesh, material, and transform data for the entity.
/// * `SleepArea`: A custom component marking the entity as a sleeping area.
/// * `RigidBody::Dynamic`: Marks the entity as a dynamic rigid body for physics simulation.
/// * `Friction`: Sets the friction coefficient for the entity's collider.
/// * `Collider::cuboid`: Defines the entity's collision shape as a cuboid.
pub fn spawn_furniture(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create a bed
    commands.spawn((
        Name::new("Bed"),
        PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(4.0, 1.0, 2.0))),
            material: materials.add(StandardMaterial {
                base_color: Color::from(BLUE),
                ..default()
            }),
            transform: Transform {
                translation: Vec3::new(-14.155, 8.4, -0.147),
                rotation: Quat::from_rotation_z(-9.8367f32.to_radians()),
                scale: Vec3::ONE,
            },
            ..default()
        },
        SleepArea,
        RigidBody::Dynamic,
        Friction::new(1.0),
        Collider::cuboid(5.0, 1.0, 5.0),
    ));
}
