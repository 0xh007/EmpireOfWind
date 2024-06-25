use bevy::math::Vec3;
use bevy::prelude::{Mesh, Transform};

use crate::prelude::*;

/// Generates a voxel grid within the bounds of the given mesh.
///
/// This function calculates the bounding box of the mesh and fills it with a voxel grid,
/// determining the center position of each voxel and initializing it as non-solid.
/// The solidity of each voxel will be updated based on spatial queries in subsequent steps.
///
/// # Arguments
///
/// * `mesh` - A reference to the mesh to be voxelized.
/// * `mesh_transform` - The transform of the mesh in the game world.
///
/// # Returns
///
/// A vector of `Voxel` structs representing the voxel grid.
pub fn generate_voxel_grid(mesh: &Mesh, mesh_transform: &Transform) -> Vec<Voxel> {
    let bounds = calculate_mesh_bounds(mesh);
    let grid_size = calculate_grid_size(&bounds);
    let mut voxels = Vec::new();

    for x in 0..grid_size.x {
        for y in 0..grid_size.y {
            for z in 0..grid_size.z {
                let position = Vec3::new(
                    bounds.0.x + x as f32 * VOXEL_SIZE + VOXEL_SIZE / 2.0,
                    bounds.0.y + y as f32 * VOXEL_SIZE + VOXEL_SIZE / 2.0,
                    bounds.0.z + z as f32 * VOXEL_SIZE + VOXEL_SIZE / 2.0,
                ) + mesh_transform.translation;

                voxels.push(Voxel {
                    position,
                    is_solid: false,
                });
            }
        }
    }

    voxels
}

/// Calculates the size of the voxel grid based on the mesh bounds.
///
/// This function computes the dimensions of the bounding box of the mesh and
/// determines the number of voxels along each axis, ensuring that the grid size
/// fully encompasses the mesh.
///
/// # Arguments
///
/// * `bounds` - A tuple containing the minimum and maximum coordinates of the bounding box.
///
/// # Returns
///
/// A `Vec3I` struct representing the number of voxels along each axis.
fn calculate_grid_size(bounds: &(Vec3, Vec3)) -> Vec3I {
    let (min, max) = bounds;
    let size = *max - *min;

    Vec3I::new(
        (size.x / VOXEL_SIZE).ceil() as i32,
        (size.y / VOXEL_SIZE).ceil() as i32,
        (size.z / VOXEL_SIZE).ceil() as i32,
    )
}

/// Calculates the submerged volume of a voxel based on its position and water height.
///
/// This function determines the volume of a voxel that is submerged under water,
/// taking into account the voxel's size and its position relative to the water height.
///
/// # Arguments
///
/// * `world_position` - The position of the voxel in the game world.
/// * `water_height` - The height of the water surface.
/// * `voxel_size` - The size of the voxel.
///
/// # Returns
///
/// A `f32` value representing the submerged volume of the voxel.
pub fn calculate_submerged_volume(world_position: Vec3, water_height: f32, voxel_size: f32) -> f32 {
    let bottom_of_voxel = world_position.y - voxel_size / 2.0;
    let top_of_voxel = world_position.y + voxel_size / 2.0;

    if top_of_voxel <= water_height {
        voxel_size.powi(3) // Fully submerged
    } else if bottom_of_voxel >= water_height {
        0.0 // Not submerged
    } else {
        let submerged_height = water_height - bottom_of_voxel;
        submerged_height * voxel_size * voxel_size // Partially submerged volume
    }
}
