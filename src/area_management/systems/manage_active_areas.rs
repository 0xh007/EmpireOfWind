use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy_xpbd_3d::prelude::{Collision, Sensor};

use crate::area_management::components::*;
use crate::area_management::resources::ActiveAreas;
use crate::area_management::utils::{update_camera_layers, update_zoom_target};
use crate::camera_control::{CameraZoom, MainCamera};
use crate::player::Player;

/// Manages active areas based on player interactions with sensors.
///
/// This system updates the set of active areas when the player enters or exits an area,
/// adjusts the camera zoom level, and updates the render layers accordingly. It uses a
/// ParamSet to avoid conflicts with other systems that might access the same components.
///
/// # Functionality
/// - Detects collisions between the player and area sensors.
/// - Updates the `ActiveAreas` resource when the player enters a new area.
/// - Adjusts the camera zoom when entering a new area.
/// - Updates the camera's render layers based on the entered area.
///
/// # Parameters
/// - `collision_event_reader`: Event reader to capture collision events.
/// - `sensor_query`: Query to fetch sensor components, area markers, and render layers.
/// - `player_query`: Query to identify player entities.
/// - `active_areas`: Resource to manage the set of active areas.
/// - `camera_param_set`: ParamSet to safely access and modify the main camera's components.
/// - `occluding_query`: Query to check for entities with `Occluding` components.
#[allow(clippy::type_complexity)]
pub fn manage_active_areas(
    mut collision_event_reader: EventReader<Collision>,
    sensor_query: Query<(Entity, &Sensor, Option<&AreaMarker>, Option<&RenderLayers>)>,
    player_query: Query<&Player>,
    mut active_areas: ResMut<ActiveAreas>,
    mut camera_param_set: ParamSet<(
        Query<&mut RenderLayers, With<MainCamera>>,
        Query<&mut CameraZoom, With<MainCamera>>,
    )>,
    occluding_query: Query<&Occluding>,
) {
    let mut camera_updated = false;
    let mut new_render_layers = None;
    let mut new_zoom = None;

    for Collision(contacts) in collision_event_reader.read() {
        let (player_entity, other_entity) = if player_query.contains(contacts.entity1) {
            (contacts.entity1, contacts.entity2)
        } else if player_query.contains(contacts.entity2) {
            (contacts.entity2, contacts.entity1)
        } else {
            continue; // Neither entity is the player, skip this collision
        };

        if let Ok((_, _, Some(area_marker), render_layers)) = sensor_query.get(other_entity) {
            active_areas.0.insert(area_marker.name.clone());
            new_zoom = Some(10.0); // Adjust zoom for entry

            // Use the entity's RenderLayers if available, otherwise use a default
            let layers = render_layers.cloned().unwrap_or_else(|| RenderLayers::layer(0));
            new_render_layers = Some(layers);

            camera_updated = true;
        }
    }

    // Apply changes to camera if needed
    if camera_updated {
        if let Some(layers) = new_render_layers {
            if let Ok(mut camera_layers) = camera_param_set.p0().get_single_mut() {
                update_camera_layers(&mut camera_layers, &active_areas, &occluding_query, layers);
            }
        }
        if let Some(zoom) = new_zoom {
            if let Ok(mut camera_zoom) = camera_param_set.p1().get_single_mut() {
                update_zoom_target(&mut camera_zoom, zoom);
            }
        }
    }
}