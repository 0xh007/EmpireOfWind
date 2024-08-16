use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy_xpbd_3d::prelude::Collision;
use crate::area_management::EnterArea;
use crate::player::Player;

pub fn observe_area_sensors(
    mut collision_events: EventReader<Collision>,
    player_query: Query<Entity, With<Player>>,
    area_query: Query<(Entity, &AreaMarker, &RenderLayers), With<AreaSensor>>,
    mut enter_area_events: EventWriter<EnterArea>,
) {
    for collision in collision_events.read() {
        let (entity1, entity2) = (collision.entity1, collision.entity2);

        let player_entity = if player_query.contains(entity1) {
            Some(entity1)
        } else if player_query.contains(entity2) {
            Some(entity2)
        } else {
            None
        };

        if let Some(_player_entity) = player_entity {
            let area_entity = if area_query.contains(entity1) {
                entity1
            } else if area_query.contains(entity2) {
                entity2
            } else {
                continue;
            };

            if let Ok((_, area_marker, render_layers)) = area_query.get(area_entity) {
                enter_area_events.send(EnterArea {
                    area_name: area_marker.name.clone(),
                    render_layers: *render_layers,
                });
            }
        }
    }
}
