use bevy::prelude::*;

use crate::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, keyboard_input);
    }
}

/// Sends ['MovementAction'] event based on keyboard input
fn keyboard_input(
    mut movement_event_writer: EventWriter<MovementAction>,
    mut nav_mesh_event_writer: EventWriter<NavMeshDebugToggle>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let up = keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
    let down = keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);
    let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);

    let horizontal = right as i8 - left as i8;
    let vertical = up as i8 - down as i8;

    // Create a direction vector from the input
    let mut direction = Vec2::new(horizontal as f32, vertical as f32);

    if direction != Vec2::ZERO {
        // Normalize the direction to have a maximum length of 1
        direction = direction.normalize_or_zero();

        // Rotate the direction vector by +45 degrees to align with the isometric perspective
        let rotation_angle = 45.0f32.to_radians(); // Convert +45 degrees to radians
        let cos_angle = rotation_angle.cos();
        let sin_angle = rotation_angle.sin();
        let rotated_direction = Vec2::new(
            direction.x * cos_angle - direction.y * sin_angle,
            direction.x * sin_angle + direction.y * cos_angle,
        );

        movement_event_writer.send(MovementAction::Move(
            rotated_direction.clamp_length_max(1.0),
        ));
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        movement_event_writer.send(MovementAction::Jump);
    }

    if keyboard_input.just_pressed(KeyCode::KeyM) {
        nav_mesh_event_writer.send(NavMeshDebugToggle);
    }
}
