use bevy::prelude::*;
use bevy::render::mesh::VertexAttributeValues;
use bevy_tnua::prelude::*;
use bevy_tnua_xpbd3d::*;
use bevy_water::WaterParam;
use bevy_xpbd_3d::math::Matrix3;
use bevy_xpbd_3d::prelude::*;
use oxidized_navigation::NavMeshAffector;
use serde::{Deserialize, Serialize};

use crate::plugins::ship::Ship;
use crate::prelude::*;

const VOXEL_SIZE: f32 = 2.0;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        // TODO: Add xpbd in here
        app.add_plugins(TnuaControllerPlugin)
            .add_plugins(TnuaXpbd3dPlugin)
            .register_type::<AreaName>()
            .register_type::<AreaEnterMarker>()
            .register_type::<AreaExitMarker>()
            .register_type::<BuoyancyMarker>()
            .register_type::<ColliderMarker>()
            .register_type::<COMMarker>()
            .register_type::<Hideable>()
            .register_type::<NavMeshMarker>()
            .add_systems(Update, hide_show_objects.run_if(in_state(AppStates::Next)))
            .add_systems(Update, read_area_markers.run_if(in_state(AppStates::Next)))
            .add_systems(
                Update,
                read_buoyancy_objects.run_if(in_state(AppStates::Next)),
            )
            .add_systems(
                Update,
                update_voxel_solidity.run_if(in_state(AppStates::Next)),
            )
            .add_systems(
                Update,
                calculate_and_apply_buoyancy.run_if(in_state(AppStates::Next)),
            )
            // .add_systems(
            //     Update,
            //     visualize_voxel_grid.run_if(in_state(AppStates::Next)),
            // );
            .add_systems(Update, read_colliders.run_if(in_state(AppStates::Next)));
    }
}

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

// #[derive(Component)]
// pub struct VoxelVisual;

#[derive(Component)]
pub struct Buoyancy {
    voxels: Vec<Voxel>, // List of voxel data, possibly pulled from generate_voxel_grid
    needs_update: bool,
}

