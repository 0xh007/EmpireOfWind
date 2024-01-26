use crate::prelude::*;
use bevy::prelude::*;
use big_brain::prelude::*;

pub struct HungerPlugin;

impl Plugin for HungerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Hunger>()
            .add_systems(Update, hunger_system)
            .add_systems(
                PreUpdate,
                (hunger_scorer_system).in_set(BigBrainSet::Scorers),
            );
    }
}

/// Increases an entity's hunger over time
pub fn hunger_system(time: Res<Time>, mut hungers: Query<&mut Hunger>) {
    for mut hunger in &mut hungers {
        hunger.level += hunger.per_second * time.delta_seconds();
        if hunger.level >= 100.0 {
            hunger.level = 100.0;
        }
        trace!("Hunger: {}", hunger.level);
    }
}

/// This system calculates a score based on an entity's hunger level. The higher the hunger, the
/// higher the score, indicating a greater need for the entity to eat.
pub fn hunger_scorer_system(
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
