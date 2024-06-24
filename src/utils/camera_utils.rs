use bevy::prelude::{Query, With};
use bevy::render::view::RenderLayers;

use crate::components::{CameraZoom, MainCamera};
use crate::resources::active_areas::ActiveAreas;

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


pub fn update_zoom_target(
    camera_zoom_query: &mut Query<&mut CameraZoom, With<MainCamera>>,
    target_scale: f32,
) {
    for mut zoom in camera_zoom_query.iter_mut() {
        zoom.target_scale = target_scale;
        println!("Setting target zoom scale to {}", target_scale);
    }
}
