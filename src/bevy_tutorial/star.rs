use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

pub const NUMBER_OF_STARS: i32 = 10;

#[derive(Component)]
pub struct Star {}

pub fn spawn_star(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    // is there an easy way to load this to the heap?
    // does bevy asset server optimize for this?
    // is there a difference between loading it here and loading it in the loop?
    let star_sprite: Handle<Image> = asset_server.load("sprites/star.png");

    let window = window_query.get_single().unwrap();

    let w_width = window.width();
    let w_height = window.height();

    // why does this work for _ in [1..NUMBER_OF_STARTS]
    for _ in 1..NUMBER_OF_STARS {
        let star_x = random::<f32>() * w_width;
        let star_y = random::<f32>() * w_height;

        let star_transform = Transform::from_xyz(star_x, star_y, 0.0);

        commands.spawn((
            SpriteBundle {
                transform: star_transform,
                texture: star_sprite.clone(),
                ..default()
            },
            Star {},
        ));
    }
}
