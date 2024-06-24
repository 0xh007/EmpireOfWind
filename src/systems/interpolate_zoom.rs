use bevy::prelude::{Projection, Query, Res, Time, With};

use crate::components::camera_zoom::CameraZoom;
use crate::components::MainCamera;

/// Smoothly interpolates the camera zoom level towards the target zoom level.
///
/// This system adjusts the camera's zoom level based on the `CameraZoom` component,
/// ensuring smooth transitions between different zoom levels. The interpolation
/// speed is controlled by the `speed` field in the `CameraZoom` component.
///
/// # Parameters
/// - `camera_zoom_query`: Query to fetch the `CameraZoom` and `Projection` components of the main camera.
/// - `time`: Resource providing the delta time for the game.
pub fn interpolate_zoom(
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
        }
    }
}
