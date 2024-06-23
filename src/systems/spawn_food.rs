use bevy::prelude::{Color, Commands, default, Mesh, Meshable, ResMut, Sphere, Transform};
use bevy::asset::Assets;
use bevy::pbr::{PbrBundle, StandardMaterial};
use bevy::core::Name;
use bevy_xpbd_3d::components::{Friction, RigidBody};
use bevy_xpbd_3d::prelude::Collider;
use crate::components::Food;

// TODO: Eviction notice
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
