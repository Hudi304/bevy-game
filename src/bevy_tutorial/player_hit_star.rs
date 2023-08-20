use bevy::prelude::*;

use super::{
    player::Player,
    player_movement::PLAYER_SIZE,
    score::Score,
    star::{Star, STAR_SIZE},
};

pub fn player_hit_star(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    mut score_res: ResMut<Score>,
) {
    let pl_res = player_query.get_single();

    if let Ok(pl_transform) = pl_res {
        let pl_translation = pl_transform.translation;

        for (star_ent, star_tr) in star_query.iter() {
            let star_translation = star_tr.translation;
            let dist = pl_translation.distance(star_translation);

            let min_dist = PLAYER_SIZE / 2.0 + STAR_SIZE / 2.0;

            if dist <= min_dist {
                commands.entity(star_ent).despawn();

                let score = score_res.as_mut();
                score.value += 1;
                let score_val = score.value;
                println!("Score : {score_val}");
            }
        }
    }
}
