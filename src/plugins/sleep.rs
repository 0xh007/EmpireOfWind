use crate::prelude::*;
use bevy::prelude::*;
use big_brain::prelude::*;

const DEFAULT_COLOR: Color = Color::YELLOW;
const SLEEP_COLOR: Color = Color::RED;

pub struct SleepPlugin;

impl Plugin for SleepPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, sleep_action_system);
    }
}

/// This system manages the sleeping action of entities. It reduces the fatigue
/// level of the entity as it sleeps and updates the entity's state based on
/// the Sleep component's parameters.
fn sleep_action_system(
    time: Res<Time>,
    mut fatigues: Query<(&mut Fatigue, &Handle<StandardMaterial>)>,
    // Resource used to modify the appearance of the farmer.
    mut materials: ResMut<Assets<StandardMaterial>>,
    // We execute actions by querying for their associated Action Component
    // (Sleep in this case). You'll always need both Actor and ActionState.
    mut query: Query<(&Actor, &mut ActionState, &Sleep, &ActionSpan)>,
) {
    for (Actor(actor), mut state, sleep, span) in &mut query {
        // This sets up the tracing scope. Any `debug` calls here will be
        // spanned together in the output.
        let _guard = span.span().enter();

        // Use the sleep_action's actor to look up the corresponding Fatigue component.
        if let Ok((mut fatigue, material)) = fatigues.get_mut(*actor) {
            match *state {
                ActionState::Requested => {
                    debug!("Time to sleep!");
                    fatigue.is_sleeping = true;
                    *state = ActionState::Executing;
                }
                ActionState::Executing => {
                    trace!("Sleeping...");
                    fatigue.level -= sleep.per_second * time.delta_seconds();
                    materials.get_mut(material).unwrap().base_color = SLEEP_COLOR;

                    if fatigue.level <= sleep.until {
                        debug!("Woke up well-rested");
                        materials.get_mut(material).unwrap().base_color = DEFAULT_COLOR;
                        fatigue.is_sleeping = false;
                        *state = ActionState::Success;
                    }
                }
                // All actions should make sure to handle cancellations
                ActionState::Cancelled => {
                    debug!("Sleep was interrupted. Still tired.");
                    materials.get_mut(material).unwrap().base_color = DEFAULT_COLOR;
                    fatigue.is_sleeping = false;
                    *state = ActionState::Failure;
                }
                _ => {}
            }
        }
    }
}
