use bevy::prelude::*;

mod common;

use crate::common::{
    collider::Collider, paddle::Paddle, wall_bundle::WallBundle, wall_location::WallLocation,
};

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

#[derive(Event, Default)]
struct CollisionEvent;

const PADDLE_SIZE: Vec3 = Vec3::new(120.0, 40.0, 0.0);

const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 60.0;
// How close can the paddle get to the wall

const BOTTOM_WALL: f32 = -300.;
const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .insert_resource(common::scoreboard::Scoreboard { score: 0 })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_event::<CollisionEvent>()
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Update, system)
        .run();
}

fn system(mut gizmos: Gizmos, time: Res<Time>) {
    // let sin = time.elapsed_seconds().sin() * 50.;

    let r = 5.;
    let center_position = Vec3::new(0., 0., 0.);

    gizmos.circle(center_position, Vec3::Z, r, Color::RED);

    gizmos.line_2d(Vec2::Y, Vec2::splat(-80.), Color::RED);
    gizmos.ray_2d(Vec2::Y, Vec2::splat(80.), Color::GREEN);
}

// Add the game's entities to our world
fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Paddle
    let paddle_y = BOTTOM_WALL + GAP_BETWEEN_PADDLE_AND_FLOOR;

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, paddle_y, 0.0),
                scale: PADDLE_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: PADDLE_COLOR,
                ..default()
            },
            ..default()
        },
        Paddle,
        Collider,
    ));

    // Walls
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(WallBundle::new(WallLocation::Top));
}
