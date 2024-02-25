use crate::prelude::*;
// use bevy::core_pipeline::experimental::taa::{TemporalAntiAliasBundle, TemporalAntiAliasPlugin};
use bevy::core_pipeline::prepass::DepthPrepass;
use bevy::core_pipeline::Skybox;
use bevy::{prelude::*, render::camera::ScalingMode, transform::TransformSystem};
use bevy_atmosphere::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
// TODO: Figure out what this is doing so we're not depending on a water plugin for our main camera
use bevy_water::{ImageReformat, ImageUtilsPlugin};
use bevy_xpbd_3d::PhysicsSet;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_plugins(ImageUtilsPlugin)
            .add_plugins(PanOrbitCameraPlugin)
            .add_systems(Update, camera_switching)
            .add_systems(
                PostUpdate,
                move_camera
                    .after(PhysicsSet::Sync)
                    .before(TransformSystem::TransformPropagate),
            );
    }
}

fn setup_camera(mut commands: Commands, asset_server: Res<AssetServer>) {
    // TODO: Move skybox stuff into it's own plugin
    let skybox_name: &str =
        "textures/skybox/table_mountain_2_puresky/table_mountain_2_puresky_4k_cubemap.jpg";

    let skybox_handle = ImageReformat::cubemap(&mut commands, &asset_server, skybox_name);
    commands.spawn((
        Name::new("Main Camera"),
        Camera3dBundle {
            camera: Camera {
                order: 1,
                is_active: true,
                ..default()
            },
            projection: OrthographicProjection {
                scale: 25.0,
                scaling_mode: ScalingMode::FixedVertical(2.0),
                ..default()
            }
            .into(),
            transform: Transform::from_xyz(86.829, 90.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        FogSettings {
            color: Color::rgba(0.1, 0.2, 0.4, 1.0),
            //directional_light_color: Color::rgba(1.0, 0.95, 0.75, 0.5),
            //directional_light_exponent: 30.0,
            falloff: FogFalloff::from_visibility_colors(
                400.0, // distance in world units up to which objects retain visibility (>= 5% contrast)
                Color::rgb(0.35, 0.5, 0.66), // atmospheric extinction color (after light is lost due to absorption by atmospheric particles)
                Color::rgb(0.8, 0.844, 1.0), // atmospheric inscattering color (light gained due to scattering from the sun)
            ),
            ..default()
        },
        // TemporalAntiAliasBundle::default(),
        MainCamera,
        // DepthPrepass,
        DepthPrepass,
        AtmosphereCamera::default(),
        Skybox(skybox_handle),
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

fn move_camera(
    query: Query<&Transform, (With<CharacterController>, With<Player>)>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<CharacterController>)>,
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
