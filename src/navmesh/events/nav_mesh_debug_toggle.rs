use bevy::prelude::*;

/// An event sent for toggling the visibility of the navigation mesh debug drawing.
///
/// This event is used to enable or disable the debug visualization of the navigation mesh
/// within the game. It can be triggered by various systems, typically in response to user input.
#[derive(Event)]
pub struct NavMeshDebugToggle;
