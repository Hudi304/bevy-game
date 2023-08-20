use bevy::prelude::*;

use self::{
    player::spawn_player,
    player_hit_enemy::player_hit_enemy,
    player_hit_star::player_hit_star,
    player_movement::{confine_player_movement, player_input},
};

pub mod player;
pub mod player_hit_enemy;
pub mod player_hit_star;
pub mod player_movement;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_input.before(confine_player_movement))
            .add_systems(Update, confine_player_movement)
            .add_systems(Update, player_hit_enemy)
            .add_systems(Update, player_hit_star);
    }
}
