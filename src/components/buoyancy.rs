use bevy::prelude::Component;

use crate::components::voxel::Voxel;

#[derive(Component)]
pub struct Buoyancy {
    pub voxels: Vec<Voxel>, // List of voxel data, possibly pulled from generate_voxel_grid
    pub needs_update: bool,
}

impl Buoyancy {
    pub fn from_voxels(voxels: Vec<Voxel>, needs_update: bool) -> Self {
        Self {
            voxels,
            needs_update,
        }
    }
}