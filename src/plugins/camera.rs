use bevy::{prelude::*, transform::TransformSystem};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_xpbd_3d::PhysicsSet;
// use bevy_xpbd_3d::{
//     math::*, parry::transformation::vhacd::VHACDParameters, prelude::*, SubstepSchedule, SubstepSet,
// };

use crate::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
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

fn setup_camera(mut commands: Commands) {
    // transform: Transform::from_xyz(0.0, 15.0, 0.0),
    let focus = Vec3::new(0.0, 8.0, 0.0);
    let camera_position = Vec3::new(28.0, 20., 0.0);

    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                order: 0,
                is_active: true,
                ..default()
            },
            transform: Transform::from_translation(camera_position).looking_at(focus, Vec3::Y),
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

fn move_camera(
    query: Query<&Transform, With<CharacterController>>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<CharacterController>)>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            // let camera_offset = Vec3::new(25.0, 12.0, 0.0); // Adjust as needed
            let camera_offset = Vec3::new(0.0, 12.0, 25.0); // Adjust as needed

            // Calculate the target position based on the player's position and the offset
            let target_position = player_transform.translation + camera_offset;

            // Interpolation factor
            let interpolation_factor = 10.0 * time.delta_seconds();

            // Smoothly interpolate the camera's position
            camera_transform.translation = camera_transform
                .translation
                .lerp(target_position, interpolation_factor.clamp(0.0, 1.0));

            // Calculate the desired up vector, which should be the global up vector
            let up = Vec3::Y;

            // Use the `look_at` method to point the camera towards the player while maintaining the up vector
            camera_transform.look_at(player_transform.translation, up);
        }
    }
}
