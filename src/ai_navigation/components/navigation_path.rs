use bevy::prelude::*;

/// Component for storing a path of navigation points in 3D space.
///
/// Each `NavigationPath` contains a sequence of points (`Vec3`) that define
/// waypoints for navigation purposes.
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
