use bevy::prelude::*;
use bevy_water::*;

use crate::ocean::consts::{WATER_HEIGHT, WAVE_AMPLITUDE};

mod consts;

/// Plugin for managing the ocean and water effects within the game world.
///
/// The `OceanPlugin` integrates the `bevy_water` crate to provide realistic water
/// and ocean wave effects. It sets up the necessary resources and plugins to simulate
/// the ocean's appearance and behavior.
///
/// # Resources
/// - `WaterSettings`: Configures the water properties such as height and wave amplitude.
///
/// # Plugins
/// - `WaterPlugin`: Adds the core water simulation capabilities from the `bevy_water` crate.
///
/// This plugin is added to the app during the application setup and is configured to
/// operate during the `AppStates::Running` state.
pub struct OceanPlugin;

impl Plugin for OceanPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WaterSettings {
            height: WATER_HEIGHT,
            amplitude: WAVE_AMPLITUDE,
            ..default()
        })
        .add_plugins(WaterPlugin);
    }
}
