use bevy::prelude::*;

#[derive(Event)]
pub struct ExitArea {
    pub area_name: String,
}
