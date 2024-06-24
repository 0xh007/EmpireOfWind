use std::collections::HashSet;

use bevy::prelude::Resource;

/// A resource that tracks the currently active areas in the game.
///
/// The `ActiveAreas` resource contains a set of strings, each representing the name of an active area.
/// This resource is used by various systems to determine which areas are currently active, influencing
/// gameplay elements such as camera zoom and rendering layers.
///
/// ### Example:
///
/// ```
/// use bevy::prelude::ResMut;
/// use empire_of_wind::resources::active_areas::ActiveAreas;
///
/// fn some_system(mut active_areas: ResMut<ActiveAreas>) {
///     // Add an area to the set of active areas
///     active_areas.0.insert(String::from("New Area"));
///
///     // Remove an area from the set of active areas
///     active_areas.0.remove("Old Area");
/// }
/// ```
#[derive(Default, Resource)]
pub struct ActiveAreas(pub HashSet<String>);
