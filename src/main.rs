use bevy::prelude::*;
use bevy_tutorial::player::{player_movement, spawn_camera, spawn_player};
use hex::{gizmos_system::gizmos_system, map::render_map};

mod bevy_tutorial;
mod common;
mod hex;
mod setup;

use crate::setup::setup_camera_and_walls;

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

fn _catan_clone() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
        .add_systems(Startup, setup_camera_and_walls)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Update, gizmos_system)
        .add_systems(Update, render_map)
        .run();
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, player_movement)
        .run();

    return ();
}
