use bevy::{prelude::*, window::WindowResolution};
use bevy_tutorial::{
    camera::spawn_camera,
    enamy::spawn_enamy,
    enamy_movement::{confine_enamy_movement, enamy_movement, enamy_wall_collison},
    player::spawn_player,
    player_movement::{confine_player_movement, player_input},
};
use hex::{gizmos_system::gizmos_system, map::render_map};

mod bevy_tutorial;
mod common;
mod hex;
mod setup;

use crate::setup::setup_camera_and_walls;

const _BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

fn _catan_clone() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(_BACKGROUND_COLOR))
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
        .add_systems(Startup, setup_camera_and_walls)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Update, gizmos_system)
        .add_systems(Update, gizmos_system)
        .add_systems(Update, render_map)
        .run();
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                window_level: bevy::window::WindowLevel::AlwaysOnTop,
                resolution: WindowResolution::new(400.0, 400.0),
                ..default()
            }),
            ..default()
        }))
        // .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_enamy)
        .add_systems(Update, player_input)
        .add_systems(Update, enamy_movement)
        .add_systems(Update, confine_player_movement)
        .add_systems(Update, confine_enamy_movement)
        .add_systems(Update, enamy_wall_collison)
        .run();

    return ();
}
