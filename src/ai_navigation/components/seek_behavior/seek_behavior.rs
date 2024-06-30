use std::marker::PhantomData;

use bevy::prelude::*;
use big_brain::prelude::*;
use serde::{Deserialize, Serialize};

/// Component for entities that seek the nearest target of type `T`.
///
/// This component is used in conjunction with an `ActionBuilder` to create actions
/// where an entity will navigate towards and seek the nearest target of a specified type `T`.
///
/// # Type Parameters
/// - `T`: The type of the target component that the entity will seek. It must implement
///   the `Component`, `Debug`, and `Clone` traits.
///
/// # Fields
/// - `_marker`: A phantom data marker to hold the type `T`.
/// - `speed`: The movement speed of the entity.
#[derive(Debug, Clone, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct SeekBehavior<T: Component + std::fmt::Debug + Clone> {
    #[reflect(ignore)]
    pub _marker: PhantomData<T>,
    pub speed: f32,
}

impl<T> ActionBuilder for SeekBehavior<T>
where
    T: Component + std::fmt::Debug + Clone,
{
    /// Attaches the `SeekBehavior` component to the specified actor entity.
    ///
    /// This method is used by the `ActionBuilder` trait to add the `SeekBehavior` component
    /// to an entity, enabling it to seek the nearest target of type `T`.
    ///
    /// # Parameters
    /// - `cmd`: The `Commands` object used to issue commands to the ECS.
    /// - `action`: The entity representing the action.
    /// - `_actor`: The entity to which the action will be attached.
    fn build(&self, cmd: &mut Commands, action: Entity, _actor: Entity) {
        cmd.entity(action).insert(SeekBehavior::<T> {
            _marker: PhantomData,
            speed: self.speed,
        });
    }
}
