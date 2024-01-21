use bevy::prelude::*;
use bevy::tasks::Task;

#[derive(Default, Resource)]
pub struct AsyncPathfindingTasks {
    tasks: Vec<Task<Option<Vec<Vec3>>>>,
}
