use bevy::ecs::reflect::ReflectComponent;
use bevy::prelude::{Component, Reflect};
use serde::{Deserialize, Serialize};

/// The `AreaName` component is used to assign a name to specific areas within the game world.
///
/// This component stores the name of an area as a string and is utilized primarily in
/// collision detection and event handling to manage player interactions with different areas.
///
/// # Usage
///
/// The `AreaName` component is added to entities that represent distinct areas in the game. When
/// the player enters or exits these areas, the `AreaName` component is used to identify the
/// specific area and trigger appropriate responses such as updating the active areas and adjusting
/// camera settings.
///
/// # Fields
///
/// - `0`: A `String` representing the name of the area.
#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct AreaName(pub String);
