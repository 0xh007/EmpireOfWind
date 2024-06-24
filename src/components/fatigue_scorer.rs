use bevy::prelude::*;
use big_brain::prelude::*;

/// This component serves as a scorer for evaluating the entity's need to sleep based on its
/// fatigue level.
///
/// The `FatigueScorer` component is used to determine the priority of the sleeping action for
/// an entity by calculating a score that reflects the entity's current fatigue level.
///
/// # Usage
///
/// ## Example 1: Initializing an Entity with a FatigueScorer
///
/// ```rust
/// use bevy::prelude::*;
/// use big_brain::prelude::*;
/// use empire_of_wind::components::FatigueScorer;
///
/// fn spawn_entity(mut commands: Commands) {
///     commands.spawn((
///         FatigueScorer,
///         // Other components...
///     ));
/// }
/// ```
///
/// ## Example 2: Calculating Fatigue Score in a System
///
/// ```rust
/// use bevy::prelude::{Local, Query, With};
/// use big_brain::prelude::{Actor, Score, ScorerSpan};
/// use empire_of_wind::components::{Fatigue, FatigueScorer};
///
/// fn calculate_fatigue_score(
///     mut last_score: Local<Option<f32>>,
///     fatigues: Query<&Fatigue>,
///     mut query: Query<(&Actor, &mut Score, &ScorerSpan), With<FatigueScorer>>,
/// ) {
///     for (Actor(actor), mut score, span) in &mut query {
///         if let Ok(fatigue) = fatigues.get(*actor) {
///             let new_score = fatigue.level / 100.0;
///
///             if fatigue.is_sleeping {
///                 let _score = last_score.get_or_insert(new_score);
///                 score.set(*_score);
///             } else {
///                 last_score.take();
///                 score.set(new_score);
///             }
///         }
///     }
/// }
/// ```
#[derive(Clone, Component, Debug, ScorerBuilder)]
pub struct FatigueScorer;
