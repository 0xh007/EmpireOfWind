use bevy::prelude::Component;

/// Manages the zoom functionality of the camera.
///
/// The `CameraZoom` component allows for smooth interpolation between different zoom levels
/// for the camera. It maintains the target zoom scale, the current zoom scale, and the speed
/// at which the zoom interpolation occurs.
///
/// # Fields
/// - `target_scale`: The desired zoom scale that the camera should move towards.
/// - `current_scale`: The current zoom scale of the camera.
/// - `speed`: The speed at which the camera zooms in or out to reach the target scale.
#[derive(Component)]
pub struct CameraZoom {
    pub target_scale: f32,
    pub current_scale: f32,
    pub speed: f32,
}

impl CameraZoom {
    /// Creates a new `CameraZoom` component.
    ///
    /// # Parameters
    /// - `target_scale`: The desired zoom scale that the camera should move towards.
    /// - `current_scale`: The initial zoom scale of the camera.
    /// - `speed`: The speed at which the camera zooms in or out to reach the target scale.
    ///
    /// # Returns
    /// A new instance of `CameraZoom`.
    pub fn new(target_scale: f32, current_scale: f32, speed: f32) -> Self {
        Self {
            target_scale,
            current_scale,
            speed,
        }
    }
}
