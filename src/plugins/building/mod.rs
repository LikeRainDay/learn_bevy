mod tilemap;
mod helper;

use bevy::prelude::*;
use crate::scenes::SceneState;

/**
建筑系统相关
 */
pub struct BuildingPlugin;


impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(helper::TiledMapPlugin)
            .add_systems(OnEnter(SceneState::GameScene), tilemap::setup)
        ;
    }
}