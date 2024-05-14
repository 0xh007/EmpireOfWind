use bevy::prelude::*;
use bevy_tnua::prelude::*;

use crate::events::NavMeshDebugToggle;
use crate::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_controls.in_set(TnuaUserControlsSystemSet));
    }
}

fn apply_controls(
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
            println!("Jumping");
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
