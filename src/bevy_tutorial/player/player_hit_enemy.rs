use bevy::prelude::*;

use crate::{
    bevy_tutorial::{
        enemy::{enemy::Enemy, enemy_movement::ENEMY_SIZE},
        score::Score,
    },
    GameOverEvent,
};

use super::{player::Player, player_movement::PLAYER_SIZE};

pub fn player_hit_enemy(
    mut commands: Commands,
    player_query: Query<(Entity, &Transform), With<Player>>, // this does not have to be mutable to despawn the entity
    enemy_query: Query<&Transform, With<Enemy>>,
    mut game_over_event_writer: EventWriter<GameOverEvent>,
    score: Res<Score>,
) {
    let pl_query_result = player_query.get_single();

    if let Err(_) = pl_query_result {
        return;
    }

    let (pl_ent, pl_tr) = player_query.get_single().unwrap();
    let en_tr_iter = enemy_query.iter();

    for en_transform in en_tr_iter {
        let en_translation = en_transform.translation;
        let pl_translation = pl_tr.translation;

        let pl_en_distance = pl_translation.distance(en_translation);
        // let en_x = en_translation.x;
        // let en_y = en_translation.y;

        // let pl_x = pl_translation.x;
        // let pl_y = pl_translation.y;

        // let pl_en_distance_pow_2 = (pl_x - en_x).powi(2) + (pl_y - en_y).powi(2);
        // let pl_en_distance = pl_en_distance_pow_2.abs().sqrt();

        let min_dist = PLAYER_SIZE / 2.0 + ENEMY_SIZE / 2.0;

        if pl_en_distance <= min_dist {
            println!("HIT");
            commands.entity(pl_ent).despawn();

            let final_score = score.as_ref().value;
            game_over_event_writer.send(GameOverEvent { final_score });
        }
    }

    return ();
}
