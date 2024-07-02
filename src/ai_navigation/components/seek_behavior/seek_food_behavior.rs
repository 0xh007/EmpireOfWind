use std::marker::PhantomData;

use bevy::prelude::*;
use big_brain::prelude::*;
use serde::{Deserialize, Serialize};

use crate::food::Food;

/// Component for entities that seek the nearest `Food` target.
///
/// This component is used in conjunction with an `ActionBuilder` to create actions
/// where an entity will navigate towards the nearest `Food` target.
///
/// # Fields
/// - `_marker`: A phantom data marker to hold the type `Food`.
/// - `speed`: The movement speed of the entity.
#[derive(Debug, Clone, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct SeekFoodBehavior {
    #[reflect(ignore)]
    pub _marker: PhantomData<Food>,
    pub speed: f32,
}

impl ActionBuilder for SeekFoodBehavior {
    /// Attaches the `SeekFoodBehavior` component to the specified actor entity.
    ///
    /// This method is used by the `ActionBuilder` trait to add the `SeekFoodBehavior` component
    /// to an entity, enabling it to move towards the nearest `Food` target.
    ///
    /// # Parameters
    /// - `cmd`: The `Commands` object used to issue commands to the ECS.
    /// - `action`: The entity representing the action.
    /// - `_actor`: The entity to which the action will be attached.
    fn build(&self, cmd: &mut Commands, action: Entity, _actor: Entity) {
        cmd.entity(action).insert(SeekFoodBehavior {
            _marker: PhantomData,
            speed: self.speed,
        });
    }
}
