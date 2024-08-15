use bevy::{prelude::*, render::view::RenderLayers};

use crate::{
    area_management::{ActiveAreas, EnterArea, Occluding},
    camera_control::{self, CameraZoom, MainCamera},
};

pub fn setup_area_management(mut commands: Commands) {
    // Set up global observer for area entry
    commands.observe(
        |trigger: Trigger<EnterArea>,
         mut active_areas: ResMut<ActiveAreas>,
         mut camera_query: Query<(&mut RenderLayers, &mut CameraZoom), With<MainCamera>>,
         occluding_query: Query<&Occluding>| {
            let event = trigger.event();
            active_areas.0.insert(event.area_name.clone());

            if let Ok((mut camera_layers, mut camera_zoom)) = camera_query.get_single_mut() {
                update_camera_layers(
                    &mut camera_layers,
                    &active_areas,
                    &occluding_query,
                    event.render_layers,
                );
                update_zoom_target(&mut camera_zoom, 10.0); // Adjust zoom for entry
            }
        },
    );

    // Set up global observer for area exit
    commands.observe(
        |trigger: Trigger<ExitArea>,
         mut active_areas: ResMut<ActiveAreas>,
         mut camera_query: Query<&mut RenderLayers, With<MainCamera>>,
         occluding_query: Query<&Occluding>| {
            let event = trigger.event();
            active_areas.0.remove(&event.area_name);

            if let Ok(mut camera_layers) = camera_query.get_single_mut() {
                // Update layers based on remaining active areas
                let remaining_layers = RenderLayers::default(); // You might need to calculate this based on remaining areas
                update_camera_layers(
                    &mut camera_layers,
                    &active_areas,
                    &occluding_query,
                    remaining_layers,
                );
            }
        },
    );
}
