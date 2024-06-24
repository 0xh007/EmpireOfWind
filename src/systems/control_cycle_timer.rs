use bevy::input::ButtonInput;
use bevy::prelude::{KeyCode, Res, ResMut};

use crate::constants::day_cycle::DAY_CYCLE_SPEED_DELTA;
use crate::resources::cycle_timer::CycleTimer;

/// Controls the cycle timer based on user input.
///
/// - Press `P` to toggle the pause state of the cycle timer.
/// - Press `NumpadAdd` to increase the speed of the cycle timer.
/// - Press `NumpadSubtract` to decrease the speed of the cycle timer.
///
/// # Parameters
/// - `input`: Resource that captures keyboard input.
/// - `timer`: Mutable resource that manages the cycle timer.
///
/// # Example Usage
/// The `control_cycle_timer` system should be added to your Bevy app like this:
/// ```rust
/// use bevy::prelude::*;
/// use empire_of_wind::systems::control_cycle_timer;
/// use empire_of_wind::resources::cycle_timer::CycleTimer;
///
/// fn main() {
///     App::new()
///         .add_plugins(DefaultPlugins)
///         .insert_resource(CycleTimer::default())
///         .add_systems(Update, control_cycle_timer)
///         .run();
/// }
/// ```
pub fn control_cycle_timer(input: Res<ButtonInput<KeyCode>>, mut timer: ResMut<CycleTimer>) {
    if input.just_pressed(KeyCode::KeyP) {
        timer.toggle_pause();
    }

    if input.pressed(KeyCode::NumpadAdd) {
        timer.update_speed(DAY_CYCLE_SPEED_DELTA);
        eprintln!("Increase speed: {}", timer.speed);
    }

    if input.pressed(KeyCode::NumpadSubtract) {
        timer.update_speed(-DAY_CYCLE_SPEED_DELTA);
        eprintln!("Decrease speed: {}", timer.speed);
    }
}
