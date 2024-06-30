use bevy::prelude::*;

pub use components::*;
use systems::*;

use crate::AppStates;

mod components;
mod systems;

/// Plugin for managing AI eating behavior within the game.
///
/// The AiEatingBehaviorPlugin provides functionality for handling the hunger and eating
/// behaviors of entities. It registers the necessary components, initializes resources,
/// and sets up systems to manage hunger scoring, eating actions, and hunger increments.
///
/// # Components
/// - Eat: Manages the eating action of an entity, defining when it stops eating and how quickly
///   hunger decreases.
/// - Hunger: Tracks the hunger level of an entity, including whether it is eating and how quickly
///   hunger increases.
/// - HungerScorer: Calculates a score indicating the urgency for an entity to eat.
///
/// # Systems
/// - calculate_hunger_score: Calculates a score based on an entity's hunger level.
/// - eat_action: Manages the eating action of entities, reducing their hunger level.
/// - increase_hunger: Increases an entity's hunger level over time.
pub struct AiEatingBehaviorPlugin;

impl Plugin for AiEatingBehaviorPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Eat>()
            .register_type::<Hunger>()
            .register_type::<HungerScorer>()
            .add_systems(
                Update,
                calculate_hunger_score.run_if(in_state(AppStates::Running)),
            )
            .add_systems(Update, eat_action.run_if(in_state(AppStates::Running)))
            .add_systems(Update, increase_hunger.run_if(in_state(AppStates::Running)));
    }
}
