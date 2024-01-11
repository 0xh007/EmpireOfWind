use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ship);
    }
}

fn spawn_ship(mut commands: Commands, assets: ResMut<AssetServer>) {
    commands.spawn((
        SceneBundle {
            scene: assets.load("models/export/ship/ship.glb#Scene0"),
            transform: Transform::from_xyz(0.0, 9.0, 0.0),
            ..default()
        },
        AsyncSceneCollider::new(Some(ComputedCollider::TriMesh)),
        RigidBody::Static,
    ));
}
