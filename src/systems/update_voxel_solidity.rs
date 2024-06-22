use bevy::math::Quat;
use bevy::prelude::{Entity, Query, Transform};
use bevy_xpbd_3d::prelude::{Collider, SpatialQuery, SpatialQueryFilter};

pub fn update_voxel_solidity(
    mut query: Query<(Entity, &Transform, &mut Buoyancy)>,
    mut spatial_query: SpatialQuery,
) {
    spatial_query.update_pipeline();

    for (_entity, transform, mut buoyancy) in query.iter_mut() {
        if buoyancy.needs_update {
            for voxel in buoyancy.voxels.iter_mut() {
                let world_position = transform.translation + voxel.position;
                let voxel_collider = Collider::cuboid(crate::plugins::physics::VOXEL_SIZE, crate::plugins::physics::VOXEL_SIZE, crate::plugins::physics::VOXEL_SIZE);
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