use bevy::prelude::*;

use components::*;
use systems::*;

use crate::AppStates;

mod components;
mod constants;
mod systems;
mod utils;

/// Plugin for managing areas within the game world.
///
/// The `AreaManagementPlugin` provides functionality for handling areas that
/// players can enter and exit. It registers the necessary components, initializes
/// resources, and sets up systems to manage the active areas and process area markers.
///
/// # Components
/// - `AreaEnterMarker`: Marks an area that a player can enter.
/// - `AreaExitMarker`: Marks an area that a player can exit.
/// - `AreaName`: Assigns a name to a specific area.
///
/// # Resources
/// - `ActiveAreas`: Tracks the currently active areas in the game.
///
/// # Systems
/// - `manage_active_areas`: Manages active areas based on player interactions.
/// - `read_area_markers`: Processes and configures area markers for entry and exit points.
///
/// This plugin is added to the app during the application setup and is configured to
/// operate during the `AppStates::Running` state.
pub struct AreaManagementPlugin;

impl Plugin for AreaManagementPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Buoyancy>()
            .register_type::<BuoyancyMarker>()
            .register_type::<Vec3I>()
            .register_type::<Voxel>()
            .register_type::<VoxelVisual>()
            .add_systems(Update, calculate_and_apply_buoyancy.run_if(in_state(AppStates::Running)))
            .add_systems(Update, read_buoyancy_objects.run_if(in_state(AppStates::Running)))
            .add_systems(Update, update_voxel_solidity.run_if(in_state(AppStates::Running)))
            .add_systems(Update, visualize_mesh_bounds.run_if(in_state(AppStates::Running)))
            .add_systems(Update, visualize_voxel_grid.run_if(in_state(AppStates::Running)));
    }
}
