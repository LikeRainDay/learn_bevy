#![allow(clippy::type_complexity)]

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;

use crate::scenes::SceneState;

mod plugins;
mod components;
mod resources;
mod scenes;


pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<SceneState>()
            .add_plugins((
                TilemapPlugin,
                plugins::GamePlugin,
                scenes::loading_scene::LoadingScenePlugin,
                scenes::splash_scene::SplashScenePlugin,
                scenes::game_scene::GameScenePlugin,


                // build_system::tilemap::TilePlugin,
                // resources::loading::LoadingPlugin,
                // resources::audio::InternalAudioPlugin,
                // plugins::helpers::tiled::TiledMapPlugin,
                // components::menu::MenuPlugin,
            ));

        #[cfg(debug_assertions)]
        {
            app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
        }
    }
}
