use bevy::math::primitives::Cuboid;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_xpbd_3d::prelude::*;

use crate::prelude::*;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.configure_loading_state(
            LoadingStateConfig::new(AppStates::AssetLoading).load_collection::<ShipAssets>(),
        )
        .add_systems(OnEnter(AppStates::Next), spawn_ship)
        .add_systems(OnEnter(AppStates::Next), spawn_food)
        .add_systems(OnEnter(AppStates::Next), spawn_furniture);
    }
}

#[derive(Component)]
pub struct Ship;

#[derive(AssetCollection, Resource)]
pub struct ShipAssets {
    #[asset(path = "models/export/ship/carrack_2.glb#Scene0")]
    pub ship: Handle<Scene>,
}

fn spawn_ship(mut commands: Commands, ship_assets: Res<ShipAssets>) {
    commands.spawn((
        Ship, // Add this marker component
        Name::new("Ship"),
        SceneBundle {
            scene: ship_assets.ship.clone(),
            ..default()
        },
    ));
}

// TODO: Eviction notice
fn spawn_furniture(
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

// TODO: Eviction notice
fn spawn_food(
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
