use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::AppStates;

mod states;

pub struct AssetManagementPlugin;

impl Plugin for AssetManagementPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppStates>().add_loading_state(
            LoadingState::new(AppStates::AssetLoading).continue_to_state(AppStates::Running),
        );
    }
}
