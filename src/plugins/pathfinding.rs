use crate::prelude::*;
use bevy::prelude::*;
use big_brain::prelude::*;
use oxidized_navigation::{NavMesh, NavMeshSettings};

const MAX_DISTANCE: f32 = 0.1;

pub struct PathfindingPlugin;

impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AsyncPathfindingTasks::default())
            .add_systems(PreUpdate, move_to_nearest_system::<SleepArea>)
            .add_systems(Update, async_pathfinding_system)
            .add_systems(Update, poll_pathfinding_tasks_system);
    }
}

fn async_pathfinding_system(
    nav_mesh_settings: Res<NavMeshSettings>,
    nav_mesh: Res<NavMesh>,
    mut pathfinding_task: ResMut<AsyncPathfindingTasks>,
) {
}

fn poll_pathfinding_tasks_system() {}

fn move_to_nearest_system<T: Component + std::fmt::Debug + Clone>(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<T>>,
    mut thinkers: Query<&mut Transform, (With<HasThinker>, Without<T>)>,
    mut action_query: Query<(&Actor, &mut ActionState, &MoveToNearest<T>, &ActionSpan)>,
) {
    for (actor, mut action_state, move_to, span) in &mut action_query {
        let _guard = span.span().enter();

        match *action_state {
            ActionState::Requested => {
                debug!("Let's go find a {:?}", std::any::type_name::<T>());

                *action_state = ActionState::Executing;
            }
            ActionState::Executing => {
                let mut actor_transform = thinkers.get_mut(actor.0).unwrap();
                // The goal is the nearest entity with the specified component.
                let goal_transform = query
                    .iter_mut()
                    .map(|t| (t.translation, t))
                    .min_by(|(a, _), (b, _)| {
                        // We need partial_cmp here because f32 doesn't implement Ord.
                        let delta_a = *a - actor_transform.translation;
                        let delta_b = *b - actor_transform.translation;
                        delta_a.length().partial_cmp(&delta_b.length()).unwrap()
                    })
                    .unwrap()
                    .1;
                let delta = goal_transform.translation - actor_transform.translation;
                let distance = delta.xz().length();

                trace!("Distance: {}", distance);

                if distance > MAX_DISTANCE {
                    trace!("Stepping closer.");

                    let step_size = time.delta_seconds() * move_to.speed;
                    let step = delta.normalize() * step_size.min(distance);

                    // We only care about moving in the XZ plane.
                    actor_transform.translation.x += step.x;
                    actor_transform.translation.z += step.z;
                } else {
                    debug!("We got there!");

                    *action_state = ActionState::Success;
                }
            }
            ActionState::Cancelled => {
                debug!("Moving to is cancelled");
                *action_state = ActionState::Failure;
            }
            _ => {}
        }
    }
}
