use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// A marker component used to signify that an entity should be processed for buoyancy.
///
/// This component is added to entities that require buoyancy calculations. The system
/// looks for entities with this marker and performs necessary operations such as
/// generating voxel grids, attaching buoyancy-related components, and handling
/// mesh transformations.
#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct BuoyancyMarker;
