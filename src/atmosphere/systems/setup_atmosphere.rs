use bevy::math::Quat;
use bevy::pbr::{DirectionalLight, DirectionalLightBundle};
use bevy::prelude::{Commands, default, Transform};
use bevy::render::view::RenderLayers;

use crate::prelude::*;

/// System that sets up the atmosphere by spawning a directional light entity representing the sun.
///
/// This system initializes the "sun" entity in the scene with a directional light and default
/// settings. The light is configured to cast shadows and is tagged with the `Sun` component for
/// identification by other systems.
///
/// # Parameters
/// - `commands`: A mutable reference to the `Commands` resource to issue commands for spawning entities.
pub fn setup_atmosphere(mut commands: Commands) {
    // "Sun"
    commands
        .spawn((
            DirectionalLightBundle {
                directional_light: DirectionalLight {
                    illuminance: 11127.65,
                    shadows_enabled: true,
                    ..default()
                },
                transform: Transform::from_rotation(Quat::from_rotation_x(-0.340)),
                ..default()
            },
            RenderLayers::all()
        ))
        .insert(Sun); // Marks the light as Sun
}
