use bevy::prelude::*;

mod camera;
mod building;
mod input;
mod player;
mod ui;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(camera::CameraPlugin)
            .add_plugins(ui::UiPlugin)
            .add_plugins(building::BuildingPlugin)
            .add_plugins(input::InputHandlePlugin)
        ;
    }
}