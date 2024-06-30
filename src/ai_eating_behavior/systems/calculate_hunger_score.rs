use bevy::log::trace;
use bevy::prelude::{Local, Query, With};
use big_brain::prelude::{Actor, Score, ScorerSpan};

use crate::ai_eating_behavior::components::HungerScorer;
use crate::ai_eating_behavior::Hunger;

/// This system calculates a score based on an entity's hunger level. The higher the hunger,
/// the higher the score, indicating a greater need for the entity to eat.
///
/// The system iterates over entities with the `HungerScorer` component, fetches their `Hunger`
/// component, and updates their `Score` component. If the entity is eating, the score remains
/// unchanged. Otherwise, the score is updated based on the hunger level.
///
/// # Parameters
/// - `last_score`: A local cache to store the last calculated score for eating entities.
/// - `hungers`: A query to fetch the `Hunger` component of entities.
/// - `query`: A query to fetch the `Actor`, `Score`, and `ScorerSpan` components of entities
///   with the `HungerScorer` component.
pub fn calculate_hunger_score(
    mut last_score: Local<Option<f32>>,
    hungers: Query<&Hunger>,
    mut query: Query<(&Actor, &mut Score, &ScorerSpan), With<HungerScorer>>,
) {
    for (Actor(actor), mut score, span) in &mut query {
        if let Ok(hunger) = hungers.get(*actor) {
            let new_score = hunger.level / 100.0;

            if hunger.is_eating {
                let _score = last_score.get_or_insert(new_score);
                score.set(*_score);
            } else {
                last_score.take();
                score.set(new_score);
                if hunger.level >= 100.0 {
                    span.span().in_scope(|| {
                        trace!("Hunger above threshold! Score: {}", hunger.level / 100.0)
                    });
                }
            }
        }
    }
}
