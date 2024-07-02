use bevy::prelude::*;

pub use components::*;
use systems::*;

use crate::asset_management::AppStates;

mod components;
mod systems;

pub struct CrewManagementPlugin;

/// Plugin for managing the crew within the game world.
///
/// The `CrewManagementPlugin` handles the creation and management of crew members,
/// setting up the necessary components and systems to simulate their behavior and interactions.
///
/// # Components
/// - `CrewMember`: A marker component indicating that an entity is a crew member.
///
/// # Systems
/// - `spawn_crew_members`: Spawns crew members in the game world and configures their initial behavior and properties.
impl Plugin for CrewManagementPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CrewMember>()
            .add_systems(OnEnter(AppStates::Running), spawn_crew_members);
    }
}
