use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};

#[derive(Asset, AsBindGroup, Clone, Debug, TypePath)]
pub struct InvisibleMaterial {
    #[uniform(0)]
    pub color: Color,
    pub alpha_mode: AlphaMode,
}

impl Material for InvisibleMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/transparent_blocking.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

pub struct InvisibleMaterialPlugin;

impl Plugin for InvisibleMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<InvisibleMaterial>::default());
    }
}
