use bevy::prelude::*;
use bevy::render::mesh::VertexAttributeValues;
use bevy_asset_loader::prelude::*;
use bevy_water::WaterParam;
use bevy_xpbd_3d::components::ExternalForce;
use bevy_xpbd_3d::prelude::*;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_systems(OnEnter(AppStates::Next), spawn_cube)
            // .add_systems(Update, calculate_and_apply_buoyancy)
            .register_type::<BuoyancyMarker>()
            .add_systems(
                Update,
                read_buoyancy_objects.run_if(in_state(AppStates::Next)),
            )
            // .add_systems(Update, visualize_voxels.run_if(in_state(AppStates::Next)))
            .add_systems(
                Update,
                update_voxel_solidity.run_if(in_state(AppStates::Next)),
            )
            .configure_loading_state(
                LoadingStateConfig::new(AppStates::AssetLoading).load_collection::<ShipAssets>(),
            )
            .add_systems(OnEnter(AppStates::Next), spawn_ship);
        // .add_systems(OnEnter(AppStates::Next), spawn_furniture)
        // .add_systems(OnEnter(AppStates::Next), spawn_food);
    }
}

const VOXEL_SIZE: f32 = 0.8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vec3I {
    x: i32,
    y: i32,
    z: i32,
}

impl Vec3I {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Vec3I { x, y, z }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct BuoyancyMarker;

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

// #[derive(Component)]
// struct Buoyancy {
//     voxels: Vec<Voxel>,
//     cube_size: f32,
//     voxel_size: f32,
// }

// impl Buoyancy {
//     fn new(cube_size: f32, voxels_per_axis: usize) -> Self {
//         let voxel_size = cube_size / voxels_per_axis as f32;
//         let voxels = subdivide_cube_into_voxels(cube_size, voxels_per_axis, voxel_size);
//         Self {
//             voxels,
//             cube_size,
//             voxel_size,
//         }
//     }
// }

// impl Buoyancy {
//     fn new_from_mesh(mesh: &Mesh, voxels_per_axis: usize) -> Self {
//         // Pseudo-code to generate voxels based on mesh bounds and internal volume
//         let voxels = generate_voxels_from_mesh(mesh, voxels_per_axis);
//         Self { voxels }
//     }
// }

#[derive(Component)]
struct Voxel {
    position: Vec3,
    is_solid: bool,
}

fn update_voxel_solidity(
    mut commands: Commands,
    mut voxel_query: Query<(Entity, &Transform, &mut Voxel)>,
    mut spatial_query: SpatialQuery,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ensure the spatial query pipeline is up to date
    spatial_query.update_pipeline();

    let voxel_visual_size = VOXEL_SIZE * 0.95; // Example size and reduction to make gaps between voxels

    for (voxel_entity, voxel_transform, mut voxel) in voxel_query.iter_mut() {
        let voxel_collider = Collider::cuboid(VOXEL_SIZE / 2.0, VOXEL_SIZE / 2.0, VOXEL_SIZE / 2.0);

        let intersects = spatial_query.shape_intersections(
            &voxel_collider,
            voxel_transform.translation,
            voxel_transform.rotation,
            SpatialQueryFilter::default(), // Customize this as needed
        );

        voxel.is_solid = !intersects.is_empty();

        // If the voxel is solid, spawn a visual representation for it
        if voxel.is_solid {
            commands.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(
                    voxel_visual_size,
                    voxel_visual_size,
                    voxel_visual_size,
                )),
                material: materials.add(Color::rgb_u8(124, 144, 255)),
                transform: Transform::from_translation(voxel_transform.translation),
                ..default()
            });
            // .insert(VoxelVisual); // Optional: Tag visual voxels with a marker component for easy identification
        } else {
        }
    }
}

// fn visualize_voxels(
//     mut commands: Commands,
//     query: Query<(Entity, &Voxel), Added<Voxel>>,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     let voxel_visual_size = 0.5 * 0.95; // Example size and reduction to make gaps between voxels

//     for (entity, voxel) in query.iter() {
//         commands.entity(entity).insert(PbrBundle {
//             mesh: meshes.add(Cuboid::new(
//                 voxel_visual_size,
//                 voxel_visual_size,
//                 voxel_visual_size,
//             )),
//             material: materials.add(Color::rgb_u8(124, 144, 255)),
//             transform: Transform::from_translation(voxel.position),
//             ..default()
//         });
//     }
// }

fn generate_voxel_grid(commands: &mut Commands, mesh: &Mesh) {
    let bounds = calculate_mesh_bounds(mesh);
    let grid_size = calculate_grid_size(&bounds);

    for x in 0..grid_size.x {
        for y in 0..grid_size.y {
            for z in 0..grid_size.z {
                let position = Vec3::new(
                    x as f32 * VOXEL_SIZE + bounds.0.x + VOXEL_SIZE / 2.0 - bounds.1.x / 2.0,
                    y as f32 * VOXEL_SIZE + bounds.0.y + VOXEL_SIZE / 2.0 - bounds.1.y / 2.0,
                    z as f32 * VOXEL_SIZE + bounds.0.z + VOXEL_SIZE / 2.0 - bounds.1.z / 2.0,
                );

                // Now also spawn entities with a Transform component
                commands.spawn((
                    Voxel {
                        position,
                        is_solid: false, // This will be updated based on spatial queries later
                    },
                    Transform::from_translation(position), // Use the position for the Transform component
                    GlobalTransform::default(),
                ));
            }
        }
    }
}

