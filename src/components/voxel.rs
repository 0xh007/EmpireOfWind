use bevy::prelude::Vec3;

#[derive(Debug, Clone, PartialEq)]
pub struct Voxel {
    pub position: Vec3,
    pub is_solid: bool,
}