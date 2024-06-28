pub use bevy::app::{PluginGroup, PluginGroupBuilder};
use bevy::prelude::States;

use ai_eating_behavior::AiEatingBehaviorPlugin;
use ai_navigation::AiNavigationPlugin;
use ai_sleeping_behavior::AiSleepingBehaviorPlugin;
use area_management::AreaManagementPlugin;
use buoyancy_physics::BuoyancyPhysicsPlugin;
use camera_control::CameraControlPlugin;
use collider_management::ColliderManagementPlugin;
use food::FoodPlugin;
use navmesh::NavMeshPlugin;
use sun_cycle::SunCyclePlugin;

mod ai_eating_behavior;
mod ai_navigation;
mod ai_sleeping_behavior;
mod area_management;
mod buoyancy_physics;
mod camera_control;
mod collider_management;
mod food;
mod navmesh;
mod sun_cycle;


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
            .add(AiSleepingBehaviorPlugin)
            .add(AiNavigationPlugin)
            .add(AreaManagementPlugin)
            .add(BuoyancyPhysicsPlugin)
            .add(CameraControlPlugin)
            .add(ColliderManagementPlugin)
            .add(FoodPlugin)
            .add(NavMeshPlugin)
            .add(SunCyclePlugin)
    }
}
