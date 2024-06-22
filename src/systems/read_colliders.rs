use bevy::asset::{Assets, Handle};
use bevy::hierarchy::Children;
use bevy::log::error;
use bevy::prelude::{Added, Commands, Entity, GlobalTransform, Mesh, Query, Res, Transform, Visibility, With};
use bevy_xpbd_3d::components::RigidBody;
use bevy_xpbd_3d::prelude::Collider;
use oxidized_navigation::NavMeshAffector;

use crate::plugins::Ship;

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