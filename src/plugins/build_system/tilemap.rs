use bevy::prelude::{App, AssetServer, Commands, Handle, OnEnter, Plugin, Res};

use crate::plugins::helpers;
use crate::SceneState;

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SceneState::GameScene), startup);
    }
}

/**
初始化地图
 */
fn startup(mut commands: Commands, assert_server: Res<AssetServer>) {
    let map_handle: Handle<helpers::tiled::TiledMap> = assert_server.load("first_building.tmx");
    let bundle = helpers::tiled::TiledMapBundle {
        tiled_map: map_handle,
        ..Default::default()
    };
    commands.spawn(bundle);
}
