use bevy::log::trace;
use bevy::prelude::{Local, Query, With};
use big_brain::prelude::{Actor, Score, ScorerSpan};

use crate::components::{Fatigue, FatigueScorer};

/// This system calculates a score based on an entity's fatigue level. The higher the fatigue, the
/// higher the score, indicating a greater need for the entity to sleep.
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
