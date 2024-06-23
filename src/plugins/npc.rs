use bevy::prelude::*;
use crate::prelude::*;
use crate::systems::spawn_npc;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_npc::spawn_npc);
    }
}
