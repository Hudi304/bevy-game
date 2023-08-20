use bevy::prelude::*;

#[derive(Resource)]
pub struct Score {
    pub value: u32,
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
