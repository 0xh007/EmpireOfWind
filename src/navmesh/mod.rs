use bevy::prelude::*;
use bevy_xpbd_3d::prelude::Collider;
use oxidized_navigation::{
    debug_draw::OxidizedNavigationDebugDrawPlugin, NavMeshSettings, OxidizedNavigationPlugin,
};

pub use components::*;
pub use events::*;
use systems::*;

use crate::asset_management::states::app_states::AppStates;

mod components;
mod events;
mod systems;

/// Plugin for managing the navigation mesh within the game world.
///
/// The `NavMeshPlugin` integrates the `oxidized_navigation` crate to provide real-time
/// navigation mesh generation and debug visualization. It sets up the necessary components,
/// events, and systems to handle navigation mesh creation and visibility toggling.
///
/// # Components
/// - `NavMeshMarker`: Marks an entity that should influence the navigation mesh generation.
///
/// # Events
/// - `NavMeshDebugToggle`: Event used to toggle the visibility of the navigation mesh debug drawing.
///
/// # Systems
/// - `toggle_nav_mesh_visibility`: Toggles the visibility of the navigation mesh debug view based on events.
///
/// # Plugins
/// - `OxidizedNavigationPlugin`: Adds the core navigation mesh generation capabilities.
/// - `OxidizedNavigationDebugDrawPlugin`: Adds debug visualization for the navigation mesh.
///
/// # Resources
/// - `DrawNavMesh`: Controls the visibility of the navigation mesh debug rendering.
pub struct NavMeshPlugin;

impl Plugin for NavMeshPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<NavMeshMarker>()
            .add_event::<NavMeshDebugToggle>()
            .add_systems(
                Update,
                toggle_nav_mesh_visibility.run_if(in_state(AppStates::Running)),
            )
            .add_plugins((
                OxidizedNavigationPlugin::<Collider>::new(NavMeshSettings {
                    cell_width: 0.25,
                    cell_height: 0.125,
                    tile_width: 100,
                    world_half_extents: 250.0,
                    world_bottom_bound: -100.0,
                    max_traversable_slope_radians: (100.0_f32).to_radians(),
                    walkable_height: 10,
                    walkable_radius: 1,
                    step_height: 8,
                    min_region_area: 100,
                    merge_region_area: 500,
                    max_contour_simplification_error: 1.5,
                    max_edge_length: 80,
                    max_tile_generation_tasks: Some(9),
                }),
                OxidizedNavigationDebugDrawPlugin,
            ));
    }
}
