use bevy::{prelude::*, window::PrimaryWindow};

use crate::catan::world::land_tile::LandTile;

// TODO there can be more then one CursorMoved event per frame
// TODO I don't think it's necessary to do the calculation, more then once per frame
// we're not going for shooter precision here
pub fn _non_rotating_camera_tile_collision(
    mut tile_query: Query<(&mut Transform, &LandTile)>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let last = cursor_moved_events.iter().last();

    if let None = last {
        return;
    }

    let window = window_query.get_single().unwrap();

    let cursor_position: Vec2 = last.unwrap().position;

    let cursor_position = _get_circle_position(cursor_position, window);

    let cursor_position = Vec3::new(cursor_position.x, cursor_position.y, 0.0);
    for (mut tile_transform, _) in tile_query.iter_mut() {
        if tile_transform.translation.distance(cursor_position) <= 0.8 {
            tile_transform.translation.z = 0.5;
        } else {
            tile_transform.translation.z = 0.0;
        }
    }
}

fn _get_circle_position(cursor_position: Vec2, window: &Window) -> Vec3 {
    // w_h / 2 -> y 0
    // w_w / 2 -> x 0

    // c_x 0 -> c_x - w_w/2
    // c_y 0 -> c_y - w_h/2

    let scale_factor = 0.035;

    let w_width = window.width();
    let w_height = window.height();

    let circ_x = cursor_position.x - w_width / 2.;
    let circ_y = cursor_position.y - w_height / 2.;

    return Vec3::new(circ_x * scale_factor, -circ_y * scale_factor, 0.1);
}
