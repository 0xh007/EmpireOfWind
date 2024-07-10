use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// TODO: Add Docs
#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Occluding {
    pub areas: Vec<String>,
}