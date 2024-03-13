use bevy::prelude::*;
use bevy_tnua::prelude::*;

use crate::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_controls);
    }
}

fn apply_controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut TnuaController>,
) {
    let mut controller = match query.get_single_mut() {
        Ok(controller) => controller,
        Err(_) => return,
    };

    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
        direction -= Vec3::Z;
    }
    if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
        direction += Vec3::Z;
    }
    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction -= Vec3::X;
    }
    if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        direction += Vec3::X;
    }

    controller.basis(TnuaBuiltinWalk {
        desired_velocity: direction.normalize_or_zero() * 10.0,
        float_height: 1.5, // Example adjustment, set based on your character's size and needs
        cling_distance: 3.0, // Adjust based on testing
        spring_strengh: 500.0, // Increased strength for better grounding
        spring_dampening: 1.2, // Balance to avoid instability
        acceleration: 60.0, // Keep or adjust based on movement feel
        air_acceleration: 30.0, // Adjusted for better air control, if necessary
        free_fall_extra_gravity: 70.0, // Adjust to help with slope sliding
        ..default()
    });

    if keyboard_input.pressed(KeyCode::Space) {
        controller.action(TnuaBuiltinJump {
            height: 4.0,
            ..Default::default()
        });
    }
}

// fn keyboard_input(
//     mut movement_event_writer: EventWriter<MovementAction>,
//     mut nav_mesh_event_writer: EventWriter<NavMeshDebugToggle>,
//     keyboard_input: Res<ButtonInput<KeyCode>>,
// ) {
//     let up = keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
//     let down = keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);
//     let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
//     let right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);
//
//     let horizontal = right as i8 - left as i8;
//     let vertical = up as i8 - down as i8;
//
//     // Create a direction vector from the input
//     let mut direction = Vec2::new(horizontal as f32, vertical as f32);
//
//     if direction != Vec2::ZERO {
//         // Normalize the direction to have a maximum length of 1
//         direction = direction.normalize_or_zero();
//
//         // Rotate the direction vector by +45 degrees to align with the isometric perspective
//         let rotation_angle = 45.0f32.to_radians(); // Convert +45 degrees to radians
//         let cos_angle = rotation_angle.cos();
//         let sin_angle = rotation_angle.sin();
//         let rotated_direction = Vec2::new(
//             direction.x * cos_angle - direction.y * sin_angle,
//             direction.x * sin_angle + direction.y * cos_angle,
//         );
//
//         movement_event_writer.send(MovementAction::Move(
//             rotated_direction.clamp_length_max(1.0),
//         ));
//     }
//
//     if keyboard_input.just_pressed(KeyCode::Space) {
//         movement_event_writer.send(MovementAction::Jump);
//     }
//
//     if keyboard_input.just_pressed(KeyCode::KeyM) {
//         nav_mesh_event_writer.send(NavMeshDebugToggle);
//     }
// }
