use bevy::prelude::*;
use big_brain::prelude::*;

/// Component that will be attached to an actor entity when it's moving to a location.
#[derive(Clone, Component, Debug)]
pub struct MoveToNearest<T: Component + std::fmt::Debug + Clone> {
    pub _marker: std::marker::PhantomData<T>,
    pub speed: f32,
}

impl<T> ActionBuilder for MoveToNearest<T>
where
    T: Component + std::fmt::Debug + Clone,
{
    // An action builder needs to implement this method. It's used to attach the Action component
    // to the actor entity
    fn build(&self, cmd: &mut Commands, action: Entity, _actor: Entity) {
        cmd.entity(action).insert(MoveToNearest::<T>::clone(self));
    }
}
