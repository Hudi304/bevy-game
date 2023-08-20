use bevy::prelude::*;

use self::star::{spawn_star, spawn_stars_over_time, tick_star_spawn_timer, StarSpawnTimer};

pub mod star;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .add_systems(Startup, spawn_star)
            .add_systems(Update, spawn_stars_over_time)
            .add_systems(Update, tick_star_spawn_timer);
    }
}
