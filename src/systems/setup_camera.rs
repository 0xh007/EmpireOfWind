use bevy::core::Name;
use bevy::core_pipeline::prepass::DepthPrepass;
use bevy::math::Vec3;
use bevy::pbr::{FogFalloff, FogSettings};
use bevy::prelude::{Camera, Camera3dBundle, Color, Commands, default, OrthographicProjection, Transform};
use bevy::render::camera::ScalingMode;
use bevy::render::view::RenderLayers;
use bevy_atmosphere::plugin::AtmosphereCamera;
use bevy_panorbit_camera::PanOrbitCamera;

use crate::components::{DebugCamera, MainCamera};
use crate::components::camera_zoom::CameraZoom;

pub fn setup_camera(mut commands: Commands) {
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
        RenderLayers::from_layers(&[0, 1, 2]), // Render both layers 0 and 1 initially
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
