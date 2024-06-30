use bevy::log::trace;
use bevy::prelude::{Local, Query, With};
use big_brain::prelude::{Actor, Score, ScorerSpan};

use crate::ai_sleeping_behavior::{Fatigue, FatigueScorer};

/// This system calculates a score based on an entity's fatigue level. The higher the fatigue,
/// the higher the score, indicating a greater need for the entity to sleep.
///
/// The system iterates over entities with the `FatigueScorer` component, fetches their `Fatigue`
/// component, and updates their `Score` component. If the entity is sleeping, the score remains
/// unchanged. Otherwise, the score is updated based on the fatigue level.
///
/// # Parameters
/// - `last_score`: A local cache to store the last calculated score for sleeping entities.
/// - `fatigues`: A query to fetch the `Fatigue` component of entities.
/// - `query`: A query to fetch the `Actor`, `Score`, and `ScorerSpan` components of entities
///   with the `FatigueScorer` component.
pub fn calculate_fatigue_score(
    mut last_score: Local<Option<f32>>,
    fatigues: Query<&Fatigue>,
    mut query: Query<(&Actor, &mut Score, &ScorerSpan), With<FatigueScorer>>,
) {
    for (Actor(actor), mut score, span) in &mut query {
        if let Ok(fatigue) = fatigues.get(*actor) {
            let new_score = fatigue.level / 100.0;

            if fatigue.is_sleeping {
                let _score = last_score.get_or_insert(new_score);
                score.set(*_score);
            } else {
                last_score.take();
                score.set(new_score);
                if fatigue.level >= 80.0 {
                    span.span().in_scope(|| {
                        trace!("Fatigue above threshold! Score: {}", fatigue.level / 100.0)
                    });
                }
            }
        }
    }
}
