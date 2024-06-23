use bevy::prelude::*;
use bevy::utils::Duration;
use bevy_atmosphere::prelude::*;

use crate::resources::cycle_timer::CycleTimer;
use crate::systems::{control_cycle_timer, setup_atmosphere, update_sun_cycle};

pub struct SkyPlugin;

impl Plugin for SkyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(bevy::pbr::DirectionalLightShadowMap { size: 4 * 1024 })
            .insert_resource(AtmosphereModel::new(Nishita {
                sun_position: Vec3::new(0.0, 1.0, 1.0),
                ..default()
            }))
            .add_plugins(AtmospherePlugin)
            .insert_resource(CycleTimer::new(Duration::from_millis(1000), 0.2))
            .add_systems(Startup, setup_atmosphere::setup_atmosphere)
            .add_systems(Update, control_cycle_timer::control_cycle_timer)
            .add_systems(Update, update_sun_cycle::update_sun_cycle);
    }
}

