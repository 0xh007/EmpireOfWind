use bevy::prelude::*;

use crate::prelude::*;
use crate::systems::sleep_action;

pub struct SleepPlugin;

impl Plugin for SleepPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, sleep_action::sleep_action);
    }
}