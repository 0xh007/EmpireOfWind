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
        .add_systems(OnEnter(AppStates::Next), spawn_furniture)
        .add_systems(OnEnter(AppStates::Next), spawn_food);
    }
}

#[derive(Bundle, Debug)]
struct ColliderBundle {
    name: Name,
    collider_shape: Collider,
    rigid_body_type: RigidBody,
    transform: TransformBundle,
}

#[derive(AssetCollection, Resource)]
struct ShipAssets {
    #[asset(path = "models/export/ship/carrack.glb#Scene0")]
    carrack_hull: Handle<Scene>,
}

fn spawn_furniture(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create a bed
    commands.spawn((
        Name::new("Bed"),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 1.0, 2.0))),
            material: materials.add(Color::MAROON.into()),
            transform: Transform {
                translation: Vec3::new(-14.155, 7.8825, -0.147),
                rotation: Quat::from_rotation_z(-9.8367f32.to_radians()),
                scale: Vec3::ONE,
            },
            ..default()
        },
        SleepArea,
    ));
}

fn spawn_food(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("Food"),
        Food,
        PbrBundle {
            mesh: meshes.add(
                shape::Icosphere {
                    radius: 0.2,
                    subdivisions: 20,
                }
                .try_into()
                .unwrap(),
            ),
            material: materials.add(Color::RED.into()),
            transform: Transform::from_xyz(13.167, 6.1885, 0.0),
            ..default()
        },
    ));
}

fn spawn_ship(mut commands: Commands, ship_assets: Res<ShipAssets>) {
    commands.spawn(({
        SceneBundle {
            scene: ship_assets.carrack_hull.clone(),
            ..default()
        }
    },));
}
