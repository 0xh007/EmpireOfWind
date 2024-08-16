use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use crate::area_management::{ActiveAreas, EnterArea, Occluding};
use crate::area_management::utils::{update_camera_layers, update_zoom_target};
use crate::camera_control::{CameraZoom, MainCamera};

pub fn observe_area_entry(
    mut trigger: EventReader<EnterArea>,
    mut active_areas: ResMut<ActiveAreas>,
    mut camera_query: Query<(&mut RenderLayers, &mut CameraZoom), With<MainCamera>>,
    occluding_query: Query<&Occluding>,
) {
    for event in trigger.read() {
        active_areas.0.insert(event.area_name.clone());

        if let Ok((mut camera_layers, mut camera_zoom)) = camera_query.get_single_mut() {
            update_camera_layers(&mut camera_layers, &active_areas, &occluding_query, event.render_layers);
            update_zoom_target(&mut camera_zoom, 10.0);
        }
    }
}