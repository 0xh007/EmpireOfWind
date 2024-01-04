use bevy::prelude::*;
use bevy::pbr::{CascadeShadowConfigBuilder, NotShadowCaster};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::render::camera::ScalingMode;

const SHIP_LENGTH: i32 = 40;
const SHIP_WIDTH: i32 = 10;
const SHIP_HEIGHT: i32 = 8;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct NPC;

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct DebugCamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, spawn_ship)
        .add_systems(Startup, spawn_ocean)
        .add_systems(Startup, spawn_player)
        .add_systems(Update, camera_switching)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cascade_shadow_config = CascadeShadowConfigBuilder {
        first_cascade_far_bound: 0.3,
        maximum_distance: 3.0,
        ..default()
    }
    .build();
    
    // Sun
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::rgb(0.98, 0.95, 0.82),
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0)
            .looking_at(Vec3::new(-0.15, -0.05, 0.25), Vec3::Y),
        cascade_shadow_config,
        ..default()
    });
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                order: 0,
                is_active: true,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
            ..default()
        },
        PanOrbitCamera::default(),
        DebugCamera,
    ));

    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                order: 1,
                is_active: false,
                ..default()
            },
            transform: Transform::from_xyz(20.0, 12.0, 40.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        MainCamera,
    ));
}

fn camera_switching(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Camera, &DebugCamera), Without<MainCamera>>,
    mut query_main: Query<(&mut Camera, &MainCamera), Without<DebugCamera>>,
) {
    if keyboard_input.just_pressed(KeyCode::Key0) {
        for (mut camera, _) in query.iter_mut() {
            camera.is_active = !camera.is_active;
        }

        for (mut camera, _) in query_main.iter_mut() {
            camera.is_active = !camera.is_active;
        }
    }
}


fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player_height = 1.8;
    let player_start_height = SHIP_HEIGHT as f32 + player_height / 2.0; // Position player on top of the ship

    // Adjust X and Y position as needed. Example: Center of the ship
    let player_start_x = SHIP_LENGTH as f32 / 2.0;
    let player_start_z = SHIP_WIDTH as f32 / 2.0;

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cylinder { 
            height: player_height,
            ..default()
        })),
        material: materials.add(Color::YELLOW.into()),
        transform: Transform::from_xyz(player_start_x, player_start_height, player_start_z),
        ..default()
    });
}


fn spawn_ship(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube_size = 1.0; // Size of each cube, adjust as needed
    for x in 0..SHIP_LENGTH {
        for y in 0..SHIP_WIDTH {
            for z in 0..SHIP_HEIGHT {
                // Check if the current position is on the boundary
                if x == 0 || x == SHIP_LENGTH - 1 || y == 0 || y == SHIP_WIDTH - 1 || z == 0 || z == SHIP_HEIGHT - 1 {
                    commands.spawn(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: cube_size })),
                        material: materials.add(Color::hex("D18251").unwrap().into()),
                        transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                        ..default()
                    });
                }
            }
        }
    }
}

fn spawn_ocean(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Circle::new(400.0).into()),
        material: materials.add(Color::hex("618f92").unwrap().into()),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
}
