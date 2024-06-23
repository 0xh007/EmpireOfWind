use std::collections::HashSet;

use bevy::prelude::Resource;

#[derive(Default, Resource)]
pub struct ActiveAreas(pub HashSet<String>);
