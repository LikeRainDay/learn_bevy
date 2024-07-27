mod tilemap;
mod helper;
mod place_entity;

use bevy::prelude::*;
use crate::plugins::building::place_entity::CursorPos;
use crate::scenes::SceneState;

/**
建筑系统相关
 */
pub struct BuildingPlugin;


impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CursorPos>()
            .add_plugins(helper::TiledMapPlugin)
            .add_systems(OnEnter(SceneState::GameScene), tilemap::setup)
            .add_systems(Update, place_entity::update_cursor_pos.run_if(in_state(SceneState::GameScene)))
            .add_systems(Update, place_entity::place_entity.run_if(in_state(SceneState::GameScene)))
        ;
    }
}