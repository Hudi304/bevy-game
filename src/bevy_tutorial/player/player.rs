use bevy::{prelude::*, window::PrimaryWindow};

use super::player_movement::PLAYER_SIZE;

pub const PLAYER_SPRITE_SIZE: f32 = 512.0;
#[derive(Component)]
pub struct Player {}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let half_w_width = window.width() / 2.0;
    let half_w_height = window.height() / 2.0;

    let pos_tr = Transform::from_xyz(half_w_width, half_w_height, 0.0);

    let scale_factor = PLAYER_SIZE / PLAYER_SPRITE_SIZE;
    let scale_vec = Vec3::new(scale_factor, scale_factor, 1.0);
    let player_sprite_tr = pos_tr.with_scale(scale_vec);

    let texture_handle = asset_server.load("sprites/ball.png");

    commands.spawn((
        SpriteBundle {
            transform: player_sprite_tr,
            texture: texture_handle,
            ..default()
        },
        Player {},
    ));

    // Get the dimensions of the sprite's texture
}

