use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

use super::score::StarSpawnTimer;

pub const NUMBER_OF_STARS: i32 = 10;
pub const STAR_SIZE: f32 = 30.0;

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

pub fn tick_star_spawn_timer(mut star_timer_res: ResMut<StarSpawnTimer>, time: Res<Time>) {
    let delta_t = time.delta(); // Duration since last update (TODO is update a frame?)
    star_timer_res.timer.tick(delta_t); // the tick method advances the timer
                                        // so basically we advance the star_spawn timer with the duration between updates
                                        // on every update
                                        // because this timer is a repeating timer, it will get to the complete state
                                        // and another system will spawn a star
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();

        let w_width = window.width();
        let w_height = window.height();

        let star_x = w_width * random::<f32>();
        let star_y = w_height * random::<f32>();

        let star_transform = Transform::from_xyz(star_x, star_y, 0.0);

        commands.spawn((
            SpriteBundle {
                transform: star_transform,
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}
