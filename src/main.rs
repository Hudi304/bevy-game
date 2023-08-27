use bevy::prelude::*;
use bevy_tutorial::{exit_game::GameOverEvent, BallGame};
use catan::catan::Catan;

mod bevy_tutorial;
mod hex;
mod setup;

mod catan;

fn main() {
    // BALL GAME
    // App::new().add_plugins(BallGame).run();
    App::new().add_plugins(Catan).run();
}
