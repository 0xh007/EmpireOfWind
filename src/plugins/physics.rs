use bevy::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua_xpbd3d::*;

use crate::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TnuaControllerPlugin)
            .add_plugins(TnuaXpbd3dPlugin);
    }
}