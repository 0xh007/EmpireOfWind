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
/// adjusts the camera zoom level, and updates the render layers accordingly. It listens for
/// collision events involving the player and area sensors, and modifies the active areas
/// resource, camera zoom, and render layers as necessary.
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
/// - `param_set`: ParamSet to handle multiple conflicting queries safely.
/// - `occluding_query`: Query to check for entities with `Occluding` components.
///
/// # Type Parameters
/// - `Player`: Component type that identifies the player entity.
/// - `Sensor`: Component type for area sensors.
/// - `AreaMarker`: Component type that marks and names an area.
/// - `Occluding`: Component type for entities that can occlude areas.
#[allow(clippy::type_complexity)]
pub fn manage_active_areas(
    mut collision_event_reader: EventReader<Collision>,
    sensor_query: Query<(Entity, &Sensor, Option<&AreaMarker>, Option<&RenderLayers>)>,
    player_query: Query<&Player>,
    mut active_areas: ResMut<ActiveAreas>,
    mut param_set: ParamSet<(
        Query<&mut RenderLayers, With<MainCamera>>,
        Query<&mut CameraZoom, With<MainCamera>>,
    )>,
    occluding_query: Query<&Occluding>,
) {
    for Collision(contacts) in collision_event_reader.read() {
        let entity1 = contacts.entity1;
        let entity2 = contacts.entity2;

        let player_involved =
            player_query.get(entity1).is_ok() || player_query.get(entity2).is_ok();
        if player_involved {
            let (_player_entity, other_entity) = if player_query.get(entity1).is_ok() {
                (entity1, entity2)
            } else {
                (entity2, entity1)
            };

            if let Ok((_, _, Some(area_marker), render_layers)) = sensor_query.get(other_entity) {
                active_areas.0.insert(area_marker.name.clone());
                update_zoom_target(&mut param_set.p1(), 10.0); // Adjust zoom for entry

                // Use the entity's RenderLayers if available, otherwise use a default
                let layers = render_layers.cloned().unwrap_or_else(|| RenderLayers::layer(0));

                update_camera_layers(
                    &mut param_set.p0(),
                    &active_areas,
                    &occluding_query,
                    layers,
                );
            }
        }
    }
}