use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_xpbd_3d::prelude::*;

use empire_of_wind::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(CorePlugins)
        .run();
}
