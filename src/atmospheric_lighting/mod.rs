use bevy::pbr::*;
use bevy::prelude::*;
use bevy_atmosphere::prelude::*;

/// Plugin for managing atmospheric lighting within the game world.
///
/// The `AtmosphericLightingPlugin` integrates the `bevy_atmosphere` crate to provide
/// realistic atmospheric lighting effects. It sets up the necessary resources and plugins
/// to simulate atmospheric scattering and directional light shadows, enhancing the visual
/// quality of the game.
///
/// # Resources
/// - `DirectionalLightShadowMap`: Configures the size of the shadow map for the directional light.
/// - `AtmosphereModel`: Defines the atmospheric model used for rendering the sky and lighting effects.
///
/// # Plugins
/// - `AtmospherePlugin`: Adds the core atmospheric rendering capabilities from the `bevy_atmosphere` crate.
///
/// This plugin is added to the app during the application setup.
pub struct AtmosphericLightingPlugin;

impl Plugin for AtmosphericLightingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DirectionalLightShadowMap { size: 4 * 1024 })
            .insert_resource(AtmosphereModel::new(Nishita {
                sun_position: Vec3::new(0.0, 1.0, 1.0),
                ..default()
            }))
            .add_plugins(AtmospherePlugin);
    }
}
