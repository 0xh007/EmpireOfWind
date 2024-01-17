use bevy::prelude::*;

pub struct Fatigue {
    /// A boolean indicating whether the entity is currently sleeping.
    pub is_sleeping: bool,
    /// The rate at which the fatigue level increases per second.
    pub per_second: f32,
    /// The current fatigue level of the entity.
    pub level: f32,
}
