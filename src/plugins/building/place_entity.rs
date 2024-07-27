use bevy::color::palettes;
use bevy::prelude::*;
use bevy::utils::tracing;
use bevy_ecs_tilemap::prelude::*;
use bevy_inspector_egui::bevy_egui::systems::InputEvents;
use bevy_mod_picking::pointer::InputMove;
use log::log;

#[derive(Resource)]
pub struct CursorPos(Vec2);

impl Default for CursorPos {
    fn default() -> Self {
        Self(Vec2::new(-1000.0, -1000.0))
    }
}

pub fn update_cursor_pos(
    camera_q: Query<(&GlobalTransform, &Camera)>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut cursor_pos: ResMut<CursorPos>,
) {
    for cursor_move in cursor_moved_events.read() {
        for (cam_t, cam) in camera_q.iter() {
            if let Some(pos) = cam.viewport_to_world_2d(cam_t, cursor_move.position) {
                *cursor_pos = CursorPos(pos);
            }
        }
    }
}

pub fn place_entity(
    mut commands: Commands,
    cursor_pos: Res<CursorPos>,
    // input: Res<InputMove<MouseButton>>,
    tilemap_q: Query<(
        &TilemapGridSize,
        &TilemapSize,
        &TilemapType,
        &TileStorage,
        &Transform
    )>) {
    let tile_pos: TilePos;
    for (grid_size, size, tilemap_type, storage, transform) in tilemap_q.iter() {
        let cur_pos = cursor_pos.0;

        let cursor_in_map_pos: Vec2 = {
            // Extend the cursor_pos vec3 by 0.0 and 1.0
            let cursor_pos = Vec4::from((cur_pos, 0.0, 1.0));
            let cursor_in_map_pos = transform.compute_matrix().inverse() * cursor_pos;
            cursor_in_map_pos.xy()
        };
        // Once we have a world position we can transform it into a possible tile position.
        if let Some(tile_pos_tmp) =
            TilePos::from_world_pos(&cursor_in_map_pos, size, grid_size, tilemap_type)
        {
            tile_pos = tile_pos_tmp;
        }
    }


}

