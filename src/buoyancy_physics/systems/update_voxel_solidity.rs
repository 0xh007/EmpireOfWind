use bevy::math::Quat;
use bevy::prelude::{Entity, Query, Transform};
use bevy_xpbd_3d::prelude::{Collider, SpatialQuery, SpatialQueryFilter};

use crate::buoyancy_physics::Buoyancy;
use crate::buoyancy_physics::constants::VOXEL_SIZE;

/// Updates the solidity status of voxels for buoyancy calculations.
///
/// This system checks each voxel associated with entities that have a `Buoyancy` component
/// and updates their solidity status based on spatial intersections. The system uses
/// the `SpatialQuery` resource to determine if each voxel intersects with any colliders in the world,
/// which is crucial for calculating buoyancy of floating objects such as ships.
///
/// # Parameters
///
/// * `query`: A Query to retrieve entities with their `Transform` and `Buoyancy` components.
/// * `spatial_query`: A mutable reference to the `SpatialQuery` resource used for checking spatial intersections.
///
/// # Behavior
///
/// For each entity with a `Buoyancy` component that needs an update, the system iterates over its voxels and:
///
/// 1. Computes the world position of each voxel based on the entity's transform.
/// 2. Creates a cuboid collider representing the voxel.
/// 3. Checks for intersections between the voxel collider and other colliders in the world using the `SpatialQuery` resource.
/// 4. Updates the `is_solid` status of the voxel based on whether any intersections were found.
/// 5. Marks the `Buoyancy` component as updated to avoid redundant calculations.
///
/// This system ensures that the buoyancy calculations accurately reflect the current state of the game world,
/// taking into account any changes in the position or state of colliders.
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
