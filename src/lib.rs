pub use bevy::app::{PluginGroup, PluginGroupBuilder};
use bevy::prelude::States;

use area_management::AreaManagementPlugin;
use buoyancy_physics::BuoyancyPhysicsPlugin;
use camera_control::CameraControlPlugin;

mod area_management;
mod buoyancy_physics;
mod camera_control;

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
            .add(AreaManagementPlugin)
            .add(BuoyancyPhysicsPlugin)
    }
}
