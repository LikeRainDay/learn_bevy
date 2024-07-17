use bevy::color::Color;
use bevy::prelude::*;
use bevy::ui::{BackgroundColor, Style, Val};

use crate::scenes::SceneState;

#[derive(Resource)]
struct LoadingSceneData {
    user_interface_root: Entity,
}

pub struct LoadingScenePlugin;

impl Plugin for LoadingScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SceneState::LoadingScene), setup);
        // TODO 增加中间状态进行加载资源
        app.add_systems(OnExit(SceneState::LoadingScene), cleanup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let user_interface_root = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..Default::default()
        })
        .with_children(|parent| {
            loader_bundle(parent, &asset_server);
        })
        .id();

    commands
        .insert_resource(LoadingSceneData {
            user_interface_root
        });
}


fn loader_bundle(
    root: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
) {
    root.spawn(
        // Border
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..Default::default()
        });
}

fn cleanup(mut commands: Commands, loading_scene_data: Res<LoadingSceneData>) {
    commands
        .entity(loading_scene_data.user_interface_root)
        .despawn_recursive()
}

