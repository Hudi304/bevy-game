use bevy::{prelude::*, window::PrimaryWindow};

use super::enamy::Enamy;

pub const ENAMY_SPPED: f32 = 200.0;
pub const ENAMY_SIZE: f32 = 64.0;

pub fn enamy_movement(mut enamy_query: Query<(&mut Transform, &Enamy)>, time: Res<Time>) {
    let en_iter = enamy_query.iter_mut();

    let delta_time = time.delta_seconds();

    for (mut transform, enamy) in en_iter {
        let direction = Vec3::new(enamy.direction.x, enamy.direction.y, 0.0);
        transform.translation += direction * ENAMY_SPPED * delta_time;
    }

    ()
}

// when it hits a wall reverse the direction on that component
pub fn enamy_wall_collison(
    mut enamy_query: Query<(&Transform, &mut Enamy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let en_iter = enamy_query.iter_mut();

    for (en_tranform, mut enamy) in en_iter {
        let window = window_query.get_single().unwrap();
        let half_en_size = ENAMY_SIZE / 2.0;

        let x_min = 0.0 + half_en_size;
        let x_max = window.width() - half_en_size;
        let y_min = 0.0 + half_en_size;
        let y_max = window.height() - half_en_size;

        // this is a copy
        // let mut en_dir = enamy.direction;
        // this is a mvoe
        let en_trnsl = en_tranform.translation;

        // Bound enamy x position
        if en_trnsl.x < x_min || en_trnsl.x > x_max {
            enamy.direction.x *= -1.0;
        }

        // Bound enamy y position
        if en_trnsl.y < y_min || en_trnsl.y > y_max {
            enamy.direction.y *= -1.0;
        }
    }
}

// we don't need the enamy conponent, so we can use a With FIlter
pub fn confine_enamy_movement(
    mut enamy_query: Query<&mut Transform, With<Enamy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let en_iter = enamy_query.iter_mut();
    let half_en_size = ENAMY_SIZE / 2.0;

    let x_min = 0.0 + half_en_size;
    let x_max = window.width() - half_en_size;
    let y_min = 0.0 + half_en_size;
    let y_max = window.height() - half_en_size;

    for mut en_tranform in en_iter {
        let mut en_transaltion = en_tranform.translation;

        // Bound player x position
        if en_transaltion.x < x_min {
            en_transaltion.x = x_min;
        }

        if en_transaltion.x > x_max {
            en_transaltion.x = x_max;
        }

        // Bound player y position
        if en_transaltion.y < y_min {
            en_transaltion.y = y_min;
        }

        if en_transaltion.y > y_max {
            en_transaltion.y = y_max;
        }

        en_tranform.translation = en_transaltion;
    }
}
