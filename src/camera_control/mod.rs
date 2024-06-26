use bevy::prelude::*;

use components::*;
use systems::*;

use crate::AppStates;

mod components;
mod systems;
mod utils;

/// Plugin for managing camera controls within the game.
///
/// The `CameraControlPlugin` provides functionality for switching between different
/// cameras, moving the camera based on player input, and setting up the initial camera
/// configuration. It registers the necessary components and sets up systems to handle
/// camera operations.
///
/// # Components
/// - `CameraZoom`: Represents the zoom level of a camera.
/// - `DebugCamera`: Marks a camera as a debug camera.
/// - `MainCamera`: Marks the main camera used for gameplay.
///
/// # Systems
/// - `camera_switching`: Handles switching between different cameras.
/// - `move_camera`: Manages camera movement based on player input.
/// - `setup_camera`: Sets up the initial configuration for cameras.
///
/// This plugin is added to the app during the application setup and is configured to
/// operate during the `AppStates::Running` state.
pub struct CameraControlPlugin;

impl Plugin for CameraControlPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CameraZoom>()
            .register_type::<DebugCamera>()
            .register_type::<MainCamera>()
            .add_systems(Update, camera_switching.run_if(in_state(AppStates::Running)))
            .add_systems(Update, move_camera.run_if(in_state(AppStates::Running)))
            .add_systems(Update, setup_camera.run_if(in_state(AppStates::Running)))
            .add_systems(Update, interpolate_zoom.run_if(in_state(AppStates::Running)));
    }
}
