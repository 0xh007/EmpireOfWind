use bevy::prelude::*;
use bevy::pbr::{CascadeShadowConfigBuilder, NotShadowCaster};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

const SHIP_LENGTH: i32 = 40;
const SHIP_WIDTH: i32 = 10;
const SHIP_HEIGHT: i32 = 8;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_ship)
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
    ));
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
                //D18251
                // Check if the current position is on the boundary
                if x == 0 || x == SHIP_LENGTH - 1 || y == 0 || y == SHIP_WIDTH - 1 || z == 0 || z == SHIP_HEIGHT - 1 {
                    commands.spawn(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: cube_size })),
                        // material: materials.add(Color::rgb_u8(124, 144, 255).into()),
                        material: materials.add(Color::hex("D18251").unwrap().into()),
                        transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                        ..default()
                    });
                }
            }
        }
    }
}
