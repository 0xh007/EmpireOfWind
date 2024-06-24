use bevy::math::Vec3;
use bevy::prelude::{Query, Transform};
use bevy_water::WaterParam;
use bevy_xpbd_3d::components::{CenterOfMass, ColliderDensity, ExternalForce};

use crate::components::Buoyancy;
use crate::constants::voxel::VOXEL_SIZE;
use crate::utils::*;

/// This system calculates and applies buoyancy forces to entities with the `Buoyancy` component.
///
/// The system iterates over all entities with the `Buoyancy`, `Transform`, `ExternalForce`,
/// `ColliderDensity`, and `CenterOfMass` components. For each voxel in the `Buoyancy` component,
/// it calculates the buoyancy force based on the submerged volume and applies this force to
/// the entity at the voxel's position, taking into account the entity's rotation and center of mass.
///
/// # Arguments
///
/// * `water` - A parameter containing the global water settings and time resource, used to calculate wave heights.
/// * `query` - A query that retrieves entities with the required components for buoyancy calculation.
///
/// # Details
///
/// The buoyancy force is computed using the following formula:
///
/// buoyancy force = gravity * submerged volume * hull density
///
/// The system also applies the calculated buoyancy force at the voxel's rotated position, creating torque around the center of mass.

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

                // TODO: Make this toggleable
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
