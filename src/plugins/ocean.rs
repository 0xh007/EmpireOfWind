use bevy::prelude::*;
use bevy_water::*;

const WATER_HEIGHT: f32 = 2.0;
const WAVE_AMPLITUDE: f32 = 2.6;

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
