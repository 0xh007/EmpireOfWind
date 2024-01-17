use crate::prelude::*;
use bevy::prelude::*;
use big_brain::prelude::*;

pub struct FatiguePlugin;

impl Plugin for FatiguePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, fatigue_system).add_systems(
            PreUpdate,
            (fatigue_scorer_system).in_set(BigBrainSet::Scorers),
        );
    }
}

/// Increases an entity's fatigue over time
pub fn fatigue_system(time: Res<Time>, mut fatigues: Query<&mut Fatigue>) {
    for mut fatigue in &mut fatigues {
        fatigue.level += fatigue.per_second * time.delta_seconds();
        if fatigue.level >= 100.0 {
            fatigue.level = 100.0;
        }
        trace!("Tiredness: {}", fatigue.level);
    }
}

/// This system calculates a score based on an entity's fatigue level. The higher the fatigue, the
/// higher the score, indicating a greater need for the entity to sleep.
pub fn fatigue_scorer_system(
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
                        debug!("Fatigue above threshold! Score: {}", fatigue.level / 100.0)
                    });
                }
            }
        }
    }
}
