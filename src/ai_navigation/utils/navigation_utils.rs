use bevy::math::Vec3;
use oxidized_navigation::query::{find_polygon_path, perform_string_pulling_on_path};
use oxidized_navigation::{NavMesh, NavMeshSettings};

/// Finds a navigation path from the start position to the goal position using the navigation mesh.
///
/// This function integrates with the `oxidized_navigation` crate to perform pathfinding
/// on a navigation mesh. It first finds the polygon path on the navigation mesh and then
/// performs string pulling to convert the polygon path into a path of `Vec3` positions.
///
/// # Parameters
/// - `nav_mesh`: Reference to the `NavMesh` object used for pathfinding.
/// - `nav_mesh_settings`: Reference to the `NavMeshSettings` object containing settings for the navigation mesh.
/// - `start`: The starting position (`Vec3`) for the path.
/// - `goal`: The goal position (`Vec3`) for the path.
///
/// # Returns
/// An `Option<Vec<Vec3>>` which is:
/// - `Some(Vec<Vec3>)` containing the path from start to goal if the pathfinding is successful.
/// - `None` if the pathfinding fails.
pub fn find_navigation_path(
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
