use bevy::prelude::*;
use big_brain::prelude::*;

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct Sleep {
    /// The fatigue level at which the entity will stop sleeping.
    pub until: f32,
    /// The rate at which the fatigue level decreases while sleeping.
    pub per_second: f32,
}
