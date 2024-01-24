use bevy::prelude::*;
use big_brain::prelude::*;

/// This component serves as a scorer for evaluating the entity's need to eat based on its hunger
/// level.
#[derive(Clone, Component, Debug, ScorerBuilder)]
pub struct HungerScorer;
