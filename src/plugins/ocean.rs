use bevy::prelude::*;
use bevy_water::*;

use crate::prelude::*;

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
            .add_plugins(WaterPlugin)
            .add_systems(Update, update_water_interactables);
    }
}

fn update_water_interactables(
    water: WaterParam,
    mut water_interactables: Query<(&WaterInteractable, &mut Transform, &GlobalTransform)>,
    #[cfg(feature = "debug")] mut lines: ResMut<DebugLines>,
) {
    for (water_interactable, mut transform, global) in water_interactables.iter_mut() {
        let pos = global.translation();
        #[cfg(not(feature = "debug"))]
        water_interactable.sync_with_water(&water, pos, &mut transform);
    }
}
