use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// TODO: Review docs
/// A marker component used to designate an area that a player can enter or detect entities within.
///
/// The `AreaMarker` component is used to identify entities that represent
/// areas in the game world where specific actions should be taken when a player
/// enters them. It also allows for detecting entities within the area. It works in
/// conjunction with the `manage_active_areas` system to handle area entry and exit events.
///
/// # Usages
/// - Adding the component to an entity marks it as an area.
/// - Used in systems to detect when a player or other entities enter the marked area.
///
/// # Fields
/// - `name`: A `String` representing the name of the area.
#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct AreaMarker {
    pub name: String,
}
