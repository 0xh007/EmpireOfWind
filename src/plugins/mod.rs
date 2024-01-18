use bevy::app::{PluginGroup, PluginGroupBuilder};

mod atmosphere;
mod camera;
mod editor_types;
mod fatigue;
mod input;
mod movement;
mod navmesh;
mod npc;
mod ocean;
mod player;
mod ship;
mod ship_builder;
mod sleep;

pub struct CorePlugins;

impl PluginGroup for CorePlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(atmosphere::AtmospherePlugin)
            .add(camera::CameraPlugin)
            .add(editor_types::EditorTypesPlugin)
            .add(fatigue::FatiguePlugin)
            .add(input::InputPlugin)
            .add(movement::MovementPlugin)
            .add(navmesh::NavMeshPlugin)
            .add(npc::NpcPlugin)
            .add(ocean::OceanPlugin)
            .add(player::PlayerPlugin)
            .add(ship_builder::ShipBuilderPlugin)
            .add(sleep::SleepPlugin)

        // Disable
        // .add(ship::ShipPlugin)
    }
}
