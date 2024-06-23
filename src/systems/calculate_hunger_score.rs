use bevy::prelude::{Local, Query, With};
use big_brain::prelude::{Actor, Score, ScorerSpan};
use bevy::log::trace;
use crate::components::{Hunger, HungerScorer};

/// This system calculates a score based on an entity's hunger level. The higher the hunger, the
/// higher the score, indicating a greater need for the entity to eat.
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
