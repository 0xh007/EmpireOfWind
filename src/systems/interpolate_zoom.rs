use bevy::prelude::{Projection, Query, Res, Time, With};

use crate::components::camera_zoom::CameraZoom;
use crate::components::MainCamera;

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
