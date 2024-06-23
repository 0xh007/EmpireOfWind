use bevy::prelude::{Component, Query, Res, Time, Transform, With, Without};
use oxidized_navigation::{NavMesh, NavMeshSettings};
use big_brain::prelude::{ActionSpan, Actor, HasThinker};
use big_brain::actions::ActionState;
use bevy::log::debug;
use crate::components::{MoveToNearest, NavigationPath};
use crate::plugins::pathfinding;
use crate::plugins::pathfinding::MAX_DISTANCE;

#[allow(clippy::type_complexity)]
pub fn navigate_to_nearest<T: Component + std::fmt::Debug + Clone>(
    time: Res<Time>,
    nav_mesh: Res<NavMesh>,
    nav_mesh_settings: Res<NavMeshSettings>,
    goal_query: Query<&Transform, With<T>>,
    mut thinker_query: Query<(&mut NavigationPath, &mut Transform), (With<HasThinker>, Without<T>)>,
    mut action_query: Query<(&Actor, &mut ActionState, &MoveToNearest<T>, &ActionSpan)>,
) {
    for (Actor(actor), mut action_state, move_to, span) in &mut action_query {
        let _guard = span.span().enter();

        if let Ok((mut navigation_path, mut actor_transform)) = thinker_query.get_mut(*actor) {
            match *action_state {
                ActionState::Requested => {
                    debug!("Lets go find a {:?}", std::any::type_name::<T>());
                    *action_state = ActionState::Executing;
                }
                ActionState::Executing => {
                    let goal_transform = goal_query
                        .iter()
                        .map(|t| (t.translation, t))
                        .min_by(|(a, _), (b, _)| {
                            a.distance_squared(actor_transform.translation)
                                .partial_cmp(&b.distance_squared(actor_transform.translation))
                                .unwrap()
                        })
                        .map(|(_, t)| t);

                    if navigation_path.points.is_empty() {
                        if let Some(new_path) = pathfinding::calculate_path_blocking(
                            &nav_mesh,
                            &nav_mesh_settings,
                            actor_transform.translation,
                            goal_transform.unwrap().translation,
                        ) {
                            debug!("Updating navgation path.");
                            navigation_path.points = new_path;
                        } else {
                            *action_state = ActionState::Failure;
                            continue;
                        }
                    }

                    // Check if we have a path to follow
                    if let Some(next_point) = navigation_path.points.first() {
                        let direction = (*next_point - actor_transform.translation).normalize();
                        let distance_to_next_point =
                            (*next_point - actor_transform.translation).length();

                        // Check if we are close enough to the next point to consider it reached
                        if distance_to_next_point < MAX_DISTANCE {
                            // Remove the reached point from the navigation path
                            navigation_path.points.remove(0);

                            // If after removing the point, the path is empty, we've reached the
                            // end
                            if navigation_path.points.is_empty() {
                                debug!("Reached end of path.");
                                *action_state = ActionState::Success;
                                continue;
                            }
                        } else {
                            // Move towards the next point
                            let step_size = time.delta_seconds() * move_to.speed;
                            let step = direction * step_size.min(distance_to_next_point);
                            actor_transform.translation += step;
                        }
                    }
                }
                ActionState::Cancelled => {
                    debug!("Moving to is cancelled.");
                    *action_state = ActionState::Failure;
                }
                _ => {}
            }
        }
    }
}
