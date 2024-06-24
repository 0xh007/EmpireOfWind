use bevy::prelude::*;

/// Represents the hunger level and behavior of an entity.
///
/// The `Hunger` component is used to manage the hunger level of an entity in the game.
/// It indicates whether the entity is currently eating, how quickly the hunger level
/// increases per second, and the current hunger level of the entity.
///
/// # Fields
/// - `is_eating`: A boolean indicating whether the entity is currently eating.
/// - `per_second`: The rate at which the hunger level increases per second.
/// - `level`: The current hunger level of the entity.
///
/// # Usage
///
/// ## Example 1: Initializing an Entity with Hunger
///
/// ```rust
/// use bevy::prelude::*;
/// use empire_of_wind::components::Hunger;
///
/// fn spawn_entity(mut commands: Commands) {
///     commands.spawn((
///         Hunger {
///             is_eating: false,
///             per_second: 1.0,
///             level: 0.0,
///         },
///         // Other components...
///     ));
/// }
/// ```
///
/// ## Example 2: Managing Hunger in a System
///
/// ```rust
/// use bevy::prelude::{Query, Res, Time};
/// use empire_of_wind::components::Hunger;
///
/// fn increase_hunger(time: Res<Time>, mut hungers: Query<&mut Hunger>) {
///     for mut hunger in &mut hungers {
///         hunger.level += hunger.per_second * time.delta_seconds();
///         if hunger.level >= 100.0 {
///             hunger.level = 100.0;
///         }
///         println!("Hunger level: {}", hunger.level);
///     }
/// }
/// ```
///
/// ## Example 3: Using Hunger with an Eat System
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
#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct Hunger {
    /// A boolean indicating whether the entity is currently eating.
    pub is_eating: bool,
    /// The rate at which the hunger level increases per second.
    pub per_second: f32,
    /// The current hunger level of the entity.
    pub level: f32,
}
