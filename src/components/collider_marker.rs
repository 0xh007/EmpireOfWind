use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// A marker component used to signify that an entity should be processed for colliders.
///
/// This component is added to entities that require collider generation. The system
/// looks for entities with this marker and performs necessary operations such as
/// generating colliders from meshes, attaching physics-related components, and
/// handling transformations. If the entity also has a `NavMeshMarker`, it will be
/// marked as an affector of the navigation mesh.
///
/// # Usages
/// - Adding the component to an entity marks it for collider processing.
/// - Used in systems to detect and handle collider setup for the entity.
#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct ColliderMarker;
