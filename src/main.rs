use std::f32::consts::PI;

use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use hex::{
    hex::get_hex_vertices,
    poligon::{build_polygon_mesh, get_polygon_vert},
};

mod common;

mod hex;

use crate::common::{wall_bundle::WallBundle, wall_location::WallLocation};

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

#[derive(Event, Default)]
struct CollisionEvent;

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
        .add_systems(Update, render_map)
        .run();
}

fn system(mut gizmos: Gizmos, _time: Res<Time>) {
    // let sin = time.elapsed_seconds().sin() * 50.;

    let r = 6.;
    let center_position = Vec3::new(0., 0., 0.);

    gizmos.circle(center_position, Vec3::Z, r, Color::RED);

    gizmos.line_2d(Vec2::Y, Vec2::splat(-80.), Color::RED);
    gizmos.ray_2d(Vec2::Y, Vec2::splat(80.), Color::GREEN);
}

fn render_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // color
    let red = Color::rgb_u8(255, 0, 0);
    let material = materials.add(red.into());

    // tile mesh
    let hex_tile_vtx_pos = get_polygon_vert(6, 1.0, 0.);
    let hex_tile_mesh = build_polygon_mesh(hex_tile_vtx_pos);
    let mesh = meshes.add(hex_tile_mesh);

    // tile positions

    // center and first circle
    let mut tile_pos_vec1 = get_hex_vertices(1.8, PI / 6.);

    // outer circle
    let outer_circle = get_hex_vertices(3.1, 0.0);
    let outer_circle: Vec<[f32; 3]> = outer_circle.iter().skip(1).cloned().collect();

    tile_pos_vec1.extend(outer_circle);

    for pos in tile_pos_vec1.iter() {
        let transform = Transform::from_translation(Vec3::from_slice(pos));

        commands.spawn(PbrBundle {
            material: material.clone(),
            mesh: mesh.clone(),
            transform,
            ..Default::default()
        });
    }
}

// Add the game's entities to our world
fn setup(
    mut commands: Commands,
    mut _meshes: ResMut<Assets<Mesh>>,
    mut _materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0., 0., 15.))
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(0., 0., 10.)),
        ..Default::default()
    });

    // Walls
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(WallBundle::new(WallLocation::Top));
}
