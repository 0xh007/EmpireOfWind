use crate::prelude::*;
use bevy::{prelude::*, reflect::Reflect};
use big_brain::prelude::*;
use oxidized_navigation::{
    query::{find_path, find_polygon_path, perform_string_pulling_on_path},
    NavMesh, NavMeshSettings,
};

const MAX_DISTANCE: f32 = 0.1;

pub struct PathfindingPlugin;

impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AsyncPathfindingTasks::default())
            .register_type::<NavigationPath>()
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
    nav_mesh: Res<NavMesh>,
    nav_mesh_settings: Res<NavMeshSettings>,
    mut goal_query: Query<&mut Transform, (With<T>, Without<Actor>)>,
    mut actor_query: Query<(
        &Actor,
        &mut ActionState,
        &ActionSpan,
        &MoveToNearest<T>,
        &mut NavigationPath,
        &Transform,
    )>,
) {
    for (
        actor,
        mut action_state,
        action_span,
        move_to_nearest,
        mut navigation_path,
        actor_transform,
    ) in &mut actor_query
    {
        debug!("Query at least worked");
        let _guard = action_span.span().enter();

        match *action_state {
            ActionState::Requested => {
                debug!("Let's go find a {:?}", std::any::type_name::<T>());

                *action_state = ActionState::Executing;
            }

            ActionState::Executing => {
                // TODO: implement a function for goal_has_moved
                let goal_has_moved = false;

                let goal_transform = goal_query
                    .iter()
                    .map(|t| (t.translation, t))
                    .min_by(|(a, _), (b, _)| {
                        // We need partial_cmp here because f32 doesn't implement Ord.
                        let delta_a = *a - actor_transform.translation;
                        let delta_b = *b - actor_transform.translation;
                        delta_a.length().partial_cmp(&delta_b.length()).unwrap()
                    })
                    .unwrap()
                    .1;

                if !navigation_path.points.is_empty() {
                    debug!("Nav path not empty?");
                }

                if navigation_path.points.is_empty() || goal_has_moved {
                    if let Some(new_path) = calculate_path_blocking(
                        &nav_mesh,
                        &nav_mesh_settings,
                        actor_transform.translation,
                        goal_transform.translation,
                    ) {
                        debug!("Updating navgation path.");
                        navigation_path.points = new_path;
                    } else {
                        *action_state = ActionState::Failure;
                        continue;
                    }
                }

                // TODO: Move along the path
            }

            ActionState::Cancelled => {
                debug!("Moving to is cancelled");
                *action_state = ActionState::Failure;
            }
            _ => {}
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

// fn move_to_nearest_system<T: Component + std::fmt::Debug + Clone>(
//     time: Res<Time>,
//     mut query: Query<&mut Transform, With<T>>,
//     mut thinkers: Query<&mut Transform, (With<HasThinker>, Without<T>)>,
//     mut action_query: Query<(&Actor, &mut ActionState, &MoveToNearest<T>, &ActionSpan)>,
// ) {
//     for (actor, mut action_state, move_to, span) in &mut action_query {
//         let _guard = span.span().enter();
//
//         match *action_state {
//             ActionState::Requested => {
//                 debug!("Let's go find a {:?}", std::any::type_name::<T>());
//
//                 *action_state = ActionState::Executing;
//             }
//             ActionState::Executing => {
//                 let mut actor_transform = thinkers.get_mut(actor.0).unwrap();
//                 // The goal is the nearest entity with the specified component.
//                 let goal_transform = query
//                     .iter_mut()
//                     .map(|t| (t.translation, t))
//                     .min_by(|(a, _), (b, _)| {
//                         // We need partial_cmp here because f32 doesn't implement Ord.
//                         let delta_a = *a - actor_transform.translation;
//                         let delta_b = *b - actor_transform.translation;
//                         delta_a.length().partial_cmp(&delta_b.length()).unwrap()
//                     })
//                     .unwrap()
//                     .1;
//                 let delta = goal_transform.translation - actor_transform.translation;
//                 let distance = delta.xz().length();
//
//                 trace!("Distance: {}", distance);
//
//                 if distance > MAX_DISTANCE {
//                     trace!("Stepping closer.");
//
//                     let step_size = time.delta_seconds() * move_to.speed;
//                     let step = delta.normalize() * step_size.min(distance);
//
//                     // We only care about moving in the XZ plane.
//                     actor_transform.translation.x += step.x;
//                     actor_transform.translation.z += step.z;
//                 } else {
//                     debug!("We got there!");
//
//                     *action_state = ActionState::Success;
//                 }
//             }
//             ActionState::Cancelled => {
//                 debug!("Moving to is cancelled");
//                 *action_state = ActionState::Failure;
//             }
//             _ => {}
//         }
//     }
// }
