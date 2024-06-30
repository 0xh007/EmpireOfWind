use bevy::prelude::*;
use big_brain::prelude::*;
use serde::{Deserialize, Serialize};

/// This component serves as a scorer for evaluating the entity's need to eat based on its hunger
/// level.
///
/// The `HungerScorer` component is used to calculate a score that indicates the urgency for an
/// entity to perform the eating action. A higher score means a higher need to eat.
#[derive(
    Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default, ScorerBuilder,
)]
#[reflect(Component, Serialize, Deserialize)]
pub struct HungerScorer;
