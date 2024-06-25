use bevy::asset::{Assets, Handle};
use bevy::hierarchy::{Children, Parent};
use bevy::log::error;
use bevy::prelude::*;
use bevy_xpbd_3d::prelude::{Collider, Sensor};

use crate::area_management::components::*;

/// System to process and configure area markers for entry and exit points.
///
/// This system handles the `AreaEnterMarker` and `AreaExitMarker` components,
/// configuring the respective entities to act as sensors for entering and exiting
/// specific areas. It attaches colliders and re-parents the markers to the top-level
/// ship entity, ensuring that they are correctly integrated into the ship's hierarchy.
///
/// # Parameters
/// - `enter_marker_query`: Query to retrieve entities with `AreaEnterMarker` components and their transforms.
/// - `exit_marker_query`: Query to retrieve entities with `AreaExitMarker` components and their transforms.
/// - `commands`: Commands for modifying entities and their components.
/// - `children`: Query to retrieve the children of entities.
/// - `meshes`: Resource containing the assets of meshes.
/// - `mesh_handles`: Query to retrieve mesh handles from entities.
/// - `parent_query`: Query to navigate up the hierarchy to find parent entities.
/// - `ship_query`: Query to identify the top-level ship entity.
///
/// # Details
/// For each `AreaEnterMarker` and `AreaExitMarker`, the system:
/// - Finds the associated mesh and creates a collider from it.
/// - Navigates up the entity hierarchy to find the top-level ship entity.
/// - Re-parents the marker entity to the ship entity.
/// - Configures the marker entity with the collider, sensor, transform, and other necessary components.
#[allow(clippy::too_many_arguments)]
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
                        // Re-parent the sensor to the Ship entity
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