use bevy::prelude::{Query, With};
use bevy::render::view::RenderLayers;

use crate::area_visibility::ActiveAreas;
use crate::camera_control::{CameraZoom, MainCamera};

/// Updates the render layers of the main camera based on active areas.
///
/// This function iterates over all cameras with the `MainCamera` component and updates
/// their render layers according to the specified active areas. Specific layers are
/// removed depending on the presence of certain area names in the `active_areas` set.
///
/// # Parameters
/// - `camera_query`: Query to fetch and modify the render layers of the main camera.
/// - `active_areas`: Resource containing the set of active areas.
pub fn update_camera_layers(
    camera_query: &mut Query<&mut RenderLayers, With<MainCamera>>,
    active_areas: &ActiveAreas,
) {
    for mut render_layers in camera_query.iter_mut() {
        let mut layers = (0..RenderLayers::TOTAL_LAYERS as u8).collect::<Vec<u8>>(); // Start with all layers

        if active_areas.0.contains("Deck 2 Aft Cabin") {
            layers.retain(|&layer| layer != 1); // Remove layer 1
        }
        if active_areas.0.contains("Deck 3 Aft Cabin") {
            layers.retain(|&layer| layer != 1 && layer != 2); // Remove layers 1 and 2
        }

        *render_layers = RenderLayers::from_layers(&layers);
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
