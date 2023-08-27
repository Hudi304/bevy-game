use bevy::{prelude::*, window::WindowResolution};
use bevy_mod_picking::DefaultPickingPlugins;

use super::{
    orbit_camera::{pan_orbit_camera, spawn_orbit_camera},
    polygon::circle::spawn_circ_bevy,
    world::spawn_map::spawn_map,
};

pub struct CatanPlugin;

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

impl Plugin for CatanPlugin {
    fn build(&self, app: &mut App) {
        let default_plugins = DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                window_level: bevy::window::WindowLevel::AlwaysOnTop,
                resolution: WindowResolution::new(600.0, 400.0),
                ..default()
            }),
            ..default()
        });

        app.add_plugins(default_plugins)
            .add_plugins(DefaultPickingPlugins)
            .insert_resource(ClearColor(BACKGROUND_COLOR))
            .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
            .add_systems(Startup, spawn_orbit_camera)
            .add_systems(Update, pan_orbit_camera)
            .add_systems(Startup, spawn_map)
            .add_systems(Startup, spawn_circ_bevy);
        return ();
    }
}
