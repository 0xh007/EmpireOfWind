use bevy::prelude::Component;

/// Marker component for identifying voxel visual entities in the game.
///
/// The `VoxelVisual` component is used to tag entities that represent visual
/// representations of voxels. This component does not have any fields and is purely
/// used for identification purposes by various systems that need to interact with
/// voxel visuals.
#[derive(Component)]
pub struct VoxelVisual;
