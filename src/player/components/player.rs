use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// A marker component indicating that an entity is the main player.
///
/// This component is used to distinguish the main player entity within the game world.
#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Player;
