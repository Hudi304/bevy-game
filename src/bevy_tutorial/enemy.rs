use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

pub const ENEMY_NO: usize = 6;

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}

pub fn spawn_enemy(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..ENEMY_NO {
        let en_x = random::<f32>() * window.width();
        let en_y = random::<f32>() * window.height();
        let en_pos = Vec3::new(en_x, en_y, 0.0);

        let en_dir_x = random::<f32>();
        let en_dir_y = random::<f32>();

        let en_dir: Vec2 = (en_dir_x, en_dir_y).into();
        let en_dir = en_dir.normalize();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_translation(en_pos),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy { direction: en_dir },
        ));
    }
}
