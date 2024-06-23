use oxidized_navigation::{NavMesh, NavMeshSettings};
use bevy::math::Vec3;
use oxidized_navigation::query::{find_polygon_path, perform_string_pulling_on_path};

fn find_navigation_path(
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
