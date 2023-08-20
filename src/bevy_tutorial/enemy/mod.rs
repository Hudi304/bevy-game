use bevy::prelude::*;

use self::{
    enemy::{spawn_enemy, tick_spawn_enemies_timer, SpawnEnemyTimer},
    enemy_movement::{confine_enemy_movement, enemy_movement, enemy_wall_collision},
};

pub mod enemy;
pub mod enemy_movement;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpawnEnemyTimer>()
            .add_systems(Startup, spawn_enemy)
            .add_systems(Update, enemy_movement)
            .add_systems(Update, confine_enemy_movement)
            .add_systems(Update, enemy_wall_collision)
            .add_systems(Update, tick_spawn_enemies_timer);

        return ();
    }
}
