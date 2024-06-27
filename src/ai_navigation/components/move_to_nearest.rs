use bevy::prelude::*;
use big_brain::prelude::*;

/// Component for entities that move to the nearest target of type `T`.
///
/// This component is used in conjunction with an `ActionBuilder` to create actions
/// where an entity will navigate towards the nearest target of a specified type `T`.
///
/// # Type Parameters
/// - `T`: The type of the target component that the entity will move towards. It must implement
///   the `Component`, `Debug`, and `Clone` traits.
///
/// # Fields
/// - `_marker`: A phantom data marker to hold the type `T`.
/// - `speed`: The movement speed of the entity.
#[derive(Clone, Component, Debug)]
pub struct MoveToNearest<T: Component + std::fmt::Debug + Clone> {
    pub _marker: std::marker::PhantomData<T>,
    pub speed: f32,
}

impl<T> ActionBuilder for MoveToNearest<T>
where
    T: Component + std::fmt::Debug + Clone,
{
    /// Attaches the `MoveToNearest` component to the specified actor entity.
    ///
    /// This method is used by the `ActionBuilder` trait to add the `MoveToNearest` component
    /// to an entity, enabling it to move towards the nearest target of type `T`.
    ///
    /// # Parameters
    /// - `cmd`: The `Commands` object used to issue commands to the ECS.
    /// - `action`: The entity representing the action.
    /// - `_actor`: The entity to which the action will be attached.
    fn build(&self, cmd: &mut Commands, action: Entity, _actor: Entity) {
        cmd.entity(action).insert(MoveToNearest::<T>::clone(self));
    }
}
