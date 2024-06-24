use bevy::input::ButtonInput;
use bevy::math::Vec3;
use bevy::prelude::{EventWriter, KeyCode, Query, Res};
use bevy_tnua::builtins::{TnuaBuiltinJump, TnuaBuiltinWalk};
use bevy_tnua::controller::TnuaController;

use crate::components::Player;
use crate::events::NavMeshDebugToggle;

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
/// - `M`: Toggle the navigation mesh debug display.
///
/// # Parameters
/// - `keyboard_input`: Resource capturing the current state of keyboard inputs.
/// - `nav_mesh_event_writer`: Writer to send `NavMeshDebugToggle` events.
/// - `query`: Query to fetch the `Player` and `TnuaController` components of entities.
///
/// # Example Usage
/// The `handle_player_input` system should be added to your Bevy app like this:
/// ```rust
/// use bevy::prelude::*;
/// use empire_of_wind::systems::handle_player_input;
/// use empire_of_wind::components::Player;
/// use bevy_tnua::controller::TnuaController;
/// use empire_of_wind::events::NavMeshDebugToggle;
///
/// fn main() {
///     App::build()
///         .add_plugins(DefaultPlugins)
///         .add_event::<NavMeshDebugToggle>()
///         .add_system(handle_player_input.system())
///         .run();
/// }
///
/// fn setup(mut commands: Commands) {
///     commands.spawn((
///         Player,
///         TnuaController::default(),
///     ));
/// }
/// ```
pub fn handle_player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut nav_mesh_event_writer: EventWriter<NavMeshDebugToggle>,
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

        if keyboard_input.pressed(KeyCode::KeyM) {
            nav_mesh_event_writer.send(NavMeshDebugToggle);
        }
    }
}
