use bevy::core::Name;
use bevy::prelude::{App, Camera2dBundle, Commands, Component, Plugin, Startup};

#[derive(Component)]
pub struct UserInterfaceCamera;

#[derive(Component)]
pub struct OrthographicCamera;

pub struct CameraPlugin;

#[derive(Component)]
pub struct UiCamera;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_user_interface_camera);
        // app.add_systems(Startup, spawn_2d_camera);

        // 角色跟随


        // 界面滑动
    }
}

fn spawn_user_interface_camera(mut commands: Commands) {
    commands
        .spawn((
            Camera2dBundle::default(),
            UiCamera
        ))
        .insert(OrthographicCamera)
        .insert(UserInterfaceCamera);
}

fn spawn_2d_camera(mut commands: Commands){
    let mut camsera = Camera2dBundle::default();

    camsera.camera.order = 1;
    commands
        .spawn(camsera)
        .insert(OrthographicCamera)
        .insert(Name::new("OrthographicCamera"));
}

