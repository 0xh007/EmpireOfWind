use bevy::prelude::*;
use big_brain::prelude::*;

/// Represents the sleep state and behavior of an entity.
///
/// The `Sleep` component is used to manage the sleep behavior of an entity in the game.
/// It defines when the entity will stop sleeping based on its fatigue level and how quickly
/// the fatigue level decreases while sleeping.
///
/// # Fields
/// - `until`: The fatigue level at which the entity will stop sleeping. When the entity's
///   fatigue level drops to or below this value, it will wake up.
/// - `per_second`: The rate at which the fatigue level decreases while the entity is sleeping.
///   This value represents the amount of fatigue reduced per second.
#[derive(Clone, Component, Debug, ActionBuilder, Reflect, Default)]
#[reflect(Component)]
pub struct Sleep {
    /// The fatigue level at which the entity will stop sleeping.
    pub until: f32,
    /// The rate at which the fatigue level decreases while sleeping.
    pub per_second: f32,
}
