use bevy::math::Vec3;
use bevy_water::WaterParam;

/// Retrieves the height of the water surface at a given position.
///
/// This function uses the `WaterParam` parameter from the `bevy_water` crate to
/// calculate the height of the water surface at the specified position, accounting
/// for waves and other water dynamics.
///
/// # Arguments
///
/// * `pos` - A `Vec3` representing the position in the game world where the water height is queried.
/// * `water` - A reference to the `WaterParam` struct, which contains global water settings and time resources.
///
/// # Returns
///
/// A `f32` value representing the height of the water surface at the specified position.
pub fn get_water_height_at_position(pos: Vec3, water: &WaterParam) -> f32 {
    water.wave_point(pos).y
}
