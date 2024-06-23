use crate::prelude::*;
use bevy::prelude::*;
use crate::systems::navigate_to_nearest;

const MAX_DISTANCE: f32 = 1.0;

pub struct PathfindingPlugin;

impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<NavigationPath>().add_systems(
            PreUpdate,
            (
                navigate_to_nearest::navigate_to_nearest::<SleepArea>,
                navigate_to_nearest::navigate_to_nearest::<Food>,
            ),
        );
    }
}
