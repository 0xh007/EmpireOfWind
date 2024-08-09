use bevy::{
    diagnostic::LogDiagnosticsPlugin,
    log::LogPlugin,
    prelude::*,
    window::{PresentMode, WindowTheme},
};

use bevy_xpbd_3d::prelude::*;

use empire_of_wind::EmpireOfWindPlugins;

fn main() {
    let mut app = App::new();

    let default_plugins = DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Empire of Wind".into(),
            resolution: (1920., 1080.).into(),
            present_mode: PresentMode::AutoVsync,
            window_theme: Some(WindowTheme::Dark),
            ..default()
        }),
        ..default()
    });

    #[cfg(debug_assertions)]
    let default_plugins = default_plugins.set(LogPlugin {
        filter: "info,wgpu_core=warn,wgpu_hal=warn,empire_of_wind=debug".into(),
        level: bevy::log::Level::DEBUG,
        ..default()
    });

    // #[cfg(not(debug_assertions))]
    // let default_plugins = default_plugins.set(LogPlugin {
    //     filter: "warn".into(),
    //     level: bevy::log::Level::WARN,
    // });

    app.add_plugins(default_plugins)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(EmpireOfWindPlugins)
        .add_plugins(LogDiagnosticsPlugin::default());
    app.run();
}
