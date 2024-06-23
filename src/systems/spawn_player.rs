use bevy::prelude::{Capsule3d, Color, Commands, default, Mesh, ResMut, Transform};
use bevy::asset::Assets;
use bevy::pbr::{PbrBundle, StandardMaterial};
use bevy::log::info;
use bevy::core::Name;
use bevy_xpbd_3d::components::{LockedAxes, RigidBody};
use bevy_xpbd_3d::prelude::Collider;
use bevy_tnua::controller::TnuaControllerBundle;
use bevy_tnua_xpbd3d::TnuaXpbd3dSensorShape;
use crate::components::Player;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("Spawning player");
    commands.spawn((
        Name::new("Player"),
        Player,
        PbrBundle {
            mesh: meshes.add(Capsule3d {
                radius: 0.4,
                ..default()
            }),
            material: materials.add(Color::YELLOW),
            transform: Transform::from_xyz(-14.0, 14.5, -0.14),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::capsule(0.5, 0.5),
        TnuaControllerBundle::default(),
        TnuaXpbd3dSensorShape(Collider::cylinder(0.0, 0.49)),
        LockedAxes::ROTATION_LOCKED,
    ));
}
