use bevy::prelude::*;
use big_brain::prelude::*;

/// Represents the sleep state and behavior of an entity.
///
/// The `Sleep` component is used to manage the sleep behavior of an entity in the game.
/// It defines when the entity will stop sleeping based on its fatigue level and how quickly
/// the fatigue level decreases while sleeping.
///
/// # Fields
/// - `until`: The fatigue level at which the entity will stop sleeping. When the entity's
///   fatigue level drops to or below this value, it will wake up.
/// - `per_second`: The rate at which the fatigue level decreases while the entity is sleeping.
///   This value represents the amount of fatigue reduced per second.
///
/// # Usage
///
/// ## Example 1: Initializing an Entity with Sleep Behavior
///
/// ```rust
/// use bevy::prelude::*;
/// use empire_of_wind::components::Sleep;
///
/// fn spawn_entity(mut commands: Commands) {
///     commands.spawn((
///         Sleep {
///             until: 10.0,
///             per_second: 5.0,
///         },
///         // Other components...
///     ));
/// }
/// ```
///
/// ## Example 2: Managing Sleep in a System
///
/// ```rust
/// use bevy::prelude::{Query, Res, Time};
/// use big_brain::actions::ActionState;
/// use big_brain::prelude::{ActionSpan, Actor};
/// use empire_of_wind::components::{Fatigue, Sleep};
///
/// fn sleep_action(
///     time: Res<Time>,
///     mut fatigues: Query<&mut Fatigue>,
///     mut query: Query<(&Actor, &mut ActionState, &Sleep, &ActionSpan)>,
/// ) {
///     for (Actor(actor), mut state, sleep, span) in &mut query {
///         let _guard = span.span().enter();
///
///         if let Ok(mut fatigue) = fatigues.get_mut(*actor) {
///             match *state {
///                 ActionState::Requested => {
///                     fatigue.is_sleeping = true;
///                     *state = ActionState::Executing;
///                 }
///                 ActionState::Executing => {
///                     fatigue.level -= sleep.per_second * time.delta_seconds();
///
///                     if fatigue.level <= sleep.until {
///                         fatigue.is_sleeping = false;
///                         *state = ActionState::Success;
///                     }
///                 }
///                 ActionState::Cancelled => {
///                     fatigue.is_sleeping = false;
///                     *state = ActionState::Failure;
///                 }
///                 _ => {}
///             }
///         }
///     }
/// }
/// ```
///
/// ## Example 3: Adding Sleep Behavior to an NPC
///
/// ```rust
/// use bevy::prelude::*;
/// use empire_of_wind::components::{Sleep, Fatigue};
///
/// fn spawn_npc(mut commands: Commands) {
///     commands.spawn((
///         Sleep {
///             until: 10.0,
///             per_second: 5.0,
///         },
///         Fatigue {
///             is_sleeping: false,
///             per_second: 1.0,
///             level: 50.0,
///         },
///         // Other components...
///     ));
/// }
/// ```
#[derive(Clone, Component, Debug, ActionBuilder, Reflect, Default)]
#[reflect(Component)]
pub struct Sleep {
    /// The fatigue level at which the entity will stop sleeping.
    pub until: f32,
    /// The rate at which the fatigue level decreases while sleeping.
    pub per_second: f32,
}
