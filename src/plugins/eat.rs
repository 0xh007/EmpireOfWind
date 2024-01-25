use crate::prelude::*;
use bevy::prelude::*;
use big_brain::prelude::*;

pub struct EatPlugin;

impl Plugin for EatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, eat_action_system);
    }
}

/// This system manages the eating action of entities. It reduces the fatigue
/// level of the entity as it eats and updates the entity's state based on
/// the Eat component's parameters.
fn eat_action_system(
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
