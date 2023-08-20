use bevy::prelude::*;

pub struct Catan;

// use crate::setup::setup_camera_and_walls;

const _BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
// fn _catan_clone() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .insert_resource(ClearColor(_BACKGROUND_COLOR))
//         .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
//         .add_systems(Startup, setup_camera_and_walls)
//         .add_systems(Update, bevy::window::close_on_esc)
//         .add_systems(Update, gizmos_system)
//         .add_systems(Update, gizmos_system)
//         .add_systems(Update, render_map)
//         .run();
// }

impl Plugin for Catan {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins);
    }
}
