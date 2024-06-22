use bevy::prelude::*;
use big_brain::prelude::*;

use crate::prelude::*;
use crate::systems::{calculate_fatigue_score, increase_fatigue};

pub struct FatiguePlugin;

impl Plugin for FatiguePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, increase_fatigue::increase_fatigue).add_systems(
            PreUpdate,
            (calculate_fatigue_score::calculate_fatigue_score).in_set(BigBrainSet::Scorers),
        );
    }
}
