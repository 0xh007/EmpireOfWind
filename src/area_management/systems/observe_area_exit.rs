use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use crate::area_management::{ActiveAreas, ExitArea, Occluding};
use crate::area_management::utils::update_camera_layers;
use crate::camera_control::MainCamera;

pub fn observe_area_exit(
    mut trigger: EventReader<ExitArea>,
    mut active_areas: ResMut<ActiveAreas>,
    mut camera_query: Query<&mut RenderLayers, With<MainCamera>>,
    occluding_query: Query<&Occluding>,
) {
    for event in trigger.read() {
        active_areas.0.remove(&event.area_name);

        if let Ok(mut camera_layers) = camera_query.get_single_mut() {
            // Update layers based on remaining active areas
            let remaining_layers = RenderLayers::default(); // Calculate based on remaining areas
            update_camera_layers(&mut camera_layers, &active_areas, &occluding_query, remaining_layers);
        }
    }
}