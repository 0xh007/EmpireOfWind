use bevy::prelude::*;

pub use components::*;

mod components;
mod constants;
mod systems;

pub struct AiNavigationPlugin;

impl Plugin for AiNavigationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MoveToNearestFood>()
            .register_type::<MoveToNearestSleepArea>();
    }
}