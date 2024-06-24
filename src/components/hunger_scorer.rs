use bevy::prelude::*;
use big_brain::prelude::*;

/// This component serves as a scorer for evaluating the entity's need to eat based on its hunger
/// level.
///
/// The `HungerScorer` component is used to calculate a score that indicates the urgency for an
/// entity to perform the eating action. A higher score means a higher need to eat.
///
/// # Usage
///
/// ## Example 1: Initializing an Entity with HungerScorer
///
/// ```rust
/// use bevy::prelude::*;
/// use empire_of_wind::components::HungerScorer;
///
/// fn spawn_entity(mut commands: Commands) {
///     commands.spawn((
///         HungerScorer,
///         // Other components...
///     ));
/// }
/// ```
///
/// ## Example 2: Calculating Hunger Score in a System
///
/// ```rust
/// use bevy::prelude::{Local, Query, With};
/// use big_brain::prelude::{Actor, Score, ScorerSpan};
/// use bevy::log::trace;
/// use empire_of_wind::components::{Hunger, HungerScorer};
///
/// fn calculate_hunger_score(
///     mut last_score: Local<Option<f32>>,
///     hungers: Query<&Hunger>,
///     mut query: Query<(&Actor, &mut Score, &ScorerSpan), With<HungerScorer>>,
/// ) {
///     for (Actor(actor), mut score, span) in &mut query {
///         if let Ok(hunger) = hungers.get(*actor) {
///             let new_score = hunger.level / 100.0;
///
///             if hunger.is_eating {
///                 let _score = last_score.get_or_insert(new_score);
///                 score.set(*_score);
///             } else {
///                 last_score.take();
///                 score.set(new_score);
///                 if hunger.level >= 100.0 {
///                     span.span().in_scope(|| {
///                         trace!("Hunger above threshold! Score: {}", hunger.level / 100.0)
///                     });
///                 }
///             }
///         }
///     }
/// }
/// ```
#[derive(Clone, Component, Debug, ScorerBuilder)]
pub struct HungerScorer;
