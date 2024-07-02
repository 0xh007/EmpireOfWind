use bevy::asset::{Assets, Handle};
use bevy::hierarchy::Children;
use bevy::log::error;
use bevy::prelude::{
    Added, Commands, Entity, GlobalTransform, Mesh, Query, Res, Transform, Visibility, With,
};
use bevy_xpbd_3d::components::RigidBody;
use bevy_xpbd_3d::prelude::Collider;
use oxidized_navigation::NavMeshAffector;

use crate::collider_management::ColliderMarker;
use crate::navmesh::NavMeshMarker;
use crate::ship::Ship;
use crate::utils::find_mesh;

/// System to process and configure collider objects within the game.
///
/// This system handles entities marked with the `ColliderMarker` component, generating
/// colliders from associated meshes and attaching necessary components. If the entity is
/// also marked with `NavMeshMarker`, it will additionally be configured as an affector of the
/// navigation mesh. The system ensures colliders are correctly integrated into the ship's
/// hierarchy and physics system.
///
/// # Parameters
/// - `collider_marker_query`: Query to retrieve entities with `ColliderMarker` components, optional `NavMeshMarker`, and their transforms.
/// - `commands`: Commands for modifying entities and their components.
/// - `children`: Query to retrieve the children of entities.
/// - `meshes`: Resource containing the assets of meshes.
/// - `mesh_handles`: Query to retrieve mesh handles from entities.
/// - `parent_query`: Query to retrieve the transform of the ship entity.
///
/// # Details
/// For each `ColliderMarker` entity, the system:
/// - Finds the associated mesh and generates a collider from it.
/// - Updates the entity's transform to follow the ship if necessary.
/// - Attaches the `Collider` and `RigidBody::Kinematic` components.
/// - Hides the entity's visibility.
/// - If marked with `NavMeshMarker`, attaches the `NavMeshAffector` component.
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
        println!("QUERY WORKS");
        if let Some(mesh_handle) = find_mesh(entity, &children, &mesh_handles) {
            println!("MESH HANDLE");
            if let Some(mesh) = meshes.get(mesh_handle) {
                println!("FOUND MESH");
                if let Some(collider) = Collider::trimesh_from_mesh(mesh) {
                    println!("MAKING COLLIDER");
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
