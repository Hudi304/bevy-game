use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResolution},
};

use crate::{
    hex::polygon::{build_polygon_mesh, get_polygon_vert_with_center},
    setup::setup_camera_and_walls,
};

use super::{
    camera::spawn_3d_camera,
    map_tile::{spawn_center_tile, spawn_fist_tile_row, spawn_second_tile_row},
};

pub struct Catan;

// use crate::setup::setup_camera_and_walls;

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
// fn _catan_clone() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .insert_resource(ClearColor(_BACKGROUND_COLOR))
//         .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
//         .add_systems(Startup, setup_camera_and_walls)
//         .add_systems(Update, bevy::window::close_on_esc)
//         .add_systems(Update, gizmos_system)
//         .add_systems(Update, gizmos_system)
//         .add_systems(Update, render_map)
//         .run();
// }

impl Plugin for Catan {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                window_level: bevy::window::WindowLevel::AlwaysOnTop,
                resolution: WindowResolution::new(600.0, 400.0),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
        .add_systems(Startup, spawn_3d_camera)
        // .add_systems(Startup, render_map)
        .add_systems(Startup, spawn_center_tile)
        .add_systems(Startup, spawn_fist_tile_row)
        .add_systems(Startup, spawn_second_tile_row);

        // .add_systems(Update, bevy::window::close_on_esc);
    }
}
