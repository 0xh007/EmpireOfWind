use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy_xpbd_3d::prelude::{Collision, Sensor};

use crate::area_management::components::*;

/// Manages active areas based on player interactions with sensors.
///
/// This system updates the set of active areas when the player enters or exits an area,
/// adjusts the camera zoom level, and updates the render layers accordingly. It listens for
/// collision events involving the player and area sensors, and modifies the active areas
/// resource, camera zoom, and render layers as necessary.
///
/// # Parameters
/// - `collision_event_reader`: Event reader to capture collision events.
/// - `sensor_query`: Query to fetch sensor components and area markers.
/// - `player_query`: Query to identify player entities.
/// - `active_areas`: Resource to manage the set of active areas.
/// - `camera_layers_query`: Query to modify the render layers of the main camera.
/// - `camera_zoom_query`: Query to modify the zoom level of the main camera.
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