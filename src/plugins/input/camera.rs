use bevy::input::touch;
use bevy::prelude::*;

use crate::plugins::camera::UiCamera;

const OPPOSITES_TOLERANCE: f32 = 0.0;
const MIN_SCALE: f32 = 0.001;
const MAX_SCALE: f32 = 100.0;
const ZOOM_SENSITIVITY: f32 = 0.005;
const DRAG_SENSITIVITY: f32 = 1.;
const TOUCH_TIME_MIN: f32 = 0.005;

#[derive(PartialEq, Default)]
enum GestureType {
    #[default]
    None,
    Pan,
    Pinch,
    PinchCancelled,
}

#[derive(Resource, Default)]
struct TouchTracker {
    pub camera_start_pos: Vec3,
    pub time_start_touch: f32,
    pub gesture_type: GestureType,

    // Keeps track of position on last frame.
    // This is different from Touch.last_position as that only updates when there has been a movement
    pub last_touch_a: Option<Vec2>,
    pub last_touch_b: Option<Vec2>,
}

pub fn camera_movement(
    mut camera_query: Query<(&mut Transform, &mut OrthographicProjection), With<UiCamera>>,
    time: Res<Time>,
    mut tracker: ResMut<TouchTracker>,
    touches_res: Res<Touches>,
) {
    let Ok((mut transform, mut projection)) = camera_query.get_single_mut() else {
        return;
    };

    let touches: Vec<&touch::Touch> = touches_res.iter().collect();

    if touches.is_empty() {
        tracker.gesture_type = GestureType::None;
        tracker.last_touch_a = None;
        tracker.last_touch_b = None;
        return;
    }

    if touches_res.any_just_released() {
        tracker.gesture_type = GestureType::PinchCancelled;
        tracker.last_touch_a = None;
        tracker.last_touch_b = None;
    }

    if touches.len() == 2 {
        tracker.gesture_type = GestureType::Pinch;
        let last_a = if tracker.last_touch_b.is_none() {
            touches[0].position();
        } else {
            tracker.last_touch_a.unwrap_or(touches[0].position());
        };

        let last_b = if tracker.last_touch_b.is_none() {
            touches[1].position();
        } else {
            tracker.last_touch_b.unwrap_or(touches[1].position());
        };
        let delta_a = touches[0].position() - last_a;
        let delta_b = touches[1].position() - last_b;
        let delta_total = (delta_a + delta_b).length();
        let dot_delta = delta_a.dot(delta_b);
        if dot_delta >  OPPOSITES_TOLERANCE{
            return;
        }

        let distance_current = touches[0].position() - touches[1].position();
        let distance_prev = touches[0].previous_position() - touches[1].previous_position();
        let pinch_direction = distance_prev.length() - distance_current.length();
        projection.scale +=
            pinch_direction.signum() * delta_total *ZOOM_SENSITIVITY* projection.scale;
        if projection.scale > MAX_SCALE{
            projection.scale =MAX_SCALE;
        }
        if projection.scale < MIN_SCALE {
            projection.scale = MIN_SCALE;
        }
        tracker.last_touch_a = Some(touches[0].position());
        tracker.last_touch_b = Some(touches[1].position());
    } else if touches.len() == 1
        && matches!(tracker.gesture_type, GestureType::None | GestureType::Pan)
    {
        if tracker.gesture_type == GestureType::None {
            tracker.camera_start_pos = transform.translation;
            tracker.time_start_touch = time.elapsed_seconds();
        }
        tracker.gesture_type = GestureType::Pan;
        let time_since_start = time.elapsed_seconds() - tracker.time_start_touch;
        if time_since_start <TOUCH_TIME_MIN{
            return;
        }
        let distance = Vec3::new(touches[0].distance().x, -touches[0].distance().y, 0.);
        transform.translation =
            tracker.camera_start_pos - DRAG_SENSITIVITY* distance * projection.scale;
        tracker.last_touch_a = Some(touches[0].position());
        tracker.last_touch_b = None;
    }
}