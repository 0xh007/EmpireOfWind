use bevy::{prelude::*, render::view::RenderLayers};

#[derive(Event)]
pub struct EnterArea {
    pub area_name: String,
    pub render_layers: RenderLayers,
}
