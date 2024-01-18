use bevy::prelude::*;
use big_brain::prelude::*;

/// This component serves as a scorer for evaluating the entity's need to sleep based on its
/// fatigue level.
#[derive(Clone, Component, Debug, ScorerBuilder, Reflect, Default)]
#[reflect(Component)]
pub struct FatigueScorer;
