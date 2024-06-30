use bevy::prelude::*;
use bevy_tnua::prelude::*;

use systems::*;

mod systems;

/// Plugin for handling player input within the game.
///
/// The `PlayerInputPlugin` is responsible for processing player input to control
/// the player character's movement and actions. It registers the necessary systems
/// to handle input events and update the player character accordingly.
///
/// # Systems
/// - `handle_player_input`: Handles player input to control movement and actions of the player character.
pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_player_input.in_set(TnuaUserControlsSystemSet),
        );
    }
}
