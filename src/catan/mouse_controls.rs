use std::slice::Windows;

use bevy::{input::mouse::MouseMotion, prelude::*, window::PrimaryWindow};

pub fn mouse_motion(mut motion_evr: EventReader<MouseMotion>) {
    for ev in motion_evr.iter() {
        println!("Mouse moved: X: {} px, Y: {} px", ev.delta.x, ev.delta.y);
    }
}

// TODO there can be more then one CursorMoved event per frame
// TODO I don't think it's necessary to do the calculation, more then once per frame
// we're not going for shooter precision here
pub fn non_rotating_camera_tile_collision(
    camera_query: Query<&Transform, With<Camera>>,
    mut cursor_moved_events: EventReader<CursorMoved>,
) {
    let camera_transform: &Transform = camera_query.get_single().expect("More then 1 camera");

    let camera_pos = camera_transform.translation;

    println!("Camera in 3D (World Space): {:?}", camera_pos);

    for event in cursor_moved_events.iter() {
        let mouse_position_screen = event.position;
        println!("Mouse Position (Screen Space): {:?}", mouse_position_screen);
    }
}
