use bevy::prelude::Resource;

// This resource tracks the game's score
#[derive(Resource)]
pub struct Scoreboard {
    score: usize,
}
