use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct NavigationPath {
    pub points: Vec<Vec3>,
}

impl Default for NavigationPath {
    fn default() -> Self {
        NavigationPath { points: Vec::new() }
    }
}
