use bevy::asset::Handle;
use bevy::prelude::{Resource, Scene};
use bevy_asset_loader::asset_collection::AssetCollection;

/// A resource that contains handles to the ship assets used in the game.
///
/// This struct uses the `AssetCollection` trait from the `bevy_asset_loader` crate to automatically
/// load the assets during a configurable loading state. The assets are then available as resources
/// that can be used throughout the game.
///
/// # Fields
///
/// * `ship` - A handle to the ship scene asset.
#[derive(AssetCollection, Resource)]
pub struct ShipAssets {
    #[asset(path = "models/export/ship/carrack_2.glb#Scene0")]
    pub ship: Handle<Scene>,
}
