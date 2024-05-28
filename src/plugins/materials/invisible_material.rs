use bevy::{
    pbr::MaterialPipeline,
    pbr::MaterialPipelineKey,
    prelude::*,
    reflect::TypePath,
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{AsBindGroup, ShaderRef},
        render_resource::RenderPipelineDescriptor,
        render_resource::SpecializedMeshPipelineError,
    },
};

// Define the InvisibleMaterial struct
#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub struct InvisibleMaterial {
    #[uniform(0)]
    pub color: Color,
}

// Implement the Material trait for InvisibleMaterial
impl Material for InvisibleMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/invisible_material.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        Ok(())
    }
}

// Define the plugin to register the material
pub struct InvisibleMaterialPlugin;

impl Plugin for InvisibleMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<InvisibleMaterial>::default());
    }
}
