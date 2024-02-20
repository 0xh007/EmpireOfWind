use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_xpbd_3d::prelude::*;
use oxidized_navigation::NavMeshAffector;

use crate::prelude::*;

#[derive(Component)]
struct ColliderMarker;

pub struct NavmeshtestPlugin;

impl Plugin for NavmeshtestPlugin {
    fn build(&self, app: &mut App) {
        app.configure_loading_state(
            LoadingStateConfig::new(AppStates::AssetLoading).load_collection::<NavmeshtestAssets>(),
        )
        .add_systems(OnEnter(AppStates::Next), spawn_test_assets);
    }
}

#[derive(AssetCollection, Resource)]
struct NavmeshtestAssets {
    #[asset(path = "models/export/test/navmeshtest_2.glb#Scene0")]
    test_asset: Handle<Scene>,
}

fn spawn_test_assets(
    mut commands: Commands,
    test_assets: Res<NavmeshtestAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // commands.spawn((
    //     PbrBundle {
    //         mesh: meshes.add(Mesh::from(bevy::prelude::shape::Cube { size: 5.0 })),
    //         material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    //         ..default()
    //     },
    //     AsyncCollider(ComputedCollider::TriMesh),
    //     RigidBody::Static,
    //     NavMeshAffector,
    // ));
    commands.spawn((
        SceneBundle {
            scene: test_assets.test_asset.clone(),
            visibility: Visibility::Hidden,
            ..default()
        },
        AsyncSceneCollider::new(Some(ComputedCollider::TriMesh)),
        // AsyncSceneCollider::new(Some(ComputedCollider::ConvexDecomposition(
        //     VHACDParameters::default(),
        // ))),
        RigidBody::Static,
        // NavMeshAffector,
    ));
}
