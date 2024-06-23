use bevy::prelude::*;
use crate::prelude::*;
use crate::systems::spawn_player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player::spawn_player);
    }
}
