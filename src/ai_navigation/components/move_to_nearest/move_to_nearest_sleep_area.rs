use std::marker::PhantomData;

use bevy::prelude::*;
use big_brain::prelude::*;

use crate::sleep_area::components::SleepArea;

use super::move_to_nearest::MoveToNearest;

#[derive(Clone, Component, Debug, Reflect, FromReflect, TypePath)]
#[reflect(Component, FromReflect)]
pub struct MoveToNearestSleepArea {
    #[reflect(ignore)]
    pub _marker: PhantomData<SleepArea>,
    pub speed: f32,
}

impl ActionBuilder for MoveToNearestSleepArea {
    fn build(&self, cmd: &mut Commands, action: Entity, _actor: Entity) {
        cmd.entity(action).insert(MoveToNearestSleepArea {
            _marker: PhantomData,
            speed: self.speed,
        });
    }
}
