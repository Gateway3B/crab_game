use bevy::prelude::*;
use bevy_asset_loader::prelude::{AssetCollection, LoadingState, LoadingStateAppExt};
use bevy_kira_audio::AudioSource;

use crate::*;

// region: Plugin
pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::MainLoading)
                .continue_to_state(GameState::MainMenu)
                .with_collection::<MusicAssets>()
                .with_collection::<Crab1Assets>(),
        );
    }
}

// endregion

// region: Structs

#[derive(AssetCollection, Resource)]
pub struct MusicAssets {
    #[asset(path = "sounds/Main Theme.mp3")]
    pub main_theme: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct Crab1Assets {
    #[asset(path = "models/Crab1.glb#Scene0")]
    pub model: Handle<Scene>,
    #[asset(
        paths(
            "models/Crab1.glb#Animation0",
            "models/Crab1.glb#Animation1",
            "models/Crab1.glb#Animation2",
            "models/Crab1.glb#Animation3",
            "models/Crab1.glb#Animation4",
            "models/Crab1.glb#Animation5",
            "models/Crab1.glb#Animation6",
        ),
        collection(typed)
    )]
    pub animations: Vec<Handle<AnimationClip>>,
}

// endregion
