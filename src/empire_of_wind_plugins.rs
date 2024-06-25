pub use bevy::app::{PluginGroup, PluginGroupBuilder, PreUpdate};

use crate::area_management::AreaManagementPlugin;

/// PluginGroup for the Empire of Wind game.
///
/// The `EmpireOfWindPlugin` group encompasses all the individual plugins
/// used within the game, ensuring they are initialized and managed together.
pub struct EmpireOfWindPlugins;

impl PluginGroup for EmpireOfWindPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(AreaManagementPlugin)
    }
}