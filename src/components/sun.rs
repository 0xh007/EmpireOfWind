use bevy::prelude::Component;

/// Marker component for identifying the sun entity in the game.
///
/// The `Sun` component is used to tag an entity as the sun in the game's world. This component
/// does not have any fields and is purely used for identification purposes by various systems
/// that need to interact with the sun entity.
#[derive(Component)]
pub struct Sun;
