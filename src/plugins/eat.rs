use crate::prelude::*;
use bevy::prelude::*;
use big_brain::prelude::*;

pub struct EatPlugin;

impl Plugin for EatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, eat_action_system);
    }
}

fn eat_action_system(
    time: Res<Time>,
    mut hungers: Query<&mut Hunger>,
    mut query: Query<(&Actor, &mut ActionState, &Sleep, &ActionSpan)>,
) {
}
