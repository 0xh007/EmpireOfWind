use bevy::prelude::*;
use big_brain::prelude::*;

#[derive(Clone, Component, Debug, ActionBuilder, Reflect, Default)]
#[reflect(Component)]
pub struct Eat {
    /// The hunger level at which the entity will stop eating.
    pub until: f32,
    /// The rate at which the hunger level decreases while eating.
    pub per_second: f32,
}
