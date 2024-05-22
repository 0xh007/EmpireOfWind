use bevy::app::{PluginGroup, PluginGroupBuilder, PreUpdate};
use bevy_gltf_components::ComponentsFromGltfPlugin;

pub use assets::AppStates;
pub use {
    assets::*, camera::*, eat::*, editor_types::*, fatigue::*, hunger::*, input::*, navmesh::*,
    npc::*, ocean::*, pathfinding::*, physics::*, player::*, ship::*, sky::*, sleep::*,
};

mod assets;
mod camera;
mod eat;
mod editor_types;
mod fatigue;
mod hunger;
mod input;
mod navmesh;
mod npc;
mod ocean;
mod pathfinding;
mod physics;
mod player;
pub mod ship;
mod sky;
mod sleep;

pub struct CorePlugins;

impl PluginGroup for CorePlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(assets::AssetsPlugin)
            .add(big_brain::BigBrainPlugin::new(PreUpdate))
            .add(camera::CameraPlugin)
            .add(ComponentsFromGltfPlugin::default())
            .add(eat::EatPlugin)
            .add(editor_types::EditorTypesPlugin)
            .add(fatigue::FatiguePlugin)
            .add(hunger::HungerPlugin)
            .add(input::InputPlugin)
            .add(navmesh::NavMeshPlugin)
            // .add(npc::NpcPlugin)
            .add(ocean::OceanPlugin)
            .add(pathfinding::PathfindingPlugin)
            .add(physics::PhysicsPlugin)
            .add(player::PlayerPlugin)
            .add(sleep::SleepPlugin)
            .add(ship::ShipPlugin)
            .add(sky::SkyPlugin)
    }
}
