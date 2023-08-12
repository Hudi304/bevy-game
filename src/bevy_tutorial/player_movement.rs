use bevy::{prelude::*, window::PrimaryWindow};

use super::player::Player;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        let left = keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A);
        let right = keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D);
        let up = keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W);
        let down = keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S);

        if left {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if right {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if up {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if down {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.0;

        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;

        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;

        let mut player_transaltion = player_transform.translation;

        // Bound player x position
        if player_transaltion.x < x_min {
            player_transaltion.x = x_min;
        }

        if player_transaltion.x > x_max {
            player_transaltion.x = x_max;
        }

        // Bound player y position
        if player_transaltion.y < y_min {
            player_transaltion.y = y_min;
        }

        if player_transaltion.y > y_max {
            player_transaltion.y = y_max;
        }

        // why is this needed didn't we get a mutable reference to the 
        // player transform?  is it a copy?
        player_transform.translation = player_transaltion;
    }
}
