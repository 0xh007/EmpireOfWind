use bevy::prelude::Component;

/// Marker component for identifying the sun entity in the game.
///
/// The `Sun` component is used to tag an entity as the sun in the game's world. This component
/// does not have any fields and is purely used for identification purposes by various systems
/// that need to interact with the sun entity.
///
/// # Usage
///
/// ## Example 1: Adding the Sun Component to an Entity
///
/// ```rust
/// use bevy::prelude::*;
/// use empire_of_wind::components::Sun;
///
/// fn setup_sun(mut commands: Commands) {
///     commands.spawn(Sun);
/// }
/// ```
///
/// ## Example 2: Using the Sun Component in a System
///
/// ```rust
/// use bevy::prelude::{Entity, Query, With};
/// use empire_of_wind::components::Sun;
///
/// fn print_sun_entities(query: Query<Entity, With<Sun>>) {
///     for entity in query.iter() {
///         println!("Found a sun entity: {:?}", entity);
///     }
/// }
/// ```
#[derive(Component)]
pub struct Sun;
