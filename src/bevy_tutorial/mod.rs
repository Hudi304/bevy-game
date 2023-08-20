use bevy::{prelude::*, window::WindowResolution};

use self::{
    camera::spawn_camera,
    enemy::EnemyPlugin,
    exit_game::exit_game,
    exit_game::{handle_game_over_event, GameOverEvent},
    player::PlayerPlugin,
    score::{update_score, Score},
    star::StarPlugin,
};

pub mod camera;
pub mod enemy;
pub mod exit_game;
pub mod player;
pub mod score;
pub mod star;

pub struct BallGame;

impl Plugin for BallGame {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                window_level: bevy::window::WindowLevel::AlwaysOnTop,
                resolution: WindowResolution::new(400.0, 400.0),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<Score>()
        .add_event::<GameOverEvent>()
        .add_systems(Startup, spawn_camera)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(StarPlugin)
        .add_systems(
            Update,
            (update_score, handle_game_over_event, exit_game).chain(),
        );
    }
}
