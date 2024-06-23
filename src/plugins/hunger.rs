use crate::prelude::*;
use bevy::prelude::*;
use big_brain::prelude::*;
use crate::systems::{calculate_hunger_score, increase_hunger};

pub struct HungerPlugin;

impl Plugin for HungerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Hunger>()
            .add_systems(Update, increase_hunger::increase_hunger)
            .add_systems(
                PreUpdate,
                (calculate_hunger_score::calculate_hunger_score).in_set(BigBrainSet::Scorers),
            );
    }
}
