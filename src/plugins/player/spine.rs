use bevy::asset::{Assets, AssetServer};
use bevy::prelude::*;
use bevy_spine::{SkeletonController, SkeletonData, Spine, SpineBundle, SpineReadyEvent};

pub fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut skeletons: ResMut<Assets<SkeletonData>>,
) {

    let skeleton = SkeletonData::new_from_json(
        asset_server.load("spineboy/export/spineboy-pro.json"),
        asset_server.load("spineboy/export/spineboy-pma.atlas"),
    );
    let skeleton_handle = skeletons.add(skeleton);

    commands.spawn(SpineBundle {
        skeleton: skeleton_handle.clone(),
        transform: Transform::from_xyz(0., -200., 10.),
        ..Default::default()
    });
}

pub fn on_spawn(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut spine_query: Query<&mut Spine>,
) {
    for event in spine_ready_event.read() {
        if let Ok(mut spine) = spine_query.get_mut(event.entity) {
            let Spine(SkeletonController {
                          skeleton,
                          animation_state,
                          ..
                      }) = spine.as_mut();
            skeleton.set_scale(Vec2::splat(0.1));
            let _ = animation_state.set_animation_by_name(0, "run", true);
        }
    }
}
