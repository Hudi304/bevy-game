use bevy::{prelude::*, window::WindowResolution};
use bevy_tutorial::{
    camera::spawn_camera,
    enemy::{spawn_enemies_over_time, spawn_enemy, tick_spawn_enemies_timer, SpawnEnemyTimer},
    enemy_movement::{confine_enemy_movement, enemy_movement, enemy_wall_collision},
    player::spawn_player,
    player_hit_enemy::player_hit_enemy,
    player_hit_star::player_hit_star,
    player_movement::{confine_player_movement, player_input},
    score::{update_score, Score},
    star::{spawn_star, spawn_stars_over_time, tick_star_spawn_timer, StarSpawnTimer},
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
        .init_resource::<Score>()
        .init_resource::<StarSpawnTimer>()
        .init_resource::<SpawnEnemyTimer>()
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_enemy)
        .add_systems(Startup, spawn_star)
        .add_systems(Update, player_input)
        .add_systems(Update, enemy_movement)
        .add_systems(Update, confine_player_movement)
        .add_systems(Update, confine_enemy_movement)
        .add_systems(Update, enemy_wall_collision)
        .add_systems(Update, player_hit_enemy)
        .add_systems(Update, player_hit_star)
        .add_systems(Update, update_score)
        .add_systems(Update, tick_star_spawn_timer)
        .add_systems(Update, spawn_stars_over_time)
        .add_systems(Update, tick_spawn_enemies_timer)
        .add_systems(Update, spawn_enemies_over_time)
        .run();

    return ();
}
