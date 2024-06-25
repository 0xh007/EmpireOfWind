use bevy::prelude::Component;

use crate::components::voxel::Voxel;

/// The `Buoyancy` component represents the buoyant properties of an entity in the game.
/// It contains a list of voxels that represent the volume of the entity and a flag indicating
/// whether the voxel data needs to be updated.
///
/// # Fields
/// - `voxels`: A vector of `Voxel` instances that represent the voxelized volume of the entity.
/// - `needs_update`: A boolean flag indicating whether the voxel data requires updating.
///
/// # Methods
/// - `from_voxels(voxels: Vec<Voxel>, needs_update: bool) -> Self`:
///   Creates a new `Buoyancy` component from a list of voxels and an update flag.
#[derive(Component)]
pub struct Buoyancy {
    pub voxels: Vec<Voxel>, // List of voxel data, possibly pulled from generate_voxel_grid
    pub needs_update: bool,
}

impl Buoyancy {
    /// Creates a new `Buoyancy` component from a list of voxels and an update flag.
    pub fn from_voxels(voxels: Vec<Voxel>, needs_update: bool) -> Self {
        Self {
            voxels,
            needs_update,
        }
    }
}
