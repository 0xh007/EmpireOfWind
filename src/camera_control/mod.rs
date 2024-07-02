use bevy::prelude::*;
use bevy::transform::TransformSystem;
use bevy_xpbd_3d::PhysicsSet;

pub use components::*;
use systems::*;

use crate::asset_management::states::app_states::AppStates;

mod components;
mod systems;

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
            .add_systems(
                Update,
                camera_switching.run_if(in_state(AppStates::Running)),
            )
            .add_systems(
                PostUpdate,
                move_camera
                    .run_if(in_state(AppStates::Running))
                    .after(PhysicsSet::Sync)
                    .before(TransformSystem::TransformPropagate),
            )
            .add_systems(OnEnter(AppStates::Running), setup_camera)
            .add_systems(
                Update,
                interpolate_zoom.run_if(in_state(AppStates::Running)),
            );
    }
}
