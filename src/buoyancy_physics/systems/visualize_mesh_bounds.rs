use bevy::prelude::*;

use crate::buoyancy_physics::{BuoyancyMarker, VisualizeMeshBoundsDebugToggle};
use crate::utils::{calculate_mesh_bounds, find_mesh, visualize_bounds};

/// Visualizes the bounding box of meshes for debugging purposes.
///
/// This system is used for visual debugging by drawing the outer bounds of meshes. It helps
/// in understanding the voxelization process by showing the initial step of finding the
/// mesh boundaries. The system identifies entities with the `BuoyancyMarker` component,
/// calculates their mesh bounds, and visualizes these bounds in the game world.
///
/// Note: This system is intended to be made toggleable in the future for better control over
/// debugging visuals.
///
/// # Parameters
///
/// * `visualize_mesh_bounds_debug_event_reader`: Event reader to determine if we should run the system.
/// * `commands`: The Commands resource is used to spawn and configure entities for visualizing the bounds.
/// * `meshes`: A mutable reference to the Assets resource containing Mesh objects.
/// * `materials`: A mutable reference to the Assets resource containing StandardMaterial objects.
/// * `query`: A Query to retrieve entities with the `BuoyancyMarker` component and their Transforms that were recently added.
/// * `children`: A Query to retrieve child entities of a given parent entity.
/// * `mesh_handles`: A Query to retrieve the mesh handle associated with an entity.
///
/// # Behavior
///
/// For each entity with an `Added<BuoyancyMarker>`:
///
/// 1. The system finds the mesh handle associated with the entity.
/// 2. It retrieves the mesh and calculates its outer bounds.
/// 3. It visualizes these bounds by spawning entities that represent the bounding box using PBR (Physically Based Rendering) components.
///
/// This visualization helps developers to see the initial step of the voxelization process, where the mesh bounds are determined.
pub fn visualize_mesh_bounds(
    mut visualize_mesh_bounds_debug_event_reader: EventReader<VisualizeMeshBoundsDebugToggle>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &BuoyancyMarker, &Transform), Added<BuoyancyMarker>>,
    children: Query<&Children>,
    mesh_handles: Query<&Handle<Mesh>>,
) {
    for _event in visualize_mesh_bounds_debug_event_reader.read() {
        for (entity, _, _mesh_transform) in query.iter() {
            if let Some(mesh_handle) = find_mesh(entity, &children, &mesh_handles) {
                if let Some(mesh) = meshes.get(mesh_handle.id()) {
                    let bounds = calculate_mesh_bounds(mesh);
                    visualize_bounds(&mut commands, &mut meshes, &mut materials, bounds);
                }
            }
        }
    }
}
