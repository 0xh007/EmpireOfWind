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
        // .add_systems(OnEnter(AppStates::Next), spawn_ship_colliders)
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

    #[asset(path = "models/export/ship/colliders/carrack_top_deck_collider.glb#Scene0")]
    top_deck_collider: Handle<Scene>,

    #[asset(path = "models/export/ship/colliders/deck_1_bow_collider.glb#Scene0")]
    deck_1_bow_collider: Handle<Scene>,

    #[asset(path = "models/export/ship/colliders/deck_1_stern_aft_wall_collider.glb#Scene0")]
    deck_1_stern_aft_wall_collider: Handle<Scene>,

    #[asset(path = "models/export/ship/colliders/deck_1_stern_collider.glb#Scene0")]
    deck_1_stern_collider: Handle<Scene>,

    #[asset(path = "models/export/ship/colliders/deck_1_stern_port_wall_collider.glb#Scene0")]
    deck_1_stern_port_wall_collider: Handle<Scene>,

    #[asset(path = "models/export/ship/colliders/deck_1_stern_starboard_wall_collider.glb#Scene0")]
    deck_1_stern_starboard_wall_collider: Handle<Scene>,

    #[asset(path = "models/export/ship/colliders/deck_2_stern_collider.glb#Scene0")]
    deck_2_stern_collider: Handle<Scene>,

    #[asset(path = "models/export/ship/colliders/deck_3_collider.glb#Scene0")]
    deck_3_collider: Handle<Scene>,
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
    commands.spawn(({
        SceneBundle {
            scene: ship_assets.carrack_hull.clone(),
            ..default()
        }
    },));
}

// fn spawn_ship_colliders(mut commands: Commands, ship_assets: Res<ShipAssets>) {
//     // Create a vector of all collider handles
//     let collider_handles = vec![
//         &ship_assets.top_deck_collider,
//         &ship_assets.deck_1_bow_collider,
//         &ship_assets.deck_1_stern_aft_wall_collider,
//         &ship_assets.deck_1_stern_collider,
//         &ship_assets.deck_1_stern_port_wall_collider,
//         &ship_assets.deck_1_stern_starboard_wall_collider,
//         &ship_assets.deck_2_stern_collider,
//         &ship_assets.deck_3_collider,
//     ];
//
//     // Iterate over the collider handles and spawn each collider
//     for collider_handle in collider_handles {
//         commands.spawn((
//             SceneBundle {
//                 scene: collider_handle.clone(),
//                 visibility: Visibility::Hidden,
//                 ..default()
//             },
//             AsyncSceneCollider::new(Some(ComputedCollider::TriMesh)),
//             RigidBody::Static,
//             NavMeshAffector,
//         ));
//     }
// }
