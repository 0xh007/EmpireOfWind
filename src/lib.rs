pub use bevy::app::{PluginGroup, PluginGroupBuilder};
use bevy::prelude::States;

use ai_eating_behavior::AiEatingBehaviorPlugin;
use ai_navigation::AiNavigationPlugin;
use ai_sleeping_behavior::AiSleepingBehaviorPlugin;
use area_visibility::AreaVisibilityPlugin;
use atmospheric_lighting::AtmosphericLightingPlugin;
use buoyancy_physics::BuoyancyPhysicsPlugin;
use camera_control::CameraControlPlugin;
use collider_management::ColliderManagementPlugin;
use crew_management::CrewManagementPlugin;
use food::FoodPlugin;
use navmesh::NavMeshPlugin;
use ocean::OceanPlugin;
use player::PlayerPlugin;
use player_input::PlayerInputPlugin;
use ship::ShipPlugin;
use ship_items::ShipItemsPlugin;
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

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum AppStates {
    #[default]
    AssetLoading,
    Running,
}

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
            .add(AreaVisibilityPlugin)
            .add(AtmosphericLightingPlugin)
            .add(BuoyancyPhysicsPlugin)
            .add(CameraControlPlugin)
            .add(ColliderManagementPlugin)
            .add(CrewManagementPlugin)
            .add(FoodPlugin)
            .add(NavMeshPlugin)
            .add(OceanPlugin)
            .add(PlayerPlugin)
            .add(PlayerInputPlugin)
            .add(ShipPlugin)
            .add(ShipItemsPlugin)
            .add(SunCyclePlugin)
    }
}
