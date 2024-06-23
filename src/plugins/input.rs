use bevy::prelude::*;
use bevy_tnua::prelude::*;

use crate::prelude::*;
use crate::systems::handle_player_input;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_player_input::handle_player_input.in_set(TnuaUserControlsSystemSet));
    }
}
