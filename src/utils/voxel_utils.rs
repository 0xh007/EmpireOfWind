use bevy::math::Vec3;
use bevy::prelude::{Mesh, Transform};

use crate::components::{
    vec3i::Vec3I,
    voxel::Voxel,
};
use crate::constants::voxel::VOXEL_SIZE;
use crate::utils::mesh_utils::calculate_mesh_bounds;

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
                    is_solid: false, // Solidity will be updated based on spatial queries
                });
            }
        }
    }

    voxels
}

fn calculate_grid_size(bounds: &(Vec3, Vec3)) -> Vec3I {
    let (min, max) = bounds;
    let size = *max - *min;

    Vec3I::new(
        (size.x / VOXEL_SIZE).ceil() as i32,
        (size.y / VOXEL_SIZE).ceil() as i32,
        (size.z / VOXEL_SIZE).ceil() as i32,
    )
}

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
