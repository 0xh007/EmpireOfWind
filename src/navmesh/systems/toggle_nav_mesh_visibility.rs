use bevy::prelude::{EventReader, ResMut};
use oxidized_navigation::debug_draw::DrawNavMesh;
use crate::events::NavMeshDebugToggle;

/// Toggles the visibility of the navigation mesh debug view.
///
/// This system listens for `NavMeshDebugToggle` events and toggles the visibility
/// of the navigation mesh in the game world. It utilizes the `DrawNavMesh` resource
/// from the `oxidized_navigation` crate to control the debug rendering of the
/// navigation mesh.
///
/// # Parameters
///
/// * `navmesh_debug_event_reader`: An EventReader for `NavMeshDebugToggle` events. This
/// reader listens for events that trigger the toggling of the navigation mesh visibility.
/// * `show_navmesh`: A mutable reference to the `DrawNavMesh` resource, which controls
/// whether the navigation mesh is currently being visualized in debug mode.
///
/// # Behavior
///
/// For each `NavMeshDebugToggle` event read from the event reader, the system toggles
/// the `DrawNavMesh` resource's boolean value, effectively turning the debug visualization
/// of the navigation mesh on or off.
pub fn toggle_nav_mesh_visibility(
    mut navmesh_debug_event_reader: EventReader<NavMeshDebugToggle>,
    mut show_navmesh: ResMut<DrawNavMesh>,
) {
    for _event in navmesh_debug_event_reader.read() {
        show_navmesh.0 = !show_navmesh.0;
    }
}

