use bevy::input::ButtonInput;
use bevy::prelude::{KeyCode, Res, ResMut};

use crate::sun::consts::SUN_CYCLE_SPEED_DELTA;
use crate::sun::resources::SunCycleTimer;

/// Controls the cycle timer based on user input.
///
/// - Press `P` to toggle the pause state of the sun cycle timer.
/// - Press `NumpadAdd` to increase the speed of the sun cycle timer.
/// - Press `NumpadSubtract` to decrease the speed of the sun cycle timer.
///
/// # Parameters
/// - `input`: Resource that captures keyboard input.
/// - `timer`: Mutable resource that manages the sun cycle timer.
pub fn control_sun_cycle_timer(input: Res<ButtonInput<KeyCode>>, mut timer: ResMut<SunCycleTimer>) {
    if input.just_pressed(KeyCode::KeyP) {
        timer.toggle_pause();
    }

    if input.pressed(KeyCode::NumpadAdd) {
        timer.update_speed(SUN_CYCLE_SPEED_DELTA);
        eprintln!("Increase speed: {}", timer.speed);
    }

    if input.pressed(KeyCode::NumpadSubtract) {
        timer.update_speed(-SUN_CYCLE_SPEED_DELTA);
        eprintln!("Decrease speed: {}", timer.speed);
    }
}
