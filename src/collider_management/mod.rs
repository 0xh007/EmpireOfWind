use bevy::app::{App, Plugin, Update};
use bevy::prelude::*;

use components::*;
use systems::*;

use crate::AppStates;

mod components;
mod systems;

/// Plugin for managing colliders within the game.
///
/// The `ColliderManagementPlugin` provides functionality for handling collider generation
/// and configuration for entities marked with the `ColliderMarker` component. It registers
/// the necessary components and sets up systems to process and configure colliders.
///
/// # Components
/// - `ColliderMarker`: Marks an entity for collider processing.
///
/// # Systems
/// - `read_colliders`: Processes entities marked with `ColliderMarker`, generates colliders
///   from meshes, and attaches necessary components such as `Collider` and `RigidBody::Kinematic`.
///
/// This plugin is added to the app during the application setup and is configured to
/// operate during the `AppStates::Running` state.
pub struct ColliderManagementPlugin;

impl Plugin for ColliderManagementPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ColliderMarker>()
            .add_systems(Update, read_colliders.run_if(in_state(AppStates::Running)));
    }
}