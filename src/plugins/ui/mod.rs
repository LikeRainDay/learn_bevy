pub mod home;

use std::fmt::Pointer;
use bevy::prelude::*;
use crate::scenes::SceneState;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(SceneState::GameScene), home::avatar_ui::setup)
            .add_systems(OnEnter(SceneState::GameScene), home::building_ui::setup)
            .add_systems(OnEnter(SceneState::GameScene), home::main_ui::setup)
            .add_systems(Update, home::main_ui::click_event);
    }
}