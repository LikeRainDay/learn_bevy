use bevy::prelude::States;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum SceneState {
    #[default]
    LoadingScene,
    MainMenuScene,
    OptionsScene,
    HelpScene,
    HeroSelectScene,
    ResultScene,
    RewardScene,
}
