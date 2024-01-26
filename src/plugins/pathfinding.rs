use crate::prelude::*;
use bevy::prelude::*;
use big_brain::prelude::*;
use oxidized_navigation::{
    query::{find_polygon_path, perform_string_pulling_on_path},
    NavMesh, NavMeshSettings,
};

const MAX_DISTANCE: f32 = 1.0;

pub struct PathfindingPlugin;

impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<NavigationPath>().add_systems(
            PreUpdate,
            (
                move_to_nearest_system::<SleepArea>,
                move_to_nearest_system::<Food>,
            ),
        );
    }
}

#[allow(clippy::type_complexity)]
fn move_to_nearest_system<T: Component + std::fmt::Debug + Clone>(
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
                        if let Some(new_path) = calculate_path_blocking(
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

fn calculate_path_blocking(
    nav_mesh: &NavMesh,
    nav_mesh_settings: &NavMeshSettings,
    start: Vec3,
    goal: Vec3,
) -> Option<Vec<Vec3>> {
    // Lock the nav_mesh for reading
    let nav_mesh_lock = nav_mesh.get();
    let nav_mesh = nav_mesh_lock.read().unwrap();

    // Find the polygon path using the navigation mesh
    let polygon_path = match find_polygon_path(
        &nav_mesh,
        nav_mesh_settings,
        start,
        goal,
        None, // You can specify options here if needed
        Some(&[
            nav_mesh_settings.walkable_radius.into(),
            nav_mesh_settings.walkable_height.into(),
        ]),
    ) {
        Ok(path) => path,
        Err(_) => return None,
    };

    // Perform string pulling on the polygon path to get a Vec3 path
    match perform_string_pulling_on_path(&nav_mesh, start, goal, &polygon_path) {
        Ok(string_path) => Some(string_path),
        Err(_) => None,
    }
}
