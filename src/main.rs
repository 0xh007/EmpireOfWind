use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::{PresentMode, WindowTheme},
};
use bevy_editor_pls::{prelude, EditorPlugin};
use bevy_xpbd_3d::prelude::*;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;

use empire_of_wind::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Empire of Wind".into(),
                    resolution: (1920., 1080.).into(),
                    present_mode: PresentMode::AutoVsync,
                    window_theme: Some(WindowTheme::Dark),
                    ..default()
                }),
                ..default()
            }),
            LogDiagnosticsPlugin::default(),
            // FrameTimeDiagnosticsPlugin,
        ))
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(EditorPlugin::default())
        .add_plugins(CorePlugins)
        .run();
}
