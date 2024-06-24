use bevy::prelude::{Commands, default, Res, SceneBundle};
use bevy::core::Name;
use crate::components::ship::Ship;
use crate::resources::ship_assets::ShipAssets;

pub fn spawn_ship(mut commands: Commands, ship_assets: Res<ShipAssets>) {
    commands.spawn((
        Ship,
        Name::new("Ship"),
        SceneBundle {
            scene: ship_assets.ship.clone(),
            ..default()
        },
    ));
}
