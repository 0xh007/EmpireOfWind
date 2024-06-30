use bevy::prelude::*;

pub use components::*;
use systems::*;

use crate::AppStates;

mod components;
mod constants;
mod systems;
mod utils;

/// Plugin for managing buoyancy physics within the game.
///
/// The `BuoyancyPhysicsPlugin` provides functionality for handling buoyancy calculations
/// and visualizing voxel grids and mesh bounds. It registers the necessary components,
/// initializes resources, and sets up systems to manage and apply buoyancy forces to
/// objects in the game world.
///
/// # Components
/// - `Buoyancy`: Represents the buoyancy properties of an object.
/// - `BuoyancyMarker`: Marks an object to be processed for buoyancy calculations.
/// - `Vec3I`: Represents a 3D vector with integer components.
/// - `Voxel`: Represents a single voxel within the voxel grid.
/// - `VoxelVisual`: Represents the visual properties of a voxel.
///
/// # Systems
/// - `calculate_and_apply_buoyancy`: Calculates and applies buoyancy forces to marked objects.
/// - `read_buoyancy_objects`: Reads and processes objects marked for buoyancy calculations.
/// - `update_voxel_solidity`: Updates the solidity state of voxels based on game state.
/// - `visualize_mesh_bounds`: Visualizes the bounds of the mesh for debugging and tuning.
/// - `visualize_voxel_grid`: Visualizes the voxel grid for debugging and tuning.
///
/// This plugin is added to the app during the application setup and is configured to
/// operate during the `AppStates::Running` state.

pub struct BuoyancyPhysicsPlugin;

impl Plugin for BuoyancyPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Buoyancy>()
            .register_type::<BuoyancyMarker>()
            .register_type::<Voxel>()
            .register_type::<VoxelVisual>()
            .add_systems(
                Update,
                calculate_and_apply_buoyancy.run_if(in_state(AppStates::Running)),
            )
            .add_systems(
                Update,
                read_buoyancy_objects.run_if(in_state(AppStates::Running)),
            )
            .add_systems(
                Update,
                update_voxel_solidity.run_if(in_state(AppStates::Running)),
            )
            .add_systems(
                Update,
                visualize_mesh_bounds.run_if(in_state(AppStates::Running)),
            )
            .add_systems(
                Update,
                visualize_voxel_grid.run_if(in_state(AppStates::Running)),
            );
    }
}
