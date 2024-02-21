use bevy::prelude::*;
use bevy_xpbd_3d::prelude::Collider;
use oxidized_navigation::{
    debug_draw::{DrawNavMesh, OxidizedNavigationDebugDrawPlugin},
    NavMeshSettings, OxidizedNavigationPlugin,
};

use crate::prelude::*;

pub struct NavMeshPlugin;

impl Plugin for NavMeshPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NavMeshDebugToggle>()
            .add_plugins((
                OxidizedNavigationPlugin::<Collider>::new(NavMeshSettings {
                    cell_width: 0.25,
                    cell_height: 0.1,
                    tile_width: 100,
                    world_half_extents: 250.0,
                    world_bottom_bound: -100.0,
                    max_traversable_slope_radians: (40.0_f32 - 0.1).to_radians(),
                    walkable_height: 20,
                    walkable_radius: 1,
                    step_height: 3,
                    min_region_area: 100,
                    merge_region_area: 500,
                    max_contour_simplification_error: 1.1,
                    max_edge_length: 80,
                    max_tile_generation_tasks: Some(9),
                }),
                OxidizedNavigationDebugDrawPlugin,
            ))
            .add_systems(Update, toggle_nav_mesh);
    }
}

fn toggle_nav_mesh(
    mut navmesh_debug_event_reader: EventReader<NavMeshDebugToggle>,
    mut show_navmesh: ResMut<DrawNavMesh>,
) {
    for _event in navmesh_debug_event_reader.read() {
        println!("TOGGLING NAVMESH!");
        show_navmesh.0 = !show_navmesh.0;
    }
}
