use bevy::core_pipeline::Skybox;
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;
use bevy::render::render_resource::{TextureDescriptor, TextureDimension, TextureViewDescriptor};
use bevy_asset_loader::prelude::*;
use bevy_atmosphere::prelude::*;

const SKYBOX_NAME: &str =
    "textures/skybox/table_mountain_2_puresky/table_mountain_2_puresky_4k_cubemap.jpg";

const SPEED_MIN: f32 = 0.05;
const SPEED_DELTA: f32 = 0.01;
const SPEED_MAX: f32 = 1.0;

#[derive(AssetCollection, Resource)]
struct AtmosphereAssets {
    #[asset(
        path = "textures/skybox/table_mountain_2_puresky/table_mountain_2_puresky_4k_cubemap.jpg"
    )]
    skybox: Handle<TextureDescriptor>,
}

pub struct GameAtmospherePlugin;

impl Plugin for GameAtmospherePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(bevy::pbr::DirectionalLightShadowMap { size: 4 * 1024 })
            .insert_resource(AtmosphereModel::new(Nishita {
                sun_position: Vec3::new(0.0, 1.0, 1.0),
                ..default()
            }))
            .add_plugins(AtmospherePlugin);
    }
}

fn asset_loaded(
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut cubemap: ResMut<Cubemap>,
) {
    if !cubemap.is_loaded
        && asset_server.get_load_state(cubemap.image_handle.clone_weak()) == Some(LoadState::Loaded)
    {
        let image = images.get_mut(&cubemap.image_handle).unwrap();
        // NOTE: PNGs do not have any metadata that could indicate they contain a cubemap texture,
        // so they appear as one texture. The following code reconfigures the texture as necessary.
        if image.texture_descriptor.array_layer_count() == 1 {
            info!("Reinterpret 2D image {} into Cubemap", cubemap.name);
            image.reinterpret_stacked_2d_as_array(
                image.texture_descriptor.size.height / image.texture_descriptor.size.width,
            );
            image.texture_view_descriptor = Some(TextureViewDescriptor {
                dimension: Some(TextureViewDimension::Cube),
                ..default()
            });
        }

        cubemap.is_loaded = true;
    }
}
