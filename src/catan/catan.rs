use bevy::{
    prelude::*,
    window::{WindowResolution, WindowTheme},
};

use super::{
    camera::spawn_3d_camera,
    map_tile::spawn_map,
    mouse_controls::{mouse_motion, non_rotating_camera_tile_collision}, circle::spawn_3d_quad,
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
                window_theme: Some(WindowTheme::Dark),

                window_level: bevy::window::WindowLevel::AlwaysOnTop,
                resolution: WindowResolution::new(600.0, 400.0),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
        .add_systems(Startup, spawn_3d_camera)
        // .add_systems(Startup, spawn_map)
        // .add_systems(Startup, )
        // .add_systems(Startup, spawn_map)
        // .add_systems(Update, mouse_motion)
        // .add_systems(Startup, spawn_circle)
        .add_systems(Startup, spawn_3d_quad)
        .add_systems(Update, non_rotating_camera_tile_collision);
    }
}
