use bevy::{prelude::*, transform::TransformSystem};
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use bevy_xpbd_3d::PhysicsSet;

use crate::prelude::*;
use crate::systems::{camera_switching, interpolate_zoom, move_camera, setup_camera};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_plugins(PanOrbitCameraPlugin)
            .add_systems(Update, camera_switching)
            .add_systems(Update, interpolate_zoom)
            .add_systems(
                PostUpdate,
                move_camera
                    .after(PhysicsSet::Sync)
                    .before(TransformSystem::TransformPropagate),
            );
    }
}