impl Buoyancy {
    fn from_voxels(voxels: Vec<Voxel>, needs_update: bool) -> Self {
        Self {
            voxels,
            needs_update,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct BuoyancyMarker;

#[derive(Debug, Clone, PartialEq)]
struct Voxel {
    position: Vec3,
    is_solid: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct AreaEnterMarker;

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct AreaExitMarker;

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct AreaName(String);

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct ColliderMarker;

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct COMMarker;

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Hideable(String);

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct NavMeshMarker;

fn hide_show_objects(
    mut commands: Commands,
    mut collision_event_reader: EventReader<Collision>,
    sensor_query: Query<(
        Entity,
        &Sensor,
        Option<&AreaEnterMarker>,
        Option<&AreaExitMarker>,
    )>,
    player_query: Query<&Player>,
    hideable_query: Query<(Entity, &Hideable, &mut Visibility)>,
) {
    for Collision(contacts) in collision_event_reader.read() {
        let entity1 = contacts.entity1;
        let entity2 = contacts.entity2;

        let player_involved =
            player_query.get(entity1).is_ok() || player_query.get(entity2).is_ok();
        if player_involved {
            let (player_entity, other_entity) = if player_query.get(entity1).is_ok() {
                (entity1, entity2)
            } else {
                (entity2, entity1)
            };

            if let Ok((sensor_entity, _, enter_marker, exit_marker)) =
                sensor_query.get(other_entity)
            {
                if enter_marker.is_some() {
                    println!(
                        "Player {:?} entered area: {:?}",
                        player_entity, sensor_entity
                    );
                    update_visibility(&mut commands, &hideable_query, Visibility::Hidden);
                } else if exit_marker.is_some() {
                    println!(
                        "Player {:?} exited area: {:?}",
                        player_entity, sensor_entity
                    );
                    update_visibility(&mut commands, &hideable_query, Visibility::Visible);
                }
            }
        }
    }
}

fn update_visibility(
    commands: &mut Commands,
    hideable_query: &Query<(Entity, &Hideable, &mut Visibility)>,
    visibility: Visibility,
) {
    for (hideable_entity, _hideable, _) in hideable_query.iter() {
        commands.entity(hideable_entity).insert(visibility);
    }
}

pub fn read_area_markers(
    enter_marker_query: Query<(Entity, &Transform), Added<AreaEnterMarker>>, // Query for AreaEnterMarker
    exit_marker_query: Query<(Entity, &Transform), Added<AreaExitMarker>>, // Query for AreaExitMarker
    mut commands: Commands,
    children: Query<&Children>,
    meshes: Res<Assets<Mesh>>,
    mesh_handles: Query<&Handle<Mesh>>,
    parent_query: Query<&Parent>, // Query to navigate up the hierarchy
    ship_query: Query<Entity, With<Ship>>, // Query for the Ship entity
) {
    // Process AreaEnterMarkers
    for (entity, transform) in enter_marker_query.iter() {
        if let Some(mesh_handle) = find_mesh(entity, &children, &mesh_handles) {
            if let Some(mesh) = meshes.get(mesh_handle) {
                if let Some(collider) = Collider::trimesh_from_mesh(mesh) {
                    // Find the top-level Ship entity
                    let mut current_parent = entity;
                    let mut ship_entity = None;
                    while let Ok(parent) = parent_query.get(current_parent) {
                        if ship_query.get(parent.get()).is_ok() {
                            ship_entity = Some(parent.get());
                            break;
                        }
                        current_parent = parent.get();
                    }

                    if let Some(ship) = ship_entity {
                        // Reparent the sensor to the Ship entity
                        commands.entity(ship).add_child(entity);

                        // Insert components for AreaEnterMarker
                        let entry_position = transform.translation; // Use the current position
                        commands.entity(entity).insert((
                            collider,
                            Sensor,
                            Transform {
                                translation: entry_position,
                                ..Default::default()
                            },
                            GlobalTransform::default(),
                            Visibility::Hidden,
                        ));
                    } else {
                        error!("No Ship entity found for the area marker");
                    }
                } else {
                    error!("Failed to create area collider from mesh");
                }
            } else {
                error!("Failed to get mesh from mesh handle");
            }
        } else {
            error!("Failed to find mesh for area collider");
        }
    }

    // Process AreaExitMarkers
    for (entity, transform) in exit_marker_query.iter() {
        if let Some(mesh_handle) = find_mesh(entity, &children, &mesh_handles) {
            if let Some(mesh) = meshes.get(mesh_handle) {
                if let Some(collider) = Collider::trimesh_from_mesh(mesh) {
                    // Find the top-level Ship entity
                    let mut current_parent = entity;
                    let mut ship_entity = None;
                    while let Ok(parent) = parent_query.get(current_parent) {
                        if ship_query.get(parent.get()).is_ok() {
                            ship_entity = Some(parent.get());
                            break;
                        }
                        current_parent = parent.get();
                    }

                    if let Some(ship) = ship_entity {
                        // Reparent the sensor to the Ship entity
                        commands.entity(ship).add_child(entity);

                        // Insert components for AreaExitMarker
                        let exit_position = transform.translation; // Use the current position
                        commands.entity(entity).insert((
                            collider,
                            Sensor,
                            Transform {
                                translation: exit_position,
                                ..Default::default()
                            },
                            GlobalTransform::default(),
                            Visibility::Hidden,
                        ));
                    } else {
                        error!("No Ship entity found for the area marker");
                    }
                } else {
                    error!("Failed to create area collider from mesh");
                }
            } else {
                error!("Failed to get mesh from mesh handle");
            }
        } else {
            error!("Failed to find mesh for area collider");
        }
    }
}

pub fn read_colliders(
    collider_marker_query: Query<
        (Entity, Option<&NavMeshMarker>, &Transform),
        Added<ColliderMarker>,
    >,
    mut commands: Commands,
    children: Query<&Children>,
    meshes: Res<Assets<Mesh>>,
    mesh_handles: Query<&Handle<Mesh>>,
    parent_query: Query<&Transform, With<Ship>>,
) {
    for (entity, nav_mesh_marker_opt, transform) in collider_marker_query.iter() {
        if let Some(mesh_handle) = find_mesh(entity, &children, &mesh_handles) {
            if let Some(mesh) = meshes.get(mesh_handle) {
                if let Some(collider) = Collider::trimesh_from_mesh(mesh) {
                    // Update transform to follow the ship if needed
                    if let Ok(ship_transform) = parent_query.get_single() {
                        commands.entity(entity).insert((
                            Transform {
                                translation: ship_transform.translation + transform.translation,
                                rotation: ship_transform.rotation * transform.rotation,
                                scale: ship_transform.scale * transform.scale,
                            },
                            GlobalTransform::default(),
                        ));
                    }

                    commands.entity(entity).insert((
                        collider,
                        RigidBody::Kinematic, // Change to Kinematic
                        Visibility::Hidden,
                    ));

                    if nav_mesh_marker_opt.is_some() {
                        commands.entity(entity).insert(NavMeshAffector);
                    }
                } else {
                    error!("Failed to create collider from mesh");
                }
            } else {
                error!("Failed to get mesh from mesh handle");
            }
        } else {
            error!("Failed to find mesh handle for collider");
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

pub fn update_voxel_solidity(
    mut query: Query<(Entity, &Transform, &mut Buoyancy)>,
    mut spatial_query: SpatialQuery,
) {
    spatial_query.update_pipeline();

    for (_entity, transform, mut buoyancy) in query.iter_mut() {
        if buoyancy.needs_update {
            for voxel in buoyancy.voxels.iter_mut() {
                let world_position = transform.translation + voxel.position;
                let voxel_collider = Collider::cuboid(VOXEL_SIZE, VOXEL_SIZE, VOXEL_SIZE);
                let intersects = spatial_query.shape_intersections(
                    &voxel_collider,
                    world_position,
                    Quat::IDENTITY,
                    SpatialQueryFilter::default(),
                );

                voxel.is_solid = !intersects.is_empty();
            }
            buoyancy.needs_update = false;
        }
    }
}

// TODO: Make this into a toggle debug system
// fn visualize_voxel_grid(
//     mut commands: Commands,
//     query: Query<(Entity, &Transform, &Buoyancy), Changed<Buoyancy>>,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     let voxel_visual_size = VOXEL_SIZE * 0.95; // Adjust size for visual gaps
//
//     for (_entity, transform, buoyancy) in query.iter() {
//         for voxel in &buoyancy.voxels {
//             if voxel.is_solid {
//                 // Transform for each voxel based on its position relative to the parent entity
//                 let voxel_position = transform.translation + voxel.position;
//
//                 // Spawn visual representation for each solid voxel
//                 commands
//                     .spawn(PbrBundle {
//                         mesh: meshes.add(Cuboid::new(voxel_visual_size, voxel_visual_size, voxel_visual_size)),
//                         material: materials.add(Color::rgb(0.5, 0.5, 1.0)), // Custom color
//                         transform: Transform::from_translation(voxel_position),
//                         ..default()
//                     })
//                     .insert(VoxelVisual {}); // Mark it visually if needed for tracking/deletion
//             }
//         }
//     }
// }

// TODO: Make this into a toggle debug system
// fn visualize_ship_bounds(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     query: Query<(Entity, &BuoyancyMarker, &Transform), Added<BuoyancyMarker>>,
//     children: Query<&Children>,
//     mesh_handles: Query<&Handle<Mesh>>,
// ) {
//     for (entity, _, _mesh_transform) in query.iter() {
//         if let Some(mesh_handle) = find_mesh(entity, &children, &mesh_handles) {
//             if let Some(mesh) = meshes.get(mesh_handle) {
//                 let bounds = calculate_mesh_bounds(mesh);
//                 visualize_bounds(&mut commands, &mut meshes, &mut materials, bounds);
//             }
//         }
//     }
// }

// TODO: Make this into a toggle debug system
// fn visualize_bounds(
//     commands: &mut Commands,
//     meshes: &mut ResMut<Assets<Mesh>>,
//     materials: &mut ResMut<Assets<StandardMaterial>>,
//     bounds: (Vec3, Vec3),
// ) {
//     let bbox_size = bounds.1 - bounds.0;
//     let bbox_position = (bounds.0 + bounds.1) * 0.5;
//
//     commands.spawn(PbrBundle {
//         mesh: meshes.add(Cuboid::new(
//             bbox_size.x,
//             bbox_size.y,
//             bbox_size.z,
//         )),
//         material: materials.add(Color::rgb(1.0, 0.0, 0.0)),
//         transform: Transform::from_translation(bbox_position),
//         ..default()
//     });
// }

fn generate_voxel_grid(mesh: &Mesh, mesh_transform: &Transform) -> Vec<Voxel> {
    let bounds = calculate_mesh_bounds(mesh);
    let grid_size = calculate_grid_size(&bounds);
    let mut voxels = Vec::new();

    for x in 0..grid_size.x {
        for y in 0..grid_size.y {
            for z in 0..grid_size.z {
                let position = Vec3::new(
                    bounds.0.x + x as f32 * VOXEL_SIZE + VOXEL_SIZE / 2.0,
                    bounds.0.y + y as f32 * VOXEL_SIZE + VOXEL_SIZE / 2.0,
                    bounds.0.z + z as f32 * VOXEL_SIZE + VOXEL_SIZE / 2.0,
                ) + mesh_transform.translation;

                voxels.push(Voxel {
                    position,
                    is_solid: false, // Solidity will be updated based on spatial queries
                });
            }
        }
    }

    voxels
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
    println!("Calculated Bounds: Min: {:?}, Max: {:?}", min, max);
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
    buoyancy_marker_query: Query<(Entity, &BuoyancyMarker, &Transform), Added<BuoyancyMarker>>,
    mut commands: Commands,
    children_query: Query<&Children>,
    parent_query: Query<&Parent>,
    ship_query: Query<Entity, With<Ship>>,
    meshes: Res<Assets<Mesh>>,
    mesh_handles: Query<&Handle<Mesh>>,
) {
    for (entity, _, mesh_transform) in buoyancy_marker_query.iter() {
        println!(
            "Processing Entity: {:?}, Transform: {:?}",
            entity, mesh_transform
        );

        if let Some(mesh_handle) = find_mesh(entity, &children_query, &mesh_handles) {
            println!("Mesh handle found: {:?}", mesh_handle);
            if let Some(mesh) = meshes.get(mesh_handle) {
                println!("Generating voxel grid for mesh.");
                let voxels = generate_voxel_grid(mesh, mesh_transform);

                // Find the top-level Ship entity
                let mut current_parent = entity;
                let mut ship_entity = None;
                while let Ok(parent) = parent_query.get(current_parent) {
                    if ship_query.get(parent.get()).is_ok() {
                        ship_entity = Some(parent.get());
                        break;
                    }
                    current_parent = parent.get();
                }

                if let Some(ship) = ship_entity {
                    // Attach the Buoyancy component and collider to the Ship entity
                    println!("Inserting collider and dynamics components to the Ship entity.");
                    if let Some(collider) = Collider::trimesh_from_mesh(mesh) {
                        commands.entity(ship).insert((
                            Buoyancy::from_voxels(voxels, true),
                            collider,
                            ColliderDensity(0.0),
                            RigidBody::Dynamic,
                            LinearDamping(0.8),
                            AngularDamping(0.8),
                            ExternalForce::new(Vec3::ZERO).with_persistence(false),
                            Visibility::Visible,
                            NavMeshAffector,
                            CenterOfMass(Vec3::new(-2.0, 0.0, 0.2)),
                            Mass(2000.0),
                            Inertia(Matrix3::from_cols(
                                Vec3::new(126395.3, -28743.2, 16967.54),
                                Vec3::new(-28743.2, 259213.7, -6361.74),
                                Vec3::new(16967.54, -6361.74, 246570.2),
                            )),
                        ));
                        commands.entity(entity).despawn_recursive();
                    }
                } else {
                    println!("No Ship entity found for the buoyancy component.");
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

fn get_water_height_at_position(pos: Vec3, water: &WaterParam) -> f32 {
    water.wave_point(pos).y
}

/// Calculates and applies the buoyancy force to gizmos submerged in water based on their density,
/// taking into account the elapsed time since the last frame.
///
/// # Arguments
///
/// * `time` - The game's time resource, providing delta time.
/// * `gizmos` - The collection of gizmos.
/// * `water` - The water parameters.
/// * `query` - The query to retrieve gizmos with buoyancy components.
///
pub fn calculate_and_apply_buoyancy(
    water: WaterParam,
    mut query: Query<(
        &Buoyancy,
        &Transform,
        &mut ExternalForce,
        &ColliderDensity,
        &CenterOfMass,
    )>,
) {
    let gravity = 9.81; // Acceleration due to gravity in m/s^2

    for (buoyancy, transform, mut external_force, _collider_density, center_of_mass) in
        query.iter_mut()
    {
        for voxel in &buoyancy.voxels {
            if voxel.is_solid {
                // Apply the ship's rotation to the voxel's position relative to the ship's center of mass
                let rotated_position = transform.rotation.mul_vec3(voxel.position);
                let world_position = transform.translation + rotated_position;

                let water_height = get_water_height_at_position(world_position, &water);
                let submerged_volume =
                    calculate_submerged_volume(world_position, water_height, VOXEL_SIZE);
                let hull_density = 1.0;
                let buoyancy_force = Vec3::new(0.0, gravity * submerged_volume * hull_density, 0.0);

                // Apply the force at the voxel's rotated position, creating torque around the center of mass
                external_force.apply_force_at_point(
                    buoyancy_force,
                    world_position,
                    center_of_mass.0,
                );

                // gizmos.sphere(center_of_mass.0, Quat::IDENTITY, 2.3, Color::RED);

                // Visualize the buoyancy force as an arrow
                // gizmos.arrow(
                //     world_position, // Start point of the arrow
                //     world_position + buoyancy_force * 0.1, // End point scaled for visibility
                //     Color::BLUE, // Color of the arrow
                // );
                // Optionally visualize the lever arm
                // gizmos.line(
                //     center_of_mass.0,
                //     world_position,
                //     Color::YELLOW,
                // );
            }
        }
    }
}

fn calculate_submerged_volume(world_position: Vec3, water_height: f32, voxel_size: f32) -> f32 {
    let bottom_of_voxel = world_position.y - voxel_size / 2.0;
    let top_of_voxel = world_position.y + voxel_size / 2.0;

    if top_of_voxel <= water_height {
        voxel_size.powi(3) // Fully submerged
    } else if bottom_of_voxel >= water_height {
        0.0 // Not submerged
    } else {
        let submerged_height = water_height - bottom_of_voxel;
        submerged_height * voxel_size * voxel_size // Partially submerged volume
    }
}
