use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Component)]
pub struct Player {}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let transform = Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0);

    let texture = asset_server.load("sprites/ball.png");

    commands.spawn((
        SpriteBundle {
            transform,
            texture,
            ..default()
        },
        Player {},
    ));
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 1.0)
            .with_scale(Vec3 {
                x: 8.0,
                y: 8.0,
                z: 1.0,
            }),
        ..default()
    });
}

pub const PLAYER_SPEED: f32 = 500.0;

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
