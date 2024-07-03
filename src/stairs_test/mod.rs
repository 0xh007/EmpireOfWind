use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::prelude::{ConfigureLoadingState, LoadingStateAppExt, LoadingStateConfig};

use crate::asset_management::AppStates;
use crate::ship::Ship;

pub struct StairsTestPlugin;

impl Plugin for StairsTestPlugin {
    fn build(&self, app: &mut App) {
        app.configure_loading_state(
            LoadingStateConfig::new(AppStates::AssetLoading).load_collection::<StairsTestLevelAssets>(),
        )
            .add_systems(OnEnter(AppStates::Running), spawn_test_level);
    }
}

#[derive(AssetCollection, Resource)]
pub struct StairsTestLevelAssets {
    #[asset(path = "models/export/test/navmeshtest.glb#Scene0")]
    pub stairs_test_level: Handle<Scene>,
}

pub fn spawn_test_level(mut commands: Commands, test_level_assets: Res<StairsTestLevelAssets>) {
    commands.spawn((
        Name::new("Test Level"),
        SceneBundle {
            scene: test_level_assets.stairs_test_level.clone(),
            ..default()
        },
    ));
}