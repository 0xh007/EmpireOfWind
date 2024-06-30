use bevy::log::{debug, trace};
use bevy::prelude::{Query, Res, Time};
use big_brain::actions::ActionState;
use big_brain::prelude::{ActionSpan, Actor};

use crate::ai_eating_behavior::components::{Eat, Hunger};

/// This system manages the eating action of entities. It reduces the hunger
/// level of the entity as it eats and updates the entity's state based on
/// the `Eat` component's parameters.
///
/// # Parameters
/// - `time`: Resource providing the delta time for the game.
/// - `hungers`: Query to fetch and modify the `Hunger` component of entities.
/// - `query`: Query to fetch and modify the `ActionState`, `Eat`, and `ActionSpan`
///   components of entities, as well as the `Actor` component to identify the entity.
pub fn eat_action(
    time: Res<Time>,
    mut hungers: Query<&mut Hunger>,
    mut query: Query<(&Actor, &mut ActionState, &Eat, &ActionSpan)>,
) {
    for (Actor(actor), mut state, eat, span) in &mut query {
        let _guard = span.span().enter();

        if let Ok(mut hunger) = hungers.get_mut(*actor) {
            match *state {
                ActionState::Requested => {
                    debug!("Time to eat!");
                    hunger.is_eating = true;
                    *state = ActionState::Executing;
                }
                ActionState::Executing => {
                    trace!("Eating...");
                    hunger.level -= hunger.per_second * time.delta_seconds();

                    if hunger.level <= eat.until {
                        debug!("No longer hungry!");
                        hunger.is_eating = false;
                        *state = ActionState::Success;
                    }
                }
                // All actions should make sure to handle cancellations
                ActionState::Cancelled => {
                    hunger.is_eating = false;
                    *state = ActionState::Failure;
                }
                _ => {}
            }
        }
    }
}
