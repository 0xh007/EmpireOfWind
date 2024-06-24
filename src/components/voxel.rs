use bevy::prelude::Vec3;

/// Represents a single voxel in the game world.
///
/// A `Voxel` is a cubic unit of space in the game world, defined by its position
/// and a boolean indicating whether it is solid. Voxels are used in various systems
/// such as buoyancy and collision detection.
///
/// # Fields
/// - `position`: The 3D position of the voxel.
/// - `is_solid`: A boolean indicating whether the voxel is solid.
///
/// # Usage
///
/// ## Example: Creating and Using a Voxel
///
/// ```rust
/// use bevy::prelude::Vec3;
/// use empire_of_wind::components::Voxel;
///
/// // Creating a new Voxel
/// let voxel = Voxel {
///     position: Vec3::new(1.0, 2.0, 3.0),
///     is_solid: true,
/// };
///
/// // Accessing the Voxel's fields
/// println!("Voxel position: {:?}", voxel.position);
/// println!("Is solid: {}", voxel.is_solid);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Voxel {
    pub position: Vec3,
    pub is_solid: bool,
}
