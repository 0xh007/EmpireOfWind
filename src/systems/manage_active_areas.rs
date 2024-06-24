use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy_xpbd_3d::prelude::Collision;

use crate::components::{
    AreaEnterMarker, AreaExitMarker, AreaName, CameraZoom, MainCamera, Player,
};
use crate::resources::active_areas::ActiveAreas;

pub fn manage_active_areas(
    mut collision_event_reader: EventReader<Collision>,
    sensor_query: Query<(
        Entity,
        &Sensor,
        Option<&AreaEnterMarker>,
        Option<&AreaExitMarker>,
        &AreaName,
    )>,
    player_query: Query<&Player>,
    mut active_areas: ResMut<ActiveAreas>,
    mut camera_layers_query: Query<&mut RenderLayers, With<MainCamera>>,
    mut camera_zoom_query: Query<&mut CameraZoom, With<MainCamera>>,
) {
    for Collision(contacts) in collision_event_reader.read() {
        let entity1 = contacts.entity1;
        let entity2 = contacts.entity2;

        let player_involved =
            player_query.get(entity1).is_ok() || player_query.get(entity2).is_ok();
        if player_involved {
            let (player_entity, other_entity) = if player_query.get(entity1).is_ok() {
                (entity1, entity2)
            } else {
                (entity2, entity1)
            };

            if let Ok((_, _, enter_marker, exit_marker, area_name)) =
                sensor_query.get(other_entity)
            {
                if enter_marker.is_some() {
                    println!(
                        "Player {:?} entered area: {:?}",
                        player_entity, area_name.0
                    );
                    active_areas.0.insert(area_name.0.clone());
                    update_zoom_target(&mut camera_zoom_query, 10.0); // Adjust zoom for entry
                } else if exit_marker.is_some() {
                    println!(
                        "Player {:?} exited area: {:?}",
                        player_entity, area_name.0
                    );
                    active_areas.0.remove(&area_name.0);
                    update_zoom_target(&mut camera_zoom_query, 20.0); // Adjust zoom for exit
                }
            }
        }
    }

    update_camera_layers(&mut camera_layers_query, &active_areas);
}

fn update_camera_layers(
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


fn update_zoom_target(
    camera_zoom_query: &mut Query<&mut CameraZoom, With<MainCamera>>,
    target_scale: f32,
) {
    for mut zoom in camera_zoom_query.iter_mut() {
        zoom.target_scale = target_scale;
        println!("Setting target zoom scale to {}", target_scale);
    }
}