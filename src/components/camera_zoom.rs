use bevy::prelude::Component;

#[derive(Component)]
pub struct CameraZoom {
    pub target_scale: f32,
    pub current_scale: f32,
    pub speed: f32,
}

impl CameraZoom {
    fn new(target_scale: f32, current_scale: f32, speed: f32) -> Self {
        Self {
            target_scale,
            current_scale,
            speed,
        }
    }
}
