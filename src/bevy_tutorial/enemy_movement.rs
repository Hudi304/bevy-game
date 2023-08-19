use bevy::{prelude::*, window::PrimaryWindow};

use super::enemy::Enemy;

pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0;

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    let en_iter = enemy_query.iter_mut();

    let delta_time = time.delta_seconds();

    for (mut transform, enemy) in en_iter {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * delta_time;
    }

    ()
}

// when it hits a wall reverse the direction on that component
pub fn enemy_wall_collision(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let en_iter = enemy_query.iter_mut();

    for (en_transform, mut enemy) in en_iter {
        let window = window_query.get_single().unwrap();
        let half_en_size = ENEMY_SIZE / 2.0;

        let x_min = 0.0 + half_en_size;
        let x_max = window.width() - half_en_size;
        let y_min = 0.0 + half_en_size;
        let y_max = window.height() - half_en_size;

        // this is a copy
        // let mut en_dir = enemy.direction;
        // this is a move
        let en_translation = en_transform.translation;

        // Bound enemy x position
        if en_translation.x <= x_min {
            enemy.direction.x *= -1.0;
        }

        if en_translation.x >= x_max {
            enemy.direction.x *= -1.0;
        }

        // Bound enemy y position
        if en_translation.y <=  y_min {
            enemy.direction.y *= -1.0;
        }

        if en_translation.y >= y_max {
            enemy.direction.y *= -1.0;
        }
    }
}

// we don't need the enemy component, so we can use a With FIlter
pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let en_iter = enemy_query.iter_mut();
    let half_en_size = ENEMY_SIZE / 2.0;

    let x_min = 0.0 + half_en_size;
    let x_max = window.width() - half_en_size;
    let y_min = 0.0 + half_en_size;
    let y_max = window.height() - half_en_size;

    for mut en_transform in en_iter {
        let mut en_translation = en_transform.translation;

        // Bound player x position
        if en_translation.x < x_min {
            en_translation.x = x_min;
        }

        if en_translation.x > x_max {
            en_translation.x = x_max;
        }

        // Bound player y position
        if en_translation.y < y_min {
            en_translation.y = y_min;
        }

        if en_translation.y > y_max {
            en_translation.y = y_max;
        }

        en_transform.translation = en_translation;
    }
}
