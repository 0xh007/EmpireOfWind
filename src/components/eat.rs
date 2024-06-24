use bevy::prelude::*;
use big_brain::prelude::*;

/// Represents the eating behavior of an entity.
///
/// The `Eat` component is used to manage the eating action of an entity in the game.
/// It defines when the entity will stop eating based on its hunger level and how quickly
/// the hunger level decreases while eating.
///
/// # Fields
/// - `until`: The hunger level at which the entity will stop eating. When the entity's
///   hunger level drops to or below this value, it will stop eating.
/// - `per_second`: The rate at which the hunger level decreases while the entity is eating.
///   This value represents the amount of hunger reduced per second.
///
/// # Usage
///
/// ## Example 1: Initializing an Entity with Eating Behavior
///
/// ```rust
/// use bevy::prelude::*;
/// use empire_of_wind::components::Eat;
///
/// fn spawn_entity(mut commands: Commands) {
///     commands.spawn((
///         Eat {
///             until: 10.0,
///             per_second: 5.0,
///         },
///         // Other components...
///     ));
/// }
/// ```
///
/// ## Example 2: Managing Eating Action in a System
///
/// ```rust
/// use bevy::prelude::{Query, Res, Time};
/// use big_brain::actions::ActionState;
/// use big_brain::prelude::{ActionSpan, Actor};
/// use empire_of_wind::components::{Eat, Hunger};
///
/// fn eat_action(
///     time: Res<Time>,
///     mut hungers: Query<&mut Hunger>,
///     mut query: Query<(&Actor, &mut ActionState, &Eat, &ActionSpan)>,
/// ) {
///     for (Actor(actor), mut state, eat, span) in &mut query {
///         let _guard = span.span().enter();
///
///         if let Ok(mut hunger) = hungers.get_mut(*actor) {
///             match *state {
///                 ActionState::Requested => {
///                     hunger.is_eating = true;
///                     *state = ActionState::Executing;
///                 }
///                 ActionState::Executing => {
///                     hunger.level -= eat.per_second * time.delta_seconds();
///
///                     if hunger.level <= eat.until {
///                         hunger.is_eating = false;
///                         *state = ActionState::Success;
///                     }
///                 }
///                 ActionState::Cancelled => {
///                     hunger.is_eating = false;
///                     *state = ActionState::Failure;
///                 }
///                 _ => {}
///             }
///         }
///     }
/// }
/// ```
///
/// ## Example 3: Adding Eating Behavior to an NPC
///
/// ```rust
/// use bevy::prelude::*;
/// use empire_of_wind::components::{Eat, Hunger};
///
/// fn spawn_npc(mut commands: Commands) {
///     commands.spawn((
///         Eat {
///             until: 10.0,
///             per_second: 5.0,
///         },
///         Hunger {
///             is_eating: false,
///             level: 50.0,
///             per_second: 2.0,
///         },
///         // Other components...
///     ));
/// }
/// ```
#[derive(Clone, Component, Debug, ActionBuilder, Reflect, Default)]
#[reflect(Component)]
pub struct Eat {
    /// The hunger level at which the entity will stop eating.
    pub until: f32,
    /// The rate at which the hunger level decreases while eating.
    pub per_second: f32,
}
