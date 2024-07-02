use bevy::prelude::*;

/// Event to toggle the visualization of voxel grid for debugging purposes.
///
/// This event is used to enable or disable the debug visualization of the voxel grid within the game.
/// It is triggered by a specific keyboard input and processed by the `visualize_voxel_grid` system
/// to render or hide the voxel grid, helping developers understand the voxelization and buoyancy calculations.
///
/// # Usage
/// - Triggered by a keyboard input (e.g., pressing the `7` key).
/// - Listened to by the `visualize_voxel_grid` system to toggle voxel grid visualization.
#[derive(Event)]
pub struct VisualizeVoxelsDebugToggle;
