use bevy::prelude::*;

pub use components::*;
pub use resources::*;
use systems::*;

use crate::asset_management::states::app_states::AppStates;

mod components;
mod resources;
mod systems;
mod utils;

// TODO: Update Docs
/// Plugin for managing areas within the game world.
///
/// The `AreaManagementPlugin` provides functionality for handling areas that
/// players can enter and exit. It is largely concerned with modifying the visibility of certain areas
/// based on the players location. For example, if the player walks into a room, this plugin provides
/// the ability to hide and show walls as needed. It registers the necessary components, initializes
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
pub struct AreaManagementPlugin;

impl Plugin for AreaManagementPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<AreaMarker>()
            .register_type::<AreaExitMarker>()
            .register_type::<Occluding>()
            .register_type::<LayerSet>()
            // TODO: Move this into a less specific plugin
            .register_type::<Vec<String>>()
            // TODO: Move this into a less specific plugin
            .register_type::<String>()
            // TODO: Move this into a less specific plugin
            .register_type::<Vec<u8>>()
            .insert_resource(ActiveAreas::default())
            .add_systems(
                Update,
                manage_active_areas.run_if(in_state(AppStates::Running)),
            )
            .add_systems(
                Update,
                read_area_markers.run_if(in_state(AppStates::Running)),
            )
            .add_systems(
                Update,
                propagate_render_layers.run_if(in_state(AppStates::Running)),
            );
    }
}
