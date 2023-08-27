use std::slice::Windows;

use bevy::{
    input::mouse::MouseMotion,
    prelude::{shape::Circle, *},
    window::PrimaryWindow,
};

use super::{circle::WorldCircle, map_tile::HexWorldTile};

pub fn mouse_motion(mut motion_evr: EventReader<MouseMotion>) {
    for ev in motion_evr.iter() {
        println!("Mouse moved: X: {} px, Y: {} px", ev.delta.x, ev.delta.y);
    }
}

// TODO there can be more then one CursorMoved event per frame
// TODO I don't think it's necessary to do the calculation, more then once per frame
// we're not going for shooter precision here
pub fn non_rotating_camera_tile_collision(
    // mut circle_query: Query<&mut Transform, With<WorldCircle>>,
    mut tile_query: Query<(&mut Transform, &HexWorldTile)>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // let camera_transform: &Transform = camera_query.get_single().expect("More then 1 camera");
    let last = cursor_moved_events.iter().last();
    // let circ = circle_query.get_single_mut();

    if let None = last {
        return;
    }

    // if let Err(_) = circ {
    //     return;
    // }

    let window = window_query.get_single().unwrap();

    // w_h / 2 -> y 0
    // w_w / 2 -> x 0

    // c_x 0 -> c_x - w_w/2
    // c_y 0 -> c_y - w_h/2

    let cursor_position: Vec2 = last.unwrap().position;
    // let mut circ_transform = circ.unwrap();

    let cursor_position = get_circle_position(cursor_position, window);

    let cursor_position = Vec3::new(cursor_position.x, cursor_position.y, 0.0);
    for (mut tile_transform, tile) in tile_query.iter_mut() {
        if tile_transform.translation.distance(cursor_position) <= 1.0 {
            tile_transform.translation.z = 0.5;
        } else {
            tile_transform.translation.z = 0.0;
        }
    }

    // let cursor_pos = event.position;
}

fn get_circle_position(cursor_position: Vec2, window: &Window) -> Vec3 {
    let scale_factor = 0.035;

    let w_width = window.width();
    let w_height = window.height();

    let circ_x = cursor_position.x - w_width / 2.;
    let circ_y = cursor_position.y - w_height / 2.;

    return Vec3::new(circ_x * scale_factor, -circ_y * scale_factor, 0.1);
}
