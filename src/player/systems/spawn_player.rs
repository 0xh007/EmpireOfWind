use bevy::asset::Assets;
use bevy::core::Name;
use bevy::pbr::{PbrBundle, StandardMaterial};
use bevy::prelude::{Capsule3d, Color, Commands, default, Mesh, ResMut, Transform};
use bevy_tnua::controller::TnuaControllerBundle;
use bevy_tnua_xpbd3d::TnuaXpbd3dSensorShape;
use bevy_xpbd_3d::components::{LockedAxes, RigidBody};
use bevy_xpbd_3d::prelude::Collider;

use crate::player::Player;

/// Spawns the player entity in the game world.
///
/// This system sets up the player entity with components for 3D physics and movement control. It utilizes:
///
/// - `bevy_xpbd_3d`: Provides the `RigidBody` and `Collider` components for physics simulation.
/// - `bevy_tnua`: Provides the `TnuaControllerBundle` for movement control.
///
/// The player is visually represented by a yellow capsule.
pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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
