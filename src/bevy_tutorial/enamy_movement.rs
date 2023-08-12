use bevy::prelude::*;

use super::enamy::Enamy;

pub const ENAMY_SPPED: f32 = 500.0;

pub fn enamy_movement(mut enamy_query: Query<(&mut Transform, &Enamy)>, time: Res<Time>) {
    let en_iter = enamy_query.iter_mut();

    let delta_time = time.delta_seconds();

    for (mut transform, enamy) in en_iter {
        let direction = Vec3::new(enamy.direction.x, enamy.direction.y, 0.0);
        transform.translation += direction * ENAMY_SPPED * delta_time;
    }

    return ();
}
