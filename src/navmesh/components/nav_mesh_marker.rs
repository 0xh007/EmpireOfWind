use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Marker component to designate an entity that should influence the navigation mesh.
///
/// This component is used to mark entities that will affect the navigation mesh generation or modification.
/// Entities with this component are processed to include navigation mesh affector properties.
#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct NavMeshMarker;
