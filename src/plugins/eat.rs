use bevy::prelude::*;

use crate::prelude::*;
use crate::systems::eat_action;

pub struct EatPlugin;

impl Plugin for EatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, eat_action::eat_action);
    }
}
