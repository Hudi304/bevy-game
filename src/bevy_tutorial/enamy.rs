use bevy::{prelude::*, transform::commands, window::PrimaryWindow};
use rand::random;

pub const ENAMY_NO: usize = 6;

#[derive(Component)]
pub struct Enamy {}

pub fn spawn_enamy(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..ENAMY_NO {
        let en_x = random::<f32>() * window.width();
        let en_y = random::<f32>() * window.width();

        let en_pos = Vec3::new(en_x, en_y, 0.0);

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_translation(en_pos),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enamy {},
        ));
    }
}
