use std::marker::PhantomData;

use bevy::prelude::*;
use big_brain::prelude::*;

use super::seek_behavior::SeekBehavior;

/// Component for entities that seek the nearest `SleepArea` target.
///
/// This component is used in conjunction with an `ActionBuilder` to create actions
/// where an entity will navigate towards the nearest `SleepArea` target.
///
/// # Fields
/// - `_marker`: A phantom data marker to hold the type `SleepArea`.
/// - `speed`: The movement speed of the entity.
#[derive(Clone, Component, Debug, Reflect, FromReflect, TypePath)]
#[reflect(Component, FromReflect)]
pub struct SeekSleepAreaBehavior {
    #[reflect(ignore)]
    pub _marker: PhantomData<SleepArea>,
    pub speed: f32,
}

impl ActionBuilder for SeekSleepAreaBehavior {
    /// Attaches the `SeekSleepAreaBehavior` component to the specified actor entity.
    ///
    /// This method is used by the `ActionBuilder` trait to add the `SeekSleepAreaBehavior` component
    /// to an entity, enabling it to move towards the nearest `SleepArea` target.
    ///
    /// # Parameters
    /// - `cmd`: The `Commands` object used to issue commands to the ECS.
    /// - `action`: The entity representing the action.
    /// - `_actor`: The entity to which the action will be attached.
    fn build(&self, cmd: &mut Commands, action: Entity, _actor: Entity) {
        cmd.entity(action).insert(SeekSleepAreaBehavior {
            _marker: PhantomData,
            speed: self.speed,
        });
    }
}
