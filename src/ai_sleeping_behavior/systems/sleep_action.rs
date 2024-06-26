use bevy::prelude::{Query, Res, Time};
use big_brain::actions::ActionState;
use big_brain::prelude::{ActionSpan, Actor};

use crate::components::{Fatigue, Sleep};

/// System that manages the sleeping action of entities.
///
/// This system reduces the fatigue level of an entity while it is sleeping and updates the entity's
/// state based on the `Sleep` component's parameters.
///
/// # Parameters
/// - `time`: A reference to the `Time` resource to get the elapsed time since the last update.
/// - `fatigues`: A query to get the `Fatigue` components of the entities.
/// - `query`: A query to get the `Actor`, `ActionState`, `Sleep`, and `ActionSpan` components of the entities.
pub fn sleep_action(
    time: Res<Time>,
    mut fatigues: Query<&mut Fatigue>,
    mut query: Query<(&Actor, &mut ActionState, &Sleep, &ActionSpan)>,
) {
    for (Actor(actor), mut state, sleep, span) in &mut query {
        let _guard = span.span().enter();

        // Use the sleep_action's actor to look up the corresponding Fatigue component.
        if let Ok(mut fatigue) = fatigues.get_mut(*actor) {
            match *state {
                ActionState::Requested => {
                    fatigue.is_sleeping = true;
                    *state = ActionState::Executing;
                }
                ActionState::Executing => {
                    fatigue.level -= sleep.per_second * time.delta_seconds();

                    if fatigue.level <= sleep.until {
                        fatigue.is_sleeping = false;
                        *state = ActionState::Success;
                    }
                }
                // All actions should make sure to handle cancellations
                ActionState::Cancelled => {
                    fatigue.is_sleeping = false;
                    *state = ActionState::Failure;
                }
                _ => {}
            }
        }
    }
}
