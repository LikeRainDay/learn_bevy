use bevy::asset::AssetServer;
use bevy::prelude::*;
use crate::plugins::building::helper;

/**
对tilemap相关的操作
 */

pub struct TileMap;


pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_handle: Handle<helper::TiledMap> = asset_server.load("first_building.tmx");

    commands.spawn(helper::TiledMapBundle {
        tiled_map: map_handle,
        ..Default::default()
    });
}
