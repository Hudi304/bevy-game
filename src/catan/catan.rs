use bevy::{prelude::*, window::WindowResolution};

use super::{
    orbit_camera::{pan_orbit_camera, spawn_pan_camera},
    polygon::circle::spawn_circ_bevy,
    world::map::spawn_map,
};

pub struct Catan;

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

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
        // .add_systems(Startup, spawn_3d_camera)
        .add_systems(Startup, spawn_pan_camera)
        .add_systems(Update, pan_orbit_camera)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
        .add_systems(Startup, spawn_map)
        .add_systems(Startup, spawn_circ_bevy)
        // .add_systems(Update, non_rotating_camera_tile_collision);
;
        return ();
    }
}
