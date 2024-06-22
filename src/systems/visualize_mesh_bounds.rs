// TODO: Make this into a toggle debug system
fn visualize_mesh_bounds(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &BuoyancyMarker, &Transform), Added<BuoyancyMarker>>,
    children: Query<&Children>,
    mesh_handles: Query<&Handle<Mesh>>,
) {
    for (entity, _, _mesh_transform) in query.iter() {
        if let Some(mesh_handle) = find_mesh(entity, &children, &mesh_handles) {
            if let Some(mesh) = meshes.get(mesh_handle) {
                let bounds = calculate_mesh_bounds(mesh);
                visualize_bounds(&mut commands, &mut meshes, &mut materials, bounds);
            }
        }
    }
}