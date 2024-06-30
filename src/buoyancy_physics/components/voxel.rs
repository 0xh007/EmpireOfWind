use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Represents a single voxel in the game world.
///
/// A `Voxel` is a cubic unit of space in the game world, defined by its position
/// and a boolean indicating whether it is solid. Voxels are used in various systems
/// such as buoyancy and collision detection.
///
/// # Fields
/// - `position`: The 3D position of the voxel.
/// - `is_solid`: A boolean indicating whether the voxel is solid.
#[derive(Debug, Clone, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Voxel {
    pub position: Vec3,
    pub is_solid: bool,
}
