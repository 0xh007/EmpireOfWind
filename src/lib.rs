pub use bevy::app::{PluginGroup, PluginGroupBuilder};
use bevy::app::PreUpdate;
use bevy_gltf_components::ComponentsFromGltfPlugin;
use bevy_registry_export::ExportRegistryPlugin;
use bevy_tnua::prelude::TnuaControllerPlugin;
use bevy_tnua_xpbd3d::TnuaXpbd3dPlugin;
use big_brain::BigBrainPlugin;

use ai_eating_behavior::AiEatingBehaviorPlugin;
use ai_navigation::AiNavigationPlugin;
use ai_sleeping_behavior::AiSleepingBehaviorPlugin;
use asset_management::AssetManagementPlugin;
use atmospheric_lighting::AtmosphericLightingPlugin;
use camera_control::CameraControlPlugin;
use collider_management::ColliderManagementPlugin;
use crew_management::CrewManagementPlugin;
use food::FoodPlugin;
use navmesh::NavMeshPlugin;
use player::PlayerPlugin;
use player_input::PlayerInputPlugin;
use ship_items::ShipItemsPlugin;
use stairs_test::StairsTestPlugin;
use sun::SunCyclePlugin;

mod ai_eating_behavior;
mod ai_navigation;
mod ai_sleeping_behavior;
mod area_visibility;
mod asset_management;
mod atmospheric_lighting;
mod buoyancy_physics;
mod camera_control;
mod collider_management;
mod crew_management;
mod food;
mod navmesh;
mod ocean;
mod player;
mod player_input;
mod ship;
mod ship_items;
mod sun;
mod utils;
mod stairs_test;

/// PluginGroup for the Empire of Wind game.
///
/// The `EmpireOfWindPlugins` group encompasses all the individual plugins
/// used within the game, ensuring they are initialized and managed together.
// pub struct EmpireOfWindPlugins;
//
pub struct EmpireOfWindPlugins;

impl PluginGroup for EmpireOfWindPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(AiEatingBehaviorPlugin)
            .add(AiNavigationPlugin)
            .add(AiSleepingBehaviorPlugin)
            // .add(AreaVisibilityPlugin)
            .add(AssetManagementPlugin)
            .add(AtmosphericLightingPlugin)
            .add(BigBrainPlugin::new(PreUpdate))
            // .add(BuoyancyPhysicsPlugin)
            .add(CameraControlPlugin)
            .add(ColliderManagementPlugin)
            .add(ComponentsFromGltfPlugin::default())
            .add(CrewManagementPlugin)
            .add(ExportRegistryPlugin::default())
            .add(FoodPlugin)
            .add(NavMeshPlugin)
            // .add(OceanPlugin)
            .add(PlayerPlugin)
            .add(PlayerInputPlugin)
            // .add(ShipPlugin)
            .add(ShipItemsPlugin)
            .add(SunCyclePlugin)
            .add(TnuaControllerPlugin)
            .add(TnuaXpbd3dPlugin)
            .add(StairsTestPlugin)
    }
}
