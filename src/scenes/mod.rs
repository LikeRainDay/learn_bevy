pub mod loading_scene;
pub mod splash_scene;
pub mod game_scene;

use bevy::prelude::States;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum SceneState {
    #[default]
    LoadingScene,
    SplashScene,
    GameScene,

    // 待处理
    MainMenuScene,
    OptionsScene,
    HelpScene,
    HeroSelectScene,
    ResultScene,
    RewardScene,
}
