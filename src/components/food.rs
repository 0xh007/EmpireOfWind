use bevy::prelude::*;

/// Marker component for something edible.
///
/// The `Food` component is a marker component used to identify entities that can be consumed
/// by other entities. This component does not have any fields or behavior on its own.
/// It is typically used in systems where entities search for and interact with food items.
#[derive(Component, Debug, Clone)]
pub struct Food;
