use bevy::math::Vec3;
use bevy::prelude::{Query, Transform};
use bevy_water::WaterParam;
use bevy_xpbd_3d::components::{CenterOfMass, ColliderDensity, ExternalForce};

use crate::components::Buoyancy;
use crate::plugins::physics;
use crate::plugins::physics::VOXEL_SIZE;
use crate::utils::water_utils;

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

                let water_height = water_utils::get_water_height_at_position(world_position, &water);
                let submerged_volume =
                    physics::calculate_submerged_volume(world_position, water_height, VOXEL_SIZE);
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
