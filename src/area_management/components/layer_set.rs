use bevy::prelude::*;
use bevy::render::view::RenderLayers;

// TODO: Add docs
#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct LayerSet {
    pub layers: Vec<u8>,
}

impl LayerSet {
    pub fn to_render_layers(&self) -> RenderLayers {
        RenderLayers::from_layers(&self.layers)
    }
}