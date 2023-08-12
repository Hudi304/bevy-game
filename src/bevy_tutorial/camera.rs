use bevy::{prelude::*, window::PrimaryWindow};

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    let half_w_width = window.width() / 2.0;
    let half_w_height = window.height() / 2.0;

    let camera_transform = Transform::from_xyz(half_w_width, half_w_height, 1.0);

    commands.spawn(Camera2dBundle {
        transform: camera_transform,
        ..default()
    });
}
