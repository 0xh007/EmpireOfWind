use std::collections::HashSet;

use bevy::prelude::Resource;

/// A resource that tracks the currently active areas in the game.
///
/// The `ActiveAreas` resource contains a set of strings, each representing the name of an active area.
/// This resource is used by various systems to determine which areas are currently active, influencing
/// gameplay elements such as camera zoom and rendering layers.
#[derive(Default, Resource)]
pub struct ActiveAreas(pub HashSet<String>);
