use bevy::prelude::{EventReader, ResMut};
use oxidized_navigation::debug_draw::DrawNavMesh;
use crate::events::NavMeshDebugToggle;

pub fn toggle_nav_mesh_visibility(
    mut navmesh_debug_event_reader: EventReader<NavMeshDebugToggle>,
    mut show_navmesh: ResMut<DrawNavMesh>,
) {
    for _event in navmesh_debug_event_reader.read() {
        println!("TOGGLING NAVMESH!");
        show_navmesh.0 = !show_navmesh.0;
    }
}
