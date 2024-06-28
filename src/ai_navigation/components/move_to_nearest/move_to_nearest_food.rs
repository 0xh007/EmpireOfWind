use std::marker::PhantomData;

use bevy::prelude::*;
use big_brain::prelude::*;

use crate::food::components::Food;

use super::move_to_nearest::MoveToNearest;

#[derive(Clone, Component, Debug, Reflect, FromReflect, TypePath)]
#[reflect(Component, FromReflect)]
pub struct MoveToNearestFood {
    #[reflect(ignore)]
    pub _marker: PhantomData<Food>,
    pub speed: f32,
}

impl ActionBuilder for MoveToNearestFood {
    fn build(&self, cmd: &mut Commands, action: Entity, _actor: Entity) {
        cmd.entity(action).insert(MoveToNearestFood {
            _marker: PhantomData,
            speed: self.speed,
        });
    }
}
