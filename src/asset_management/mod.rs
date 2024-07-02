use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub use states::app_states::AppStates;

pub mod states;

/// Plugin for managing the asset loading state flow within the game.
///
/// The `AssetManagementPlugin` sets up the state flow for loading assets using the `bevy_asset_loader` crate.
/// It initializes the application states and configures the loading state, ensuring that all assets are loaded
/// before transitioning to the main game state.
///
/// # States
/// - `AppStates::AssetLoading`: The state during which game assets are loaded.
/// - `AppStates::Running`: The main game state, entered after assets are loaded.
pub struct AssetManagementPlugin;

impl Plugin for AssetManagementPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppStates>().add_loading_state(
            LoadingState::new(AppStates::AssetLoading).continue_to_state(AppStates::Running),
        );
    }
}
