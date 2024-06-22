use bevy::math::Vec3;
use bevy_water::WaterParam;

pub fn get_water_height_at_position(pos: Vec3, water: &WaterParam) -> f32 {
    water.wave_point(pos).y
}
