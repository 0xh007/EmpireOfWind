use bevy::prelude::*;

use {components::*, systems::*};

use crate::AppStates;

mod components;
mod systems;

/// Plugin for managing AI sleeping behavior within the game.
///
/// The AiSleepingBehaviorPlugin provides functionality for handling the fatigue and sleeping
/// behaviors of entities. It registers the necessary components, initializes resources,
/// and sets up systems to manage fatigue scoring, sleep actions, and fatigue increments.
///
/// # Components
/// - Fatigue: Tracks the fatigue level of an entity, indicating whether it is currently sleeping and how quickly
///   fatigue increases.
/// - FatigueScorer: Calculates a score indicating the urgency for an entity to sleep.
/// - Sleep: Manages the sleeping action of an entity, defining when it stops sleeping and how quickly
///   fatigue decreases.
///
/// # Systems
/// - calculate_fatigue_score: Calculates a score based on an entity's fatigue level.
/// - increase_fatigue: Increases an entity's fatigue over time.
/// - sleep_action: Manages the sleeping action of entities, reducing their fatigue level.
pub struct AiSleepingBehaviorPlugin;

impl Plugin for AiSleepingBehaviorPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Fatigue>()
            .register_type::<FatigueScorer>()
            .register_type::<Sleep>()
            .add_systems(
                Update,
                calculate_fatigue_score.run_if(in_state(AppStates::Running)),
            )
            .add_systems(
                Update,
                increase_fatigue.run_if(in_state(AppStates::Running)),
            )
            .add_systems(Update, sleep_action.run_if(in_state(AppStates::Running)));
    }
}
