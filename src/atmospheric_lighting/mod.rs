use bevy::prelude::*;
use bevy_atmosphere::prelude::*;

pub struct AtmosphericLighting;

impl Plugin for AtmosphericLighting {
    fn build(&self, app: &mut App) {
        app.insert_resource(bevy::pbr::DirectionalLightShadowMap { size: 4 * 1024 })
            .insert_resource(AtmosphereModel::new(Nishita {
                sun_position: Vec3::new(0.0, 1.0, 1.0),
                ..default()
            }))
            .add_plugins(AtmospherePlugin);
    }
}
