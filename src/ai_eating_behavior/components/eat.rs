use bevy::prelude::*;
use big_brain::prelude::*;

/// Represents the eating behavior of an entity.
///
/// The `Eat` component is used to manage the eating action of an entity in the game.
/// It defines when the entity will stop eating based on its hunger level and how quickly
/// the hunger level decreases while eating.
///
/// # Fields
/// - `until`: The hunger level at which the entity will stop eating. When the entity's
///   hunger level drops to or below this value, it will stop eating.
/// - `per_second`: The rate at which the hunger level decreases while the entity is eating.
///   This value represents the amount of hunger reduced per second.
#[derive(Clone, Component, Debug, ActionBuilder, Reflect, Default)]
#[reflect(Component)]
pub struct Eat {
    /// The hunger level at which the entity will stop eating.
    pub until: f32,
    /// The rate at which the hunger level decreases while eating.
    pub per_second: f32,
}
