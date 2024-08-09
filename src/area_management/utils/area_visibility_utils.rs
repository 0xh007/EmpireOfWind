use bevy::prelude::{Query, With};
use bevy::render::view::RenderLayers;

use crate::area_management::{ActiveAreas, Occluding};
use crate::camera_control::{CameraZoom, MainCamera};

/// Updates the render layers of the main camera based on active areas and occlusion.
///
/// This function modifies the render layers of the main camera to show or hide certain
/// areas based on which areas are active and which are being occluded. It uses the
/// RenderLayers implementation from Bevy 0.14, which supports an unlimited number of layers.
///
/// # Parameters
/// - `camera_query`: Query to fetch and modify the render layers of the main camera.
/// - `active_areas`: Resource containing the set of active areas.
/// - `occluding_query`: Query to check for entities with `Occluding` components.
/// - `area_render_layers`: The render layers associated with the current area.
pub fn update_camera_layers(
    camera_query: &mut Query<&mut RenderLayers, With<MainCamera>>,
    active_areas: &ActiveAreas,
    occluding_query: &Query<&Occluding>,
    area_render_layers: RenderLayers,
) {
    for mut render_layers in camera_query.iter_mut() {
        // Start with the current render layers
        let mut new_layers = render_layers.clone();

        for occluding in occluding_query.iter() {
            if occluding.areas.iter().any(|area| active_areas.0.contains(area)) {
                // Remove the layers that are in area_render_layers
                new_layers = new_layers.intersection(&area_render_layers);
            }
        }

        *render_layers = new_layers;
    }
}


/// Updates the zoom target of the main camera.
///
/// This function iterates over all cameras with the `MainCamera` component and sets their
/// zoom target scale to the specified value. It is typically used to adjust the camera
/// zoom level dynamically based on game events.
///
/// # Parameters
/// - `camera_zoom_query`: Query to fetch and modify the zoom target of the main camera.
/// - `target_scale`: The new target zoom scale to set.
pub fn update_zoom_target(
    camera_zoom_query: &mut Query<&mut CameraZoom, With<MainCamera>>,
    target_scale: f32,
) {
    for mut zoom in camera_zoom_query.iter_mut() {
        zoom.target_scale = target_scale;
    }
}
