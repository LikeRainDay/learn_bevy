mod spine;

use bevy::prelude::*;
use bevy_spine::SpineSet;
use crate::scenes::SceneState;

pub struct PlayerHandlePlugin;

impl Plugin for PlayerHandlePlugin {
    fn build(&self, app: &mut App) {

        app
            .add_systems(OnEnter(SceneState::GameScene), spine::setup)
            .add_systems(Update, spine::on_spawn.in_set(SpineSet::OnReady).run_if(in_state(SceneState::GameScene)));
    }
}