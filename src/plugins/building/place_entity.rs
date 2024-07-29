use std::slice::Windows;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_spine::{SkeletonData, SpineBundle};

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
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    asset_server: Res<AssetServer>,
    mut skeletons: ResMut<Assets<SkeletonData>>,
    tilemap_q: Query<(
        &TilemapGridSize,
        &TilemapSize,
        &TilemapType,
        &TileStorage,
        &Transform
    )>) {
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }
    let mut tile_pos: TilePos = Default::default();
    for (grid_size, size, tilemap_type, storage, transform) in tilemap_q.iter() {
        let cur_pos = cursor_pos.0;
        info!("cursor_pos: {:?}", cur_pos);

        let cursor_in_map_pos: Vec2 = {
            // Extend the cursor_pos vec3 by 0.0 and 1.0
            let cursor_pos = Vec4::from((cur_pos, 0.0, 1.0));
            let cursor_in_map_pos = transform.compute_matrix().inverse() * cursor_pos;
            cursor_in_map_pos.xy()
        };

        info!("cursor_in_map_pos: {:?}", cursor_in_map_pos);
        // Once we have a world position we can transform it into a possible tile position.
        if let Some(tile_pos_tmp) =
            TilePos::from_world_pos(&cursor_in_map_pos, size, grid_size, tilemap_type)
        {
            tile_pos = tile_pos_tmp;

            let center_pos = tile_pos_tmp.center_in_world(grid_size, tilemap_type);

            info!("left mouse currently pressed");
            let skeleton = SkeletonData::new_from_json(
                asset_server.load("spineboy/export/spineboy-pro.json"),
                asset_server.load("spineboy/export/spineboy-pma.atlas"),
            );
            let skeleton_handle = skeletons.add(skeleton);

            let center_pos_world: Vec2 = {
                // Extend the center_pos vec2 by 0.0 and 1.0
                let center_pos_extended = Vec4::from((center_pos, 0.0, 1.0));
                let center_pos_world = transform.compute_matrix() * center_pos_extended;
                center_pos_world.xy()
            };
            commands.spawn(SpineBundle {
                skeleton: skeleton_handle.clone(),
                transform: Transform::from_xyz(center_pos_world.x, center_pos_world.y, 10.0),
                ..Default::default()
            });
            info!(" added tile at global position: ({}, {})", center_pos.x, center_pos.y);
            info!("center_pos in world coordinates: {:?}", center_pos_world);
        }
    }
}

