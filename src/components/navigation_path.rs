use bevy::prelude::*;

/// Component for storing a path of navigation points in 3D space.
///
/// Each `NavigationPath` contains a sequence of points (`Vec3`) that define
/// waypoints for navigation purposes.
///
/// # Example
///
/// ```
/// use bevy::prelude::*;
/// use empire_of_wind::components::NavigationPath;
///
/// fn main() {
///     // Create a new NavigationPath with some points
///     let path = NavigationPath {
///         points: vec![
///             Vec3::new(0.0, 0.0, 0.0),
///             Vec3::new(5.0, 0.0, 0.0),
///             Vec3::new(5.0, 5.0, 0.0),
///         ],
///     };
///
///     // Print the points in the navigation path
///     for point in &path.points {
///         println!("Point: {:?}", point);
///     }
/// }
/// ```
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct NavigationPath {
    /// The sequence of points that define the navigation path.
    pub points: Vec<Vec3>,
}

impl Default for NavigationPath {
    /// Creates a new `NavigationPath` with an empty path.
    fn default() -> Self {
        NavigationPath { points: Vec::new() }
    }
}
