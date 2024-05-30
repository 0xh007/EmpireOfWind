use bevy::app::{PluginGroup, PluginGroupBuilder, PreUpdate};
use bevy_gltf_components::ComponentsFromGltfPlugin;
use bevy_registry_export::ExportRegistryPlugin;

pub use {
    assets::*, camera::*, eat::*, editor_types::*, fatigue::*, hunger::*, input::*, navmesh::*,
    npc::*, ocean::*, pathfinding::*, physics::*, player::*, ship::*, sky::*, sleep::*,
};
pub use assets::AppStates;

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
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(AssetsPlugin)
            .add(big_brain::BigBrainPlugin::new(PreUpdate))
            .add(CameraPlugin)
            .add(ComponentsFromGltfPlugin::default())
            .add(EatPlugin)
            .add(EditorTypesPlugin)
            .add(ExportRegistryPlugin::default())
            .add(FatiguePlugin)
            .add(HungerPlugin)
            .add(InputPlugin)
            .add(NavMeshPlugin)
            .add(NpcPlugin)
            .add(OceanPlugin)
            .add(PathfindingPlugin)
            .add(PhysicsPlugin)
            .add(PlayerPlugin)
            .add(SleepPlugin)
            .add(ShipPlugin)
            .add(SkyPlugin)
    }
}
