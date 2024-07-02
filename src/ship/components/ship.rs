use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// A marker component indicating that an entity is a ship.
///
/// This component is used to distinguish entities that represent ships within the game world.
#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Ship;
