use bevy::prelude::*;
use big_brain::prelude::*;

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct Sleep;
// pub struct Sleep {
//     /// The fatigue level at which the entity will stop sleeping.
//     until: f32,
//     /// The rate at which the fatigue level decreases while sleeping.
//     per_second: f32,
// }
