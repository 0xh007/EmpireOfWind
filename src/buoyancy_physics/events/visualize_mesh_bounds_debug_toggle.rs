use bevy::prelude::*;

/// Event to toggle the visualization of mesh bounds for debugging purposes.
///
/// This event is used to enable or disable the debug visualization of mesh bounds within the game.
/// It is triggered by a specific keyboard input and processed by the `visualize_mesh_bounds` system
/// to render or hide the bounding boxes of meshes, helping developers understand the voxelization process.
///
/// # Usage
/// - Triggered by a keyboard input (e.g., pressing the `8` key).
/// - Listened to by the `visualize_mesh_bounds` system to toggle mesh bounds visualization.
#[derive(Event)]
pub struct VisualizeMeshBoundsDebugToggle;
