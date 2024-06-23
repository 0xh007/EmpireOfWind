use bevy::prelude::{Color, Commands, Cuboid, default, Mesh, ResMut, Transform};
use bevy::asset::Assets;
use bevy::pbr::{PbrBundle, StandardMaterial};
use bevy::core::Name;
use bevy::math::{Quat, Vec3};
use bevy_xpbd_3d::components::{Friction, RigidBody};
use bevy_xpbd_3d::prelude::Collider;
use crate::components::SleepArea;

// TODO: Eviction notice
pub fn spawn_furniture(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    println!("delete this println");
    // Create a bed
    commands.spawn((
        Name::new("Bed"),
        PbrBundle {
            mesh: meshes.add(Cuboid::new(4.0, 1.0, 2.0)),
            material: materials.add(Color::BLUE),
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
