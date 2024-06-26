use bevy::prelude::*;

use components::*;
use systems::*;

use crate::AppStates;

mod components;
mod systems;

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Food>()
            .add_systems(Update, spawn_food.run_if(in_state(AppStates::Running)));
    }
}