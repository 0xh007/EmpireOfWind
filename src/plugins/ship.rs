use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_xpbd_3d::prelude::*;

use crate::prelude::*;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppStates::Next), spawn_cube);
        // app.configure_loading_state(
        //     LoadingStateConfig::new(AppStates::AssetLoading).load_collection::<ShipAssets>(),
        // )
        // .add_systems(OnEnter(AppStates::Next), spawn_ship);
        // .add_systems(OnEnter(AppStates::Next), spawn_furniture)
        // .add_systems(OnEnter(AppStates::Next), spawn_food);
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
    #[asset(path = "models/export/ship/carrack_b.glb#Scene0")]
    carrack_hull: Handle<Scene>,
}

#[derive(Component)]
struct Buoyancy {
    voxels: Vec<Voxel>,
    cube_size: f32,
    // Add other relevant fields as necessary.
}

impl Buoyancy {
    fn new(cube_size: f32, voxels_per_axis: usize) -> Self {
        let voxels = subdivide_cube_into_voxels(cube_size, voxels_per_axis);
        Self { voxels, cube_size }
    }
}

struct Voxel {
    position: Vec3,
    is_receiver: bool,
}

fn subdivide_cube_into_voxels(cube_size: f32, voxels_per_axis: usize) -> Vec<Voxel> {
    let voxel_size = cube_size / voxels_per_axis as f32;
    let mut voxels = Vec::new();

    for x in 0..voxels_per_axis {
        for y in 0..voxels_per_axis {
            for z in 0..voxels_per_axis {
                let position = Vec3::new(
                    (x as f32 + 0.5) * voxel_size - cube_size / 2.0,
                    (y as f32 + 0.5) * voxel_size - cube_size / 2.0,
                    (z as f32 + 0.5) * voxel_size - cube_size / 2.0,
                );
                voxels.push(Voxel {
                    position,
                    is_receiver: true, // Initially set all voxels as receivers.
                });
            }
        }
    }
    voxels
}

fn spawn_cube(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let cube_size = 2.0; // Make sure this matches the size used for the Collider::cuboid
    let voxels_per_axis = 5; // This is an arbitrary choice; adjust based on desired detail level.

    let cube_mesh = meshes.add(Mesh::from(shape::Cube { size: cube_size }));
    let buoyancy_component = Buoyancy::new(cube_size, voxels_per_axis);

    commands.spawn((
        PbrBundle {
            mesh: cube_mesh,
            material: materials.add(Color::rgb(0.2, 0.7, 0.9).into()),
            transform: Transform::from_xyz(0.0, 20.0, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(cube_size / 2.0, cube_size / 2.0, cube_size / 2.0), // Adjust accordingly
        buoyancy_component,
    ));
}

fn spawn_ship(mut commands: Commands, ship_assets: Res<ShipAssets>) {
    commands.spawn((
        SceneBundle {
            scene: ship_assets.carrack_hull.clone(),
            ..default()
        },
        WaterInteractable::new(-0.4, -8.0, 9.0, -2.0, 2.0),
    ));
}

// TODO: Eviction notice
fn spawn_furniture(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create a bed
    commands.spawn((
        Name::new("Bed"),
        PbrBundle {
            mesh: meshes.add(Cuboid::new(5.0, 0.15, 5.0)),
            material: materials.add(Color::MAROON),
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
        Collider::sphere(0.2),
    ));
}
