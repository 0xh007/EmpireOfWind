use bevy::prelude::*;
use bevy::render::view::RenderLayers;

// TODO: ADD DOCS
/// Enum representing possible layers for better readability and type safety.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TypePath, FromReflect)] // Step 2: Derive the traits
pub enum Layer {
    Layer0,
    Layer1,
    Layer2,
    Layer3,
    Layer4,
    Layer5,
    Layer6,
    Layer7,
    Layer8,
    Layer9,
    Layer10,
    Layer11,
    Layer12,
    Layer13,
    Layer14,
    Layer15,
    Layer16,
    Layer17,
    Layer18,
    Layer19,
    Layer20,
    Layer21,
    Layer22,
    Layer23,
    Layer24,
    Layer25,
    Layer26,
    Layer27,
    Layer28,
    Layer29,
    Layer30,
    Layer31,
}

impl Layer {
    /// Convert `Layer` to `u8` value.
    pub fn to_u8(self) -> u8 {
        self as u8
    }
}

/// A component to specify which rendering layers an entity belongs to in a user-friendly way.
#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct LayerSet {
    pub layers: Vec<Layer>,
}

impl LayerSet {
    /// Convert the `LayerSet` to a `RenderLayers` component.
    pub fn to_render_layers(&self) -> RenderLayers {
        let layer_values = self
            .layers
            .iter()
            .map(|layer| layer.to_u8())
            .collect::<Vec<u8>>();
        RenderLayers::from_layers(&layer_values)
    }
}
