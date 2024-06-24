use bevy::input::ButtonInput;
use bevy::prelude::{Camera, KeyCode, Query, Res, Without};

use crate::components::{DebugCamera, MainCamera};

/// This system switches the active camera between the main camera and the debug camera
/// when the `0` key is pressed.
///
/// The system toggles the `is_active` state of both the main camera and the debug camera.
/// When the `0` key is pressed, the active camera is switched, allowing for quick toggling
/// between different camera perspectives for debugging or gameplay purposes.
///
/// # Parameters
/// - `keyboard_input`: A resource that provides the current state of the keyboard inputs.
/// - `query`: A query to fetch the `Camera` and `DebugCamera` components of entities
///   that are not the main camera.
/// - `query_main`: A query to fetch the `Camera` and `MainCamera` components of entities
///   that are not the debug camera.
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
