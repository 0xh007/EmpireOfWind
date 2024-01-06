use bevy::pbr::{CascadeShadowConfigBuilder, NotShadowCaster};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::transform::commands;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_xpbd_3d::prelude::*;

const SHIP_LENGTH: i32 = 40;
const SHIP_WIDTH: i32 = 10;
const SHIP_HEIGHT: i32 = 10;

const DECK_OFFSET: f32 = -1.4; // This is the additional amount needed to spawn on the deck

const PLAYER_HEIGHT: f32 = 1.8;
const PLAYER_RADIUS: f32 = 1.0;
const PLAYER_SPEED: f32 = 500.0;
const MOVEMENT_SMOOTHING_FACTOR: f32 = 0.5;

#[derive(Component)]
struct Ship;

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
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, spawn_ship)
        .add_systems(Startup, spawn_ocean)
        .add_systems(Startup, spawn_player)
        .add_systems(FixedUpdate, move_player_and_camera)
        .add_systems(Update, camera_switching)
        .run();
}

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
    let ship_center = Vec3::new(
        SHIP_LENGTH as f32 / 2.0,
        SHIP_WIDTH as f32 / 2.0,
        SHIP_HEIGHT as f32 / 2.0,
    );
    let camera_position = Vec3::new(20.0, 12., 40.0);

    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                order: 0,
                is_active: true,
                ..default()
            },
            transform: Transform::from_translation(camera_position)
                .looking_at(ship_center, Vec3::Y),
            ..default()
        },
        MainCamera,
    ));

    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                order: 1,
                is_active: false,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
            ..default()
        },
        PanOrbitCamera::default(),
        DebugCamera,
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
    let player_start_height = SHIP_HEIGHT as f32 + PLAYER_HEIGHT + DECK_OFFSET; // Position player on top of the ship

    let player_start_x = SHIP_LENGTH as f32 / 2.0;
    let player_start_z = SHIP_WIDTH as f32 / 2.0;

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cylinder {
                height: PLAYER_HEIGHT,
                ..default()
            })),
            material: materials.add(Color::YELLOW.into()),
            transform: Transform::from_xyz(player_start_x, player_start_height, player_start_z),
            ..default()
        },
        RigidBody::Kinematic,
        Collider::capsule(PLAYER_HEIGHT / 2.0, PLAYER_RADIUS),
        Player,
    ));
}

fn spawn_ship(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube_size = 1.0; // Size of each cube, adjust as needed

    // Spawn the parent entity with a minimal Transform component
    commands
        .spawn((
            TransformBundle::default(),
            Ship,
            InheritedVisibility::default(),
        ))
        .with_children(|parent| {
            for x in 0..SHIP_LENGTH {
                for z in 0..SHIP_WIDTH {
                    for y in 0..SHIP_HEIGHT {
                        // Check if the current position is on the boundary
                        if x == 0
                            || x == SHIP_LENGTH - 1
                            || z == 0
                            || z == SHIP_WIDTH - 1
                            || y == 0
                            || y == SHIP_HEIGHT - 1
                        {
                            parent.spawn((
                                PbrBundle {
                                    mesh: meshes.add(Mesh::from(shape::Cube { size: cube_size })),
                                    material: materials.add(Color::hex("D18251").unwrap().into()),
                                    transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                                    ..default()
                                },
                                RigidBody::Static,
                                Collider::cuboid(cube_size / 2.0, cube_size / 2.0, cube_size / 2.0),
                            ));
                        }
                    }
                }
            }
            // Spawn railing
            for x in 0..SHIP_LENGTH {
                for z in 0..SHIP_WIDTH {
                    let y = SHIP_HEIGHT; // One layer above the ship
                                         // Spawn railing on edges
                    if (x == 0 || x == SHIP_LENGTH - 1 || z == 0 || z == SHIP_WIDTH - 1) {
                        parent.spawn((
                            PbrBundle {
                                mesh: meshes.add(Mesh::from(shape::Cube { size: cube_size })),
                                material: materials.add(Color::hex("D18251").unwrap().into()),
                                transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                                ..default()
                            },
                            RigidBody::Static,
                            Collider::cuboid(cube_size / 2.0, cube_size / 2.0, cube_size / 2.0),
                        ));
                    }
                }
            }
        });
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

fn move_player_and_camera(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut LinearVelocity, &Transform), With<Player>>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
    time: Res<Time>,
) {
    if let Ok((mut linear_velocity, player_transform)) = query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        // Handle horizontal movement
        if keyboard_input.pressed(KeyCode::A) {
            direction.x -= 1.0; // Move left
        }
        if keyboard_input.pressed(KeyCode::D) {
            direction.x += 1.0; // Move right
        }

        // Handle forward/backward movement
        if keyboard_input.pressed(KeyCode::W) {
            direction.z -= 1.0; // Move forward
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction.z += 1.0; // Move backward
        }

        // Normalize the direction vector and scale by player speed
        if direction.length_squared() > 0.0 {
            direction = direction.normalize() * PLAYER_SPEED;
        }

        // Apply a smoothing factor to the velocity change
        linear_velocity.0 = (linear_velocity.0 * (1.0 - MOVEMENT_SMOOTHING_FACTOR))
            + (direction * time.delta_seconds() * MOVEMENT_SMOOTHING_FACTOR);

        // Update the camera position
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            let target_position = Vec3::new(
                player_transform.translation.x,
                player_transform.translation.y,
                player_transform.translation.z + 16.0,
            );

            // Interpolation factor determins how quickly the camera catches up to the target
            let interpolation_factor = 10.0 * time.delta_seconds();

            // Use linear interpolation to smoothly update the camera position
            camera_transform.translation = camera_transform
                .translation
                .lerp(target_position, interpolation_factor.clamp(0.0, 1.0));
        }
    }
}
