use bevy::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua_xpbd3d::TnuaXpbd3dSensorShape;
use bevy_xpbd_3d::{math::*, prelude::*};

use crate::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}

fn spawn_player(
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
        Collider::capsule(1.0, 0.4),
        TnuaControllerBundle::default(),
        TnuaXpbd3dSensorShape(Collider::cylinder(0.0, 0.49)),
        LockedAxes::ROTATION_LOCKED,
    ));
}
