use crate::prelude::*;
use bevy::prelude::*;

/// This plugin handles registering all of our component types so that we can inspect them in the
/// egui editor.
pub struct EditorTypesPlugin;

impl Plugin for EditorTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Fatigue>().register_type::<Sleep>();
    }
}
