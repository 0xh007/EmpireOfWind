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
/// - `render_layers`: Mutable reference to the RenderLayers component of the main camera.
/// - `active_areas`: Resource containing the set of active areas.
/// - `occluding_query`: Query to check for entities with `Occluding` components.
/// - `area_render_layers`: The render layers associated with the current area.
pub fn update_camera_layers(
    render_layers: &mut RenderLayers,
    active_areas: &ActiveAreas,
    occluding_query: &Query<&Occluding>,
    area_render_layers: RenderLayers,
) {
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



/// Updates the zoom target of the main camera.
///
/// This function sets the zoom target scale of the main camera to the specified value.
/// It is typically used to adjust the camera zoom level dynamically based on game events.
///
/// # Parameters
/// - `camera_zoom`: Mutable reference to the CameraZoom component of the main camera.
/// - `target_scale`: The new target zoom scale to set.
pub fn update_zoom_target(camera_zoom: &mut CameraZoom, target_scale: f32) {
    camera_zoom.target_scale = target_scale;
}
