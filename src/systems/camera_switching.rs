use bevy::input::ButtonInput;
use bevy::prelude::{Camera, KeyCode, Query, Res, Without};

use crate::components::{DebugCamera, MainCamera};

pub fn camera_switching(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Camera, &DebugCamera), Without<MainCamera>>,
    mut query_main: Query<(&mut Camera, &MainCamera), Without<DebugCamera>>,
) {
    if keyboard_input.just_pressed(KeyCode::Digit0) {
        for (mut camera, _) in query.iter_mut() {
            camera.is_active = !camera.is_active;
        }

        for (mut camera, _) in query_main.iter_mut() {
            camera.is_active = !camera.is_active;
        }
    }
}
