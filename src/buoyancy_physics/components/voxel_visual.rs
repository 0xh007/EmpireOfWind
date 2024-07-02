use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Marker component for identifying voxel visual entities in the game.
///
/// The `VoxelVisual` component is used to tag entities that represent visual
/// representations of voxels. This component does not have any fields and is purely
/// used for identification purposes by various systems that need to interact with
/// voxel visuals.
#[derive(Debug, Clone, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct VoxelVisual;
