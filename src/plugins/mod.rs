use bevy::{
    app::{PluginGroup, PluginGroupBuilder, PreUpdate},
    core_pipeline::experimental::taa::TemporalAntiAliasPlugin,
};
use bevy_gltf_components::ComponentsFromGltfPlugin;
use big_brain::BigBrainPlugin;

pub use assets::AppStates;
mod assets;
mod camera;
mod eat;
mod editor_types;
mod fatigue;
mod hunger;
mod input;
mod movement;
mod navmesh;
mod npc;
mod ocean;
mod pathfinding;
mod physics;
mod player;
mod ship;
mod ship_builder;
mod sky;
mod sleep;

pub struct CorePlugins;

impl PluginGroup for CorePlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(assets::AssetsPlugin)
            .add(BigBrainPlugin::new(PreUpdate))
            .add(camera::CameraPlugin)
            .add(ComponentsFromGltfPlugin::default())
            .add(eat::EatPlugin)
            .add(editor_types::EditorTypesPlugin)
            .add(fatigue::FatiguePlugin)
            .add(hunger::HungerPlugin)
            .add(input::InputPlugin)
            .add(movement::MovementPlugin)
            .add(navmesh::NavMeshPlugin)
            .add(npc::NpcPlugin)
            .add(ocean::OceanPlugin)
            .add(pathfinding::PathfindingPlugin)
            .add(physics::PhysicsPlugin)
            .add(player::PlayerPlugin)
            .add(sleep::SleepPlugin)
            .add(ship::ShipPlugin)
            .add(sky::SkyPlugin)
    }
}
