use bevy::input::ButtonInput;
use bevy::math::Vec3;
use bevy::prelude::{EventWriter, KeyCode, Query, Res};
use bevy_tnua::builtins::{TnuaBuiltinJump, TnuaBuiltinWalk};
use bevy_tnua::controller::TnuaController;

use crate::buoyancy_physics::{VisualizeMeshBoundsDebugToggle, VisualizeVoxelsDebugToggle};
use crate::navmesh::NavMeshDebugToggle;
use crate::player::Player;

/// Handles player input to control the player character's movement and actions.
///
/// This system listens for specific keyboard inputs to move the player character,
/// make it jump, and toggle the navigation mesh debug display.
///
/// - `W` or `ArrowUp`: Move forward.
/// - `S` or `ArrowDown`: Move backward.
/// - `A` or `ArrowLeft`: Move left.
/// - `D` or `ArrowRight`: Move right.
/// - `Space`: Make the player jump.
/// - `7`: Toggle debug visuals of voxel grid for buoyancy computation.
/// - `8`: Toggle debug visuals of mesh bounds finder.
/// - `9`: Toggle the navigation mesh debug display.
///
/// # Parameters
/// - `keyboard_input`: Resource capturing the current state of keyboard inputs.
/// - `nav_mesh_event_writer`: Writer to send `NavMeshDebugToggle` events.
/// - `visualize_mesh_event_writer`: Writer to send `VisualizeMeshBoundsDebugToggle` events.
/// - `visualize_voxels_event_writer`: Writer to send `VisualizeVoxelsDebugToggle` events.
/// - `query`: Query to fetch the `Player` and `TnuaController` components of entities.
pub fn handle_player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut nav_mesh_event_writer: EventWriter<NavMeshDebugToggle>,
    // mut visualize_mesh_event_writer: EventWriter<VisualizeMeshBoundsDebugToggle>,
    // mut visualize_voxels_event_writer: EventWriter<VisualizeVoxelsDebugToggle>,
    mut query: Query<(&Player, &mut TnuaController)>,
) {
    for (_, mut controller) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
            direction.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction.z += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            direction.z -= 1.0;
        }

        // Normalize the direction vector to ensure consistent movement speed in all directions
        if direction != Vec3::ZERO {
            direction = direction.normalize();
        }

        // Rotate the direction vector to align with the isometric perspective
        let rotation_angle = 45.0f32.to_radians(); // Convert 45 degrees to radians for isometric rotation
        let cos_angle = rotation_angle.cos();
        let sin_angle = rotation_angle.sin();

        // Rotate direction vector by the camera's rotation angle
        let rotated_direction = Vec3::new(
            direction.x * cos_angle - direction.z * sin_angle,
            0.0, // Y component remains 0 as we're rotating in the XZ plane
            direction.x * sin_angle + direction.z * cos_angle,
        );

        let desired_velocity = rotated_direction * 10.0; // Adjust speed as necessary

        controller.basis(TnuaBuiltinWalk {
            desired_velocity,
            float_height: 1.2,
            ..Default::default()
        });

        if keyboard_input.pressed(KeyCode::Space) {
            controller.action(TnuaBuiltinJump {
                height: 4.0,
                ..Default::default()
            });
        }

        if keyboard_input.pressed(KeyCode::Digit9) {
            nav_mesh_event_writer.send(NavMeshDebugToggle);
        }

        // if keyboard_input.pressed(KeyCode::Digit8) {
        //     visualize_mesh_event_writer.send(VisualizeMeshBoundsDebugToggle);
        // }
        // 
        // if keyboard_input.pressed(KeyCode::Digit7) {
        //     visualize_voxels_event_writer.send(VisualizeVoxelsDebugToggle);
        // }
    }
}
