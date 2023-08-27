use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::WindowResolution};
use bevy_mod_picking::{
    debug::DebugPickingPlugin,
    low_latency_window_plugin,
    prelude::{RaycastPickCamera, RaycastPickTarget},
    DefaultPickingPlugins, PickableBundle,
};

use super::{
    orbit_camera::{spawn_orbit_camera, update_camera_rotation},
    polygon::circle::spawn_circ_bevy,
    world::spawn_map::spawn_map,
};

pub struct CatanPlugin;

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

impl Plugin for CatanPlugin {
    fn build(&self, app: &mut App) {
        let default_plugins = DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                window_level: bevy::window::WindowLevel::AlwaysOnTop,
                resolution: WindowResolution::new(600.0, 400.0),
                ..default()
            }),
            ..default()
        });

        app
            // PLUGINS
            .add_plugins(default_plugins)
            .add_plugins(DefaultPickingPlugins)
            // RESOURCES
            .insert_resource(ClearColor(BACKGROUND_COLOR))
            .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
            // STARTUP
            .add_systems(Startup, spawn_orbit_camera)
            .add_systems(Startup, spawn_map)
            .add_systems(Startup, spawn_circ_bevy)
            // UPDATE
            .add_systems(Update, update_camera_rotation);

        // app.add_plugins(DefaultPlugins.set(low_latency_window_plugin()))
        //     .add_plugins(DefaultPickingPlugins)
        //     .add_systems(Startup, setup);

        return ();
    }
}

// Spawn a simple scene, like bevy's 3d_scene example.
// fn setup(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     commands.spawn((
//         PbrBundle {
//             mesh: meshes.add(Mesh::from(shape::Plane::from_size(5.0))),
//             material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
//             ..default()
//         },
//         PickableBundle::default(), // Adds selection, highlighting, and the `Pickable` override.
//         RaycastPickTarget::default(),
//     ));
//     commands.spawn((
//         PbrBundle {
//             mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
//             material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
//             transform: Transform::from_xyz(0.0, 0.5, 0.0),
//             ..default()
//         },
//         RaycastPickTarget::default(),
//     ));
//     commands.spawn(PointLightBundle {
//         point_light: PointLight {
//             intensity: 1500.0,
//             shadows_enabled: true,
//             ..default()
//         },
//         transform: Transform::from_xyz(4.0, 8.0, -4.0),
//         ..default()
//     });
//     commands.spawn((
//         Camera3dBundle {
//             transform: Transform::from_xyz(3.0, 3.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
//             ..default()
//         },
//         // RaycastPickCamera::default(),
//     ));
// }
