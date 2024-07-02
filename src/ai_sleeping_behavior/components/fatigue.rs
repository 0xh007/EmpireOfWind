use bevy::prelude::*;

/// Represents the fatigue level and behavior of an entity.
///
/// The `Fatigue` component is used to manage the fatigue level of an entity in the game.
/// It indicates whether the entity is currently sleeping, how quickly the fatigue level
/// increases per second, and the current fatigue level of the entity.
///
/// # Fields
/// - `is_sleeping`: A boolean indicating whether the entity is currently sleeping.
/// - `per_second`: The rate at which the fatigue level increases per second.
/// - `level`: The current fatigue level of the entity.
#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct Fatigue {
    /// A boolean indicating whether the entity is currently sleeping.
    pub is_sleeping: bool,
    /// The rate at which the fatigue level increases per second.
    pub per_second: f32,
    /// The current fatigue level of the entity.
    pub level: f32,
}
