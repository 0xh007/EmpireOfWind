use bevy::prelude::*;

/// Represents the fatigue level and behavior of an entity.
///
/// The `Fatigue` component is used to manage the fatigue level of an entity in the game.
/// It indicates whether the entity is currently sleeping, how quickly the fatigue level
/// increases per second, and the current fatigue level of the entity.
///
/// # Fields
/// - `is_sleeping`: A boolean indicating whether the entity is currently sleeping.
/// - `per_second`: The rate at which the fatigue level increases per second.
/// - `level`: The current fatigue level of the entity.
///
/// # Usage
///
/// ## Example 1: Initializing an Entity with Fatigue
///
/// ```rust
/// use bevy::prelude::*;
/// use empire_of_wind::components::Fatigue;
///
/// fn spawn_entity(mut commands: Commands) {
///     commands.spawn((
///         Fatigue {
///             is_sleeping: false,
///             per_second: 1.0,
///             level: 0.0,
///         },
///         // Other components...
///     ));
/// }
/// ```
///
/// ## Example 2: Managing Fatigue in a System
///
/// ```rust
/// use bevy::prelude::{Query, Res, Time};
/// use empire_of_wind::components::Fatigue;
///
/// fn increase_fatigue(time: Res<Time>, mut fatigues: Query<&mut Fatigue>) {
///     for mut fatigue in &mut fatigues {
///         fatigue.level += fatigue.per_second * time.delta_seconds();
///         if fatigue.level >= 100.0 {
///             fatigue.level = 100.0;
///         }
///         println!("Fatigue level: {}", fatigue.level);
///     }
/// }
/// ```
///
/// ## Example 3: Using Fatigue with a Sleep System
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
#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct Fatigue {
    /// A boolean indicating whether the entity is currently sleeping.
    pub is_sleeping: bool,
    /// The rate at which the fatigue level increases per second.
    pub per_second: f32,
    /// The current fatigue level of the entity.
    pub level: f32,
}
