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
///
/// # Example
/// ```rust
/// use bevy::prelude::*;
/// use empire_of_wind::components::MoveToNearest;
/// use empire_of_wind::systems::{navigate_to_nearest, spawn_npc};
///
/// // Define a component that represents the target
/// #[derive(Component, Debug, Clone)]
/// struct TargetComponent;
///
/// fn setup(mut commands: Commands) {
///     commands.spawn((
///         MoveToNearest::<TargetComponent> {
///             _marker: std::marker::PhantomData,
///             speed: 1.5,
///         },
///         // other components...
///     ));
/// }
///
/// fn main() {
///     App::new()
///         .add_plugins(DefaultPlugins)
///         .add_startup_system(setup.system())
///         .add_system(navigate_to_nearest::<TargetComponent>.system())
///         .add_system(spawn_npc.system())
///         .run();
/// }
/// ```
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
