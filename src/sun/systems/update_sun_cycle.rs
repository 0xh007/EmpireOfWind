use bevy::math::Quat;
use bevy::pbr::light_consts::lux::AMBIENT_DAYLIGHT;
use bevy::pbr::DirectionalLight;
use bevy::prelude::{Query, Res, ResMut, Time, Transform, With};
use bevy_atmosphere::prelude::{AtmosphereMut, Nishita};

use crate::sun::components::Sun;
use crate::sun::resources::SunCycleTimer;

/// System that updates the sun's position and lighting based on the cycle timer.
///
/// This system modifies the `Atmosphere` resource and adjusts the position and illuminance of the
/// directional light representing the sun. It uses the `SunCycleTimer` resource to determine when
/// and how these updates should be applied.
///
/// # Parameters
/// - `atmospheric_lighting`: A mutable reference to the `Atmosphere` resource that will be updated.
/// - `query`: A query to get the `Transform` and `DirectionalLight` components of the sun entity.
/// - `timer`: A mutable reference to the `SunCycleTimer` resource to manage the day/night cycle timing.
/// - `time`: A reference to the `Time` resource to get the elapsed time since the last update.
pub fn update_sun_cycle(
    mut atmosphere: AtmosphereMut<Nishita>,
    mut query: Query<(&mut Transform, &mut DirectionalLight), With<Sun>>,
    mut timer: ResMut<SunCycleTimer>,
    time: Res<Time>,
) {
    // Do nothing if timer is paused.
    if timer.paused() {
        return;
    }

    timer.tick(time.delta());

    if timer.update() {
        let mut pos = atmosphere.sun_position;
        let t = (timer.time() + 3.0) * 0.1;
        pos.y = t.sin();
        pos.z = t.cos();
        atmosphere.sun_position = pos;

        if let Some((mut light_trans, mut directional)) = query.single_mut().into() {
            light_trans.rotation = Quat::from_rotation_x(-pos.y.atan2(pos.z));
            directional.illuminance = t.sin().max(0.0).powf(2.0) * AMBIENT_DAYLIGHT;
        }
    }
}
