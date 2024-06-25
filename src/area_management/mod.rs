use bevy::prelude::*;

use components::*;
use resources::*;
use systems::*;

use crate::AppStates;

pub mod components;
pub mod resources;
pub mod systems;

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
            .register_type::<AreaEnterMarker>()
            .register_type::<AreaExitMarker>()
            .register_type::<AreaName>()
            .insert_resource(ActiveAreas::default())
            .add_systems(Update, manage_active_areas.run_if(in_state(AppStates::Running)))
            .add_systems(Update, read_area_markers.run_if(in_state(AppStates::Running)));
    }
}
