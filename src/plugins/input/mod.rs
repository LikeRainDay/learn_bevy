mod camera;

use bevy::prelude::*;
use crate::scenes::SceneState;

pub struct InputHandlePlugin;


impl Plugin for InputHandlePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(camera::TouchTracker::default())
            .add_systems(Update, camera::camera_movement.run_if(in_state(SceneState::GameScene)));
    }
}