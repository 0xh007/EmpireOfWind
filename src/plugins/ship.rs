use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_xpbd_3d::prelude::*;

use crate::prelude::*;
use crate::resources::ship_assets::ShipAssets;
use crate::systems::{spawn_food, spawn_furniture, spawn_ship};

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.configure_loading_state(
            LoadingStateConfig::new(AppStates::AssetLoading).load_collection::<ShipAssets>(),
        )
            .add_systems(OnEnter(AppStates::Running), spawn_ship::spawn_ship)
            .add_systems(OnEnter(AppStates::Running), spawn_food::spawn_food)
            .add_systems(OnEnter(AppStates::Running), spawn_furniture::spawn_furniture);
    }
}
