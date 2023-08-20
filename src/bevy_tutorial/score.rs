use bevy::prelude::*;

pub const STAR_SPAWN_DT_S: f32 = 1.0; // 1 sec

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}
#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> StarSpawnTimer {
        StarSpawnTimer {
            timer: Timer::from_seconds(STAR_SPAWN_DT_S, TimerMode::Repeating),
        }
    }
}

impl Default for Score {
    fn default() -> Self {
        Score { value: 0 }
    }
}

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        let score_val = score.value;
        println!("Score : {score_val}");
    }
}
