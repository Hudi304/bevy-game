use bevy::prelude::*;

pub fn spawn_3d_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0., 0., 15.))
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(0., 0., 10.)),
        ..Default::default()
    });
}

// pub fn debug_window(window_query: Query<&Window, With<PrimaryWindow>>) {}