fn calculate_mesh_bounds(mesh: &Mesh) -> (Vec3, Vec3) {
    let positions = if let Some(VertexAttributeValues::Float32x3(pos)) =
        mesh.attribute(Mesh::ATTRIBUTE_POSITION)
    {
        pos
    } else {
        panic!("Mesh does not contain position attribute.");
    };

    // Initialize min and max with the first vertex to ensure correctness.
    let mut min = Vec3::new(positions[0][0], positions[0][1], positions[0][2]);
    let mut max = min;

    for &vertex in positions.iter() {
        min = min.min(Vec3::from(vertex));
        max = max.max(Vec3::from(vertex));
    }

    (min, max)
}

fn calculate_grid_size(bounds: &(Vec3, Vec3)) -> Vec3I {
    let (min, max) = bounds;
    let size = *max - *min;

    Vec3I::new(
        (size.x / VOXEL_SIZE).ceil() as i32,
        (size.y / VOXEL_SIZE).ceil() as i32,
        (size.z / VOXEL_SIZE).ceil() as i32,
    )
}

pub fn read_buoyancy_objects(
    buoyancy_marker_query: Query<(Entity, &BuoyancyMarker), Added<BuoyancyMarker>>,
    mut commands: Commands,
    children: Query<&Children>,
    mut meshes: ResMut<Assets<Mesh>>, // Asset storage for Meshes
    mesh_handles: Query<&Handle<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, _) in buoyancy_marker_query.iter() {
        if let Some(mesh_handle) = find_mesh(entity, &children, &mesh_handles) {
            // Retrieve the actual Mesh from the Assets<Mesh> using the Handle<Mesh>
            if let Some(mesh) = meshes.get(mesh_handle) {
                generate_voxel_grid(
                    &mut commands,
                    mesh, // Pass the actual Mesh to the function
                );

                // Attempt to create a collider directly from the Mesh
                if let Some(collider) = Collider::trimesh_from_mesh(mesh) {
                    commands.entity(entity).insert((
                        collider,
                        RigidBody::Static,
                        Visibility::Visible,
                    ));
                }
            } else {
                eprintln!(
                    "Failed to retrieve mesh from handle for entity marked with BuoyancyMarker"
                );
            }
        } else {
            eprintln!("Mesh not found for entity marked with BuoyancyMarker");
        }
    }
}

fn find_mesh(
    parent: Entity,
    children_query: &Query<&Children>,
    mesh_handles: &Query<&Handle<Mesh>>,
) -> Option<Handle<Mesh>> {
    if let Ok(children) = children_query.get(parent) {
        for child in children.iter() {
            if let Ok(mesh_handle) = mesh_handles.get(*child) {
                return Some(mesh_handle.clone());
            }
        }
    }
    None
}

// fn calculate_and_apply_buoyancy(
//     water: WaterParam,
//     mut query: Query<(
//         &Buoyancy,
//         &Transform,
//         &mut ExternalForce,
//         &ColliderDensity,
//         &Collider,
//     )>,
// ) {
//     for (buoyancy, transform, mut external_force, collider_density, collider) in query.iter_mut() {
//         let mut total_buoyancy_force = Vec3::ZERO;
//         let gravity = 9.81;
//         let cube_volume = buoyancy.cube_size.powi(3);
//         let cube_weight = cube_volume * collider_density.0 * gravity;

//         for voxel in &buoyancy.voxels {
//             let world_position = transform.translation + voxel.position;
//             let water_height = get_water_height_at_position(world_position, &water);
//             let submerged_volume =
//                 calculate_submerged_volume(world_position, water_height, buoyancy.voxel_size);
//             let buoyancy_force = Vec3::new(0.0, gravity * submerged_volume, 0.0);

//             total_buoyancy_force += buoyancy_force;
//         }

//         // Limit the buoyancy force to not exceed the cube's weight
//         if total_buoyancy_force.y > cube_weight {
//             total_buoyancy_force.y = cube_weight;
//         }

//         external_force.apply_force(total_buoyancy_force);
//     }
// }

// fn calculate_submerged_volume(world_position: Vec3, water_height: f32, voxel_size: f32) -> f32 {
//     let bottom_of_voxel = world_position.y - voxel_size / 2.0;
//     let top_of_voxel = world_position.y + voxel_size / 2.0;

//     // If the top of the voxel is below the water, it's fully submerged
//     if top_of_voxel <= water_height {
//         return voxel_size.powi(3); // The volume of the voxel
//     }
//     // If the bottom of the voxel is above the water, it's not submerged
//     else if bottom_of_voxel >= water_height {
//         return 0.0;
//     }
//     // Otherwise, it's partially submerged
//     else {
//         let submerged_height = water_height - bottom_of_voxel;
//         return submerged_height * voxel_size * voxel_size; // The submerged volume
//     }
// }

fn get_water_height_at_position(pos: Vec3, water: &WaterParam) -> f32 {
    let water_height = water.wave_point(pos).y;
    water_height
}

fn spawn_ship(mut commands: Commands, ship_assets: Res<ShipAssets>) {
    commands.spawn((
        SceneBundle {
            scene: ship_assets.carrack_hull.clone(),
            ..default()
        },
        // WaterInteractable::new(-0.4, -8.0, 9.0, -2.0, 2.0),
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
