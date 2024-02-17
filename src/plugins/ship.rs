use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_xpbd_3d::prelude::*;
use oxidized_navigation::NavMeshAffector;

use crate::prelude::*;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.configure_loading_state(
            LoadingStateConfig::new(AppStates::AssetLoading).load_collection::<ShipAssets>(),
        )
        .add_systems(OnEnter(AppStates::Next), spawn_ship)
        .add_systems(OnEnter(AppStates::Next), spawn_top_deck_collider)
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
    #[asset(path = "models/export/ship/carrack_hull.glb#Scene0")]
    carrack_hull: Handle<Scene>,

    #[asset(path = "models/export/ship/carrack_top_deck_collider.glb#Scene0")]
    top_deck_collider: Handle<Scene>,
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
            mesh: meshes.add(Mesh::from(shape::Box::new(2.0, 1.0, 1.0))),
            material: materials.add(Color::MAROON.into()),
            transform: Transform {
                translation: Vec3::new(0.209, 8.0, 13.93),
                rotation: Quat::from_rotation_x(-9.099f32.to_radians()),
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
            transform: Transform::from_xyz(0.27241, 5.9766, -11.16),
            ..default()
        },
    ));
}

// Blender Y == Bevy -Z
// Blender X == Bevy X
// Blender Z == Bevy Y
fn spawn_ship(mut commands: Commands, ship_assets: Res<ShipAssets>) {
    commands.spawn(SceneBundle {
        scene: ship_assets.carrack_hull.clone(),
        // transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    // commands.spawn((
    //     ColliderBundle {
    //         name: Name::new("Deck 3 - 1"),
    //         collider_shape: Collider::cuboid(9.5, 0.1, 6.39),
    //         rigid_body_type: RigidBody::Static,
    //         transform: TransformBundle::from(Transform {
    //             translation: Vec3::new(0.0, 7.1281, 13.3236),
    //             rotation: Quat::from_rotation_x(-9.43791f32.to_radians()),
    //             scale: Vec3::ONE,
    //         }),
    //     },
    //     NavMeshAffector,
    // ));
}

fn spawn_top_deck_collider(mut commands: Commands, ship_assets: Res<ShipAssets>) {
    commands.spawn((
        SceneBundle {
            scene: ship_assets.top_deck_collider.clone(),
            visibility: Visibility::Hidden,
            // transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        AsyncSceneCollider::new(Some(ComputedCollider::TriMesh)),
        RigidBody::Static,
        NavMeshAffector,
    ));
}
