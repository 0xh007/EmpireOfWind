use std::time::Duration;

use bevy::app::{App, Plugin};
use bevy::prelude::*;

use resources::*;
use systems::*;

use crate::AppStates;

mod consts;
mod resources;
mod systems;

/// Plugin for managing the sun's movement throughout the day.
///
/// The `SunCyclePlugin` provides functionality for handling the day/night cycle
/// by moving the sun's position and adjusting the lighting. It registers the necessary
/// resources and sets up systems to control and update the sun cycle based on the
/// elapsed time and user input.
///
/// # Resources
/// - `SunCycleTimer`: Manages the timing and speed of the day/night cycle.
///
/// # Systems
/// - `control_sun_cycle_timer`: Handles user input to control the sun cycle timer (pause/unpause, adjust speed).
/// - `update_sun_cycle`: Updates the sun's position and lighting based on the cycle timer.
///
/// This plugin is added to the app during the application setup and is configured to
/// operate during the `AppStates::Running` state.
pub struct SunCyclePlugin;

impl Plugin for SunCyclePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SunCycleTimer::new(Duration::from_millis(1000), 0.2))
            .add_systems(Update, control_sun_cycle_timer.run_if(in_state(AppStates::Running)))
            .add_systems(Update, update_sun_cycle.run_if(in_state(AppStates::Running)));
    }
}