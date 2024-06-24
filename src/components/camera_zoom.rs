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
///
/// # Usage
///
/// ## Example 1: Initializing the Camera with Zoom
///
/// ```rust
/// use bevy::prelude::*;
/// use empire_of_wind::components::CameraZoom;
///
/// fn setup_camera(mut commands: Commands) {
///     let initial_scale = 20.0;
///
///     commands.spawn((
///         Camera3dBundle {
///             projection: OrthographicProjection {
///                 scale: initial_scale,
///                 ..default()
///             }.into(),
///             ..default()
///         },
///         CameraZoom::new(initial_scale, initial_scale, 5.0), // Initialize CameraZoom
///     ));
/// }
/// ```
///
/// ## Example 2: Interpolating Zoom in a System
///
/// ```rust
/// use bevy::prelude::{Projection, Query, Res, Time, With};
/// use empire_of_wind::components::CameraZoom;
///
/// fn interpolate_zoom(
///     mut camera_zoom_query: Query<(&mut CameraZoom, &mut Projection), With<MainCamera>>,
///     time: Res<Time>,
/// ) {
///     for (mut zoom, mut projection) in camera_zoom_query.iter_mut() {
///         if let Projection::Orthographic(orthographic) = &mut *projection {
///             let delta_scale = zoom.speed * time.delta_seconds();
///             if (zoom.current_scale - zoom.target_scale).abs() < delta_scale {
///                 zoom.current_scale = zoom.target_scale;
///             } else if zoom.current_scale < zoom.target_scale {
///                 zoom.current_scale += delta_scale;
///             } else {
///                 zoom.current_scale -= delta_scale;
///             }
///             orthographic.scale = zoom.current_scale;
///         }
///     }
/// }
/// ```
///
/// ## Example 3: Adjusting Zoom Target Based on Events
///
/// ```rust
/// use bevy::prelude::*;
/// use empire_of_wind::components::CameraZoom;
/// use empire_of_wind::components::MainCamera;
///
/// fn manage_active_areas(
///     mut camera_zoom_query: Query<&mut CameraZoom, With<MainCamera>>,
///     player_in_area: bool,
/// ) {
///     if player_in_area {
///         update_zoom_target(&mut camera_zoom_query, 10.0); // Adjust zoom for entry
///     } else {
///         update_zoom_target(&mut camera_zoom_query, 20.0); // Adjust zoom for exit
///     }
/// }
///
/// fn update_zoom_target(
///     camera_zoom_query: &mut Query<&mut CameraZoom, With<MainCamera>>,
///     target_scale: f32,
/// ) {
///     for mut zoom in camera_zoom_query.iter_mut() {
///         zoom.target_scale = target_scale;
///         println!("Setting target zoom scale to {}", target_scale);
///     }
/// }
/// ```
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
