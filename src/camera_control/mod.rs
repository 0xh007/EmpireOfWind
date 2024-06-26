use bevy::prelude::*;

use components::*;

mod components;
mod systems;
mod utils;

pub struct CameraControlPlugin;

impl Plugin for CameraControlPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CameraZoom>()
            .register_type::<DebugCamera>()
            .regiser_type::<MainCamera>();
    }
}
