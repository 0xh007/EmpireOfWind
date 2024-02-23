use bevy::prelude::*;
// use bevy::core_pipeline::Skybox;
use bevy::pbr::CascadeShadowConfigBuilder;
// use bevy_atmosphere::prelude::*;

pub struct AtmospherePlugin;

impl Plugin for AtmospherePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_atmosphere);
    }
}

fn spawn_atmosphere(mut commands: Commands) {
    let cascade_shadow_config = CascadeShadowConfigBuilder {
        first_cascade_far_bound: 0.3,
        maximum_distance: 3.0,
        ..default()
    }
    .build();

    // Sun
    commands.spawn((
        Name::new("Sun"),
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                color: Color::rgb(0.98, 0.95, 0.82),
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0)
                .looking_at(Vec3::new(-0.15, -0.05, -0.35), Vec3::Y),
            cascade_shadow_config,
            ..default()
        },
    ));
}
