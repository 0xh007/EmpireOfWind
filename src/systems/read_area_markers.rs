use bevy::asset::{Assets, Handle};
use bevy::hierarchy::{Children, Parent};
use bevy::log::error;
use bevy::prelude::{Added, Commands, Entity, GlobalTransform, Mesh, Query, Res, Transform, Visibility, With};
use bevy_xpbd_3d::prelude::{Collider, Sensor};

use crate::components::{AreaEnterMarker, AreaExitMarker};
use crate::plugins::Ship;

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
        if let Some(mesh_handle) = crate::plugins::physics::find_mesh(entity, &children, &mesh_handles) {
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
        if let Some(mesh_handle) = crate::plugins::physics::find_mesh(entity, &children, &mesh_handles) {
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