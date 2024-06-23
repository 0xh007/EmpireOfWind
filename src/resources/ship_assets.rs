use bevy_asset_loader::asset_collection::AssetCollection;
use bevy::prelude::{Resource, Scene};
use bevy::asset::Handle;

#[derive(AssetCollection, Resource)]
pub struct ShipAssets {
    #[asset(path = "models/export/ship/carrack_2.glb#Scene0")]
    pub ship: Handle<Scene>,
}
