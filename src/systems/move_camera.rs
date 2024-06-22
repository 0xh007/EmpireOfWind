use bevy::math::Vec3;
use bevy::prelude::{Query, Res, Time, Transform, With, Without};
use bevy_tnua::controller::TnuaController;

use crate::components::{MainCamera, Player};

pub fn move_camera(
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
