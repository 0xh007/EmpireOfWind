use bevy::prelude::*;

#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct Hunger {
    /// A boolean indicating whether the entity is currently eating.
    pub is_eating: bool,
    /// The rate at which the hunger level increases per second.
    pub per_second: f32,
    /// The current hunger level of the entity.
    pub level: f32,
}
