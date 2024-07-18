use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::prelude::{ConfigureLoadingState, LoadingState};
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

use crate::scenes::SceneState;

pub struct LoadingScenePlugin;

impl Plugin for LoadingScenePlugin {
    fn build(&self, app: &mut App) {
        let loading_state = LoadingState::new(SceneState::LoadingScene)
            .continue_to_state(SceneState::SplashScene)
            .load_collection::<AudioAssets>()
            .load_collection::<TextureAssets>();

        app.add_loading_state(loading_state);
    }
}

// 声明加载资源
#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub bevy: Handle<Image>,
    #[asset(path = "textures/github.png")]
    pub github: Handle<Image>,
}
