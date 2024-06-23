use bevy::asset::{Assets, Handle};
use bevy::hierarchy::{Children, DespawnRecursiveExt, Parent};
use bevy::math::Vec3;
use bevy::prelude::{Added, Commands, Entity, Mesh, Query, Res, Transform, Visibility, With};
use bevy_xpbd_3d::components::{AngularDamping, CenterOfMass, ColliderDensity, ExternalForce, Inertia, LinearDamping, Mass, RigidBody};
use bevy_xpbd_3d::math::Matrix3;
use bevy_xpbd_3d::prelude::Collider;
use oxidized_navigation::NavMeshAffector;

use crate::components::Buoyancy;
use crate::components::buoyancy_marker::BuoyancyMarker;
use crate::components::ship::Ship;

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
