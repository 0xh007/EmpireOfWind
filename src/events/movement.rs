use bevy::prelude::*;
use bevy_xpbd_3d::math::*;

/// An event sent for a movement input action
#[derive(Event)]
pub enum MovementAction {
    Move(Vector2),
    Jump,
}
