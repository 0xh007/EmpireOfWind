use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Marker component for a place where a character can sleep.
///
/// The `SleepArea` component is used to designate areas or objects within the game world
/// where characters are allowed to sleep. It is typically added to entities like beds or
/// sleeping quarters.
#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct SleepArea;
