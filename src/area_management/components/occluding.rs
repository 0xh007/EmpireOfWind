use bevy::prelude::*;

// TODO: Add Docs
#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Occluding {
    pub areas: Vec<String>,
}