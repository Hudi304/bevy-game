use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::WindowResolution,
};
use bevy_mod_picking::DefaultPickingPlugins;

use super::{
    orbit_camera::{spawn_orbit_camera, update_camera_rotation},
    world::{
        city_tile::spawn_city_placer_mesh, road_tile::spawn_road_placer_mesh,
        spawn_cities::spawn_cities, spawn_tiles::spawn_land_tiles,
    },
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

        app
            // DIAGNOSTICS
            .add_plugins(LogDiagnosticsPlugin::default())
            .add_plugins(FrameTimeDiagnosticsPlugin::default())
            // PLUGINS
            .add_plugins(default_plugins)
            .add_plugins(DefaultPickingPlugins)
            // RESOURCES
            .insert_resource(ClearColor(BACKGROUND_COLOR))
            .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
            // STARTUP
            .add_systems(Startup, spawn_orbit_camera)
            .add_systems(Startup, spawn_land_tiles)
            // .add_systems(Startup, spawn_cities)
            // .add_systems(Startup, spawn_map_2)
            // .add_systems(Startup, spawn_city_placer_mesh)
            // .add_systems(Startup, spawn_road_placer_mesh)
            // UPDATE
            .add_systems(Update, update_camera_rotation);

        return ();
    }
}
