use bevy::{prelude::*, render::camera::ScalingMode, transform::TransformSystem};
use bevy::core_pipeline::prepass::DepthPrepass;
use bevy::render::view::RenderLayers;
use bevy_atmosphere::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_tnua::prelude::*;
use bevy_xpbd_3d::PhysicsSet;

use crate::prelude::*;

#[derive(Component)]
pub struct CameraZoom {
    pub target_scale: f32,
    pub current_scale: f32,
    pub speed: f32,
}

impl CameraZoom {
    fn new(target_scale: f32, current_scale: f32, speed: f32) -> Self {
        Self {
            target_scale,
            current_scale,
            speed,
        }
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_plugins(PanOrbitCameraPlugin)
            .add_systems(Update, camera_switching)
            .add_systems(Update, interpolate_zoom)
            .add_systems(
                PostUpdate,
                move_camera
                    .after(PhysicsSet::Sync)
                    .before(TransformSystem::TransformPropagate),
            );
    }
}

fn setup_camera(mut commands: Commands) {
    let initial_scale = 20.0;

    commands.spawn((
        Name::new("Main Camera"),
        Camera3dBundle {
            camera: Camera {
                order: 0,
                is_active: true,
                ..default()
            },
            projection: OrthographicProjection {
                scale: initial_scale, // Initial scale for zoom level
                scaling_mode: ScalingMode::FixedVertical(2.0),
                ..default()
            }
                .into(),
            transform: Transform::from_xyz(86.829, 90.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        RenderLayers::from_layers(&[0, 1]), // Render both layers 0 and 1 initially
        FogSettings {
            color: Color::rgba(0.1, 0.2, 0.4, 1.0),
            falloff: FogFalloff::from_visibility_colors(
                400.0,
                Color::rgb(0.35, 0.5, 0.66),
                Color::rgb(0.8, 0.844, 1.0),
            ),
            ..default()
        },
        MainCamera,
        DepthPrepass,
        AtmosphereCamera::default(),
        CameraZoom::new(initial_scale, initial_scale, 20.0), // Initialize CameraZoom
    ));

    commands.spawn((
        Name::new("Debug Camera"),
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
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Camera, &DebugCamera), Without<MainCamera>>,
    mut query_main: Query<(&mut Camera, &MainCamera), Without<DebugCamera>>,
) {
    if keyboard_input.just_pressed(KeyCode::Digit0) {
        for (mut camera, _) in query.iter_mut() {
            camera.is_active = !camera.is_active;
        }

        for (mut camera, _) in query_main.iter_mut() {
            camera.is_active = !camera.is_active;
        }
    }
}

fn move_camera(
    query: Query<&Transform, (With<TnuaController>, With<Player>)>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<TnuaController>)>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            // Adjust the camera offset for an isometric view
            // The exact values here might need tweaking based on your game's scale and desired view
            let camera_offset = Vec3::new(30.0, 50.0, 30.0); // Example isometric offset

            // Calculate the target position based on the player's position and the offset
            let target_position = player_transform.translation + camera_offset;

            // Interpolation factor for smooth camera movement
            let interpolation_factor = 10.0 * time.delta_seconds();

            // Smoothly interpolate the camera's position
            camera_transform.translation = camera_transform
                .translation
                .lerp(target_position, interpolation_factor.clamp(0.0, 1.0));

            // Maintain the camera's isometric perspective while following the player
            // This might require adjusting depending on your game's specific needs
            camera_transform.look_at(player_transform.translation, Vec3::Y);
        }
    }
}

fn interpolate_zoom(
    mut camera_zoom_query: Query<(&mut CameraZoom, &mut Projection), With<MainCamera>>,
    time: Res<Time>,
) {
    for (mut zoom, mut projection) in camera_zoom_query.iter_mut() {
        if let Projection::Orthographic(orthographic) = &mut *projection {
            let delta_scale = zoom.speed * time.delta_seconds();
            if (zoom.current_scale - zoom.target_scale).abs() < delta_scale {
                zoom.current_scale = zoom.target_scale;
            } else if zoom.current_scale < zoom.target_scale {
                zoom.current_scale += delta_scale;
            } else {
                zoom.current_scale -= delta_scale;
            }
            orthographic.scale = zoom.current_scale;
            println!("Interpolating zoom: current scale is {}", orthographic.scale);
        }
    }
}
