use bevy::asset::{Assets, Handle};
use bevy::hierarchy::{Children, DespawnRecursiveExt, Parent};
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy_xpbd_3d::components::{
    AngularDamping, CenterOfMass, ColliderDensity, ExternalForce, Inertia, LinearDamping, Mass,
    RigidBody,
};
use bevy_xpbd_3d::math::Matrix3;
use bevy_xpbd_3d::prelude::Collider;
use oxidized_navigation::NavMeshAffector;

use crate::buoyancy_physics::{Buoyancy, BuoyancyMarker};
use crate::buoyancy_physics::utils::generate_voxel_grid;
use crate::ship::Ship;
use crate::utils::find_mesh;

/// System to process and configure buoyancy objects within the game.
///
/// This system handles entities marked with the `BuoyancyMarker` component, generating
/// voxel grids for buoyancy calculations and attaching necessary components to the
/// top-level ship entity. The system ensures that buoyancy objects are correctly integrated
/// into the ship's dynamics, allowing for realistic buoyancy and physics interactions.
///
/// # Parameters
/// - `buoyancy_marker_query`: Query to retrieve entities with `BuoyancyMarker` components and their transforms.
/// - `commands`: Commands for modifying entities and their components.
/// - `children_query`: Query to retrieve the children of entities.
/// - `parent_query`: Query to navigate up the hierarchy to find parent entities.
/// - `ship_query`: Query to identify the top-level ship entity.
/// - `meshes`: Resource containing the assets of meshes.
/// - `mesh_handles`: Query to retrieve mesh handles from entities.
///
/// # Details
/// For each `BuoyancyMarker` entity, the system:
/// - Finds the associated mesh and generates a voxel grid for buoyancy calculations.
/// - Navigates up the entity hierarchy to find the top-level ship entity.
/// - Attaches the `Buoyancy` component and other physics-related components to the ship entity.
/// - De-spawns the original marker entity after processing.
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
                            RigidBody::Kinematic,
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
