use bevy::prelude::*;

/// Component for storing a set of points to navigate along
#[derive(Component, Debug, Clone)]
pub struct NavigationPath {
    pub points: Vec<Vec3>,
}

impl Default for NavigationPath {
    /// Sets up a NavigationPath with an empty path Vec
    fn default() -> Self {
        NavigationPath { points: Vec::new() }
    }
}
