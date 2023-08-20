use bevy::{prelude::*, window::WindowResolution};

use super::{camera::spawn_3d_camera, map_tile::test_tile};

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
        // todo try to refactor this into cube coordinates
        //  let x,y,z  be the cube coordinates  and u,v the cartesian coordinates, then
        //  u = x + y/2 + z/2 (maybe just y/2 + z/2)
        //  v = (y + z)sqrt(3)/2
        .add_systems(Startup, spawn_3d_camera)
        // .add_systems(Startup, render_map)
        .add_systems(Startup, test_tile);
        // .add_systems(Startup, spawn_center_tile)
        // .add_systems(Startup, spawn_fist_tile_row)
        // .add_systems(Startup, spawn_second_tile_row)
        // .add_systems(Startup, spawn_water_tile_row);

        // .add_systems(Update, bevy::window::close_on_esc);
    }
}
