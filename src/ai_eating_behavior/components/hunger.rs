use bevy::prelude::*;

/// Represents the hunger level and behavior of an entity.
///
/// The `Hunger` component is used to manage the hunger level of an entity in the game.
/// It indicates whether the entity is currently eating, how quickly the hunger level
/// increases per second, and the current hunger level of the entity.
///
/// # Fields
/// - `is_eating`: A boolean indicating whether the entity is currently eating.
/// - `per_second`: The rate at which the hunger level increases per second.
/// - `level`: The current hunger level of the entity.
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
