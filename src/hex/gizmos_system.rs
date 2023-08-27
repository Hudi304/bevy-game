use bevy::prelude::*;

pub fn _gizmos_system(mut gizmos: Gizmos, _time: Res<Time>) {
    // let sin = time.elapsed_seconds().sin() * 50.;

    let r = 6.;
    let center_position = Vec3::new(0., 0., 0.);

    gizmos.circle(center_position, Vec3::Z, r, Color::RED);

    gizmos.line_2d(Vec2::Y, Vec2::splat(-80.), Color::RED);
    gizmos.ray_2d(Vec2::Y, Vec2::splat(80.), Color::GREEN);
}
