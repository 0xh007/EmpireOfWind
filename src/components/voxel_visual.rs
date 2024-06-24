use bevy::prelude::Component;

/// Marker component for identifying voxel visual entities in the game.
///
/// The `VoxelVisual` component is used to tag entities that represent visual
/// representations of voxels. This component does not have any fields and is purely
/// used for identification purposes by various systems that need to interact with
/// voxel visuals.
///
/// # Usage
///
/// ## Example: Adding the VoxelVisual Component to an Entity
///
/// ```rust
/// use bevy::prelude::*;
/// use empire_of_wind::components::VoxelVisual;
///
/// fn spawn_voxel_visual(mut commands: Commands) {
///     commands.spawn(VoxelVisual);
/// }
/// ```
#[derive(Component)]
pub struct VoxelVisual;
