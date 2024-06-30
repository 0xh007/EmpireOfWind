use bevy::prelude::*;
use big_brain::prelude::*;
use serde::{Deserialize, Serialize};

/// This component serves as a scorer for evaluating the entity's need to sleep based on its
/// fatigue level.
///
/// The `FatigueScorer` component is used to determine the priority of the sleeping action for
/// an entity by calculating a score that reflects the entity's current fatigue level.
#[derive(Debug, Clone, Component, Reflect, Serialize, Deserialize, Default, ScorerBuilder)]
#[reflect(Component, Serialize, Deserialize)]
pub struct FatigueScorer;
