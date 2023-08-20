use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

pub const ENEMY_NO: usize = 2;

pub const ENEMY_SPAWN_DT_S: f32 = 5.0;

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}

#[derive(Resource)]
pub struct SpawnEnemyTimer {
    timer: Timer,
}

impl Default for SpawnEnemyTimer {
    fn default() -> SpawnEnemyTimer {
        SpawnEnemyTimer {
            timer: Timer::from_seconds(ENEMY_SPAWN_DT_S, TimerMode::Repeating),
        }
    }
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

        let en_transform = Transform::from_translation(en_pos);

        commands.spawn((
            SpriteBundle {
                transform: en_transform,
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy { direction: en_dir },
        ));
    }
}

pub fn tick_spawn_enemies_timer(mut spawn_en_timer: ResMut<SpawnEnemyTimer>, time: Res<Time>) {
    let delta_t = time.delta();
    spawn_en_timer.timer.tick(delta_t);
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: Res<SpawnEnemyTimer>,
) {
    if enemy_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();

        let en_x = random::<f32>() * window.width();
        let en_y = random::<f32>() * window.height();
        let en_pos = Vec3::new(en_x, en_y, 0.0);

        let en_dir_x = random::<f32>();
        let en_dir_y = random::<f32>();

        let en_dir: Vec2 = (en_dir_x, en_dir_y).into();
        let en_dir = en_dir.normalize();

        let en_transform = Transform::from_translation(en_pos);

        commands.spawn((
            SpriteBundle {
                transform: en_transform,
                texture: asset_server.load("sprites/ball_red_large.png"),

                ..default()
            },
            Enemy { direction: en_dir },
        ));
    }
}
