use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "123e4567-e89b-12d3-a456-426614174000"]
pub struct CustomMaterial {
    #[uniform(0)]
    pub color: Color,
}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }
}
