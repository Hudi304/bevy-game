use std::f32::consts::PI;

use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    sprite::MaterialMesh2dBundle,
};

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

fn get_hex_vertexes(r: f32, offset: f32) -> [[f32; 3]; 6] {
    let vertex_no = 6.;

    let full_circle = 2. * PI;

    let x = |i: f32| (r * ((i * full_circle / vertex_no + offset).cos()));
    let y = |i: f32| (r * ((i * full_circle / vertex_no + offset).sin()));

    let v0 = [x(0.), y(0.), 0.];
    let v1 = [x(1.), y(1.), 0.];
    let v2 = [x(2.), y(2.), 0.];
    let v3 = [x(3.), y(3.), 0.];
    let v4 = [x(4.), y(4.), 0.];
    let v5 = [x(5.), y(5.), 0.];

    // println!("r = {}", r);
    // println!("v0 = [{}, {}] ", v0[0], v0[1]);
    // println!("v1 = [{}, {}] ", v1[0], v1[1]);
    // println!("v2 = [{}, {}] ", v2[0], v2[1]);
    // println!("v3 = [{}, {}] ", v3[0], v3[1]);
    // println!("v4 = [{}, {}] ", v4[0], v4[1]);
    // println!("v5 = [{}, {}] ", v5[0], v5[1]);

    return [v0, v1, v2, v3, v4, v5];
}

// TODO search fpr the wey bevy is storing positions
fn poligon_mesh() -> Mesh {
    let vertex_no = 6.;

    let full_circle = 2. * PI;

    let x = |i: f32| (i * full_circle / vertex_no).cos();
    let y = |i: f32| (i * full_circle / vertex_no).sin();

    let center = ([0., 0., 0.], [0., 0., 1.], [0., 0.]);

    let x = |root: f32| (root * 2. * PI / 6.).cos();
    let y = |root: f32| (root * 2. * PI / 6.).sin();

    let spike0 = ([1., 0., 0.], [0., 0., 1.], [0., 0.]);
    let spike1 = ([x(1.), y(1.), 0.], [0., 0., 1.], [0., 0.]);
    let spike2 = ([x(2.), y(2.), 0.], [0., 0., 1.], [0., 0.]);
    let spike3 = ([x(3.), y(3.), 0.], [0., 0., 1.], [0., 0.]);
    let spike4 = ([x(4.), y(4.), 0.], [0., 0., 1.], [0., 0.]);
    let spike5 = ([x(5.), y(5.), 0.], [0., 0., 1.], [0., 0.]);

    let vertices = [center, spike0, spike1, spike2, spike3, spike4, spike5];

    let mut positions = Vec::with_capacity(6);
    let mut normals = Vec::with_capacity(6);
    let mut uvs = Vec::with_capacity(6);

    println!("");

    for (position, normal, uv) in vertices.iter() {
        positions.push(*position);
        normals.push(*normal);
        uvs.push(*uv);

        print!("[{}, {}, {}]", position[0], position[1], position[2]);
        println!("");
    }

    println!("");

    let indices = Indices::U32(vec![0, 1, 2, 0, 2, 3, 0, 3, 4, 0, 4, 5, 0, 5, 6, 0, 6, 1]);

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(indices));

    return mesh;
}

fn vec_to_vec3([x, y, z]: [f32; 3]) -> Vec3 {
    return Vec3 { x, y, z };
}

fn system(mut gizmos: Gizmos, time: Res<Time>) {
    // let sin = time.elapsed_seconds().sin() * 50.;

    let r = 6.;
    let center_position = Vec3::new(0., 0., 0.);

    gizmos.circle(center_position, Vec3::Z, r, Color::RED);

    gizmos.line_2d(Vec2::Y, Vec2::splat(-80.), Color::RED);
    gizmos.ray_2d(Vec2::Y, Vec2::splat(80.), Color::GREEN);
}

// Add the game's entities to our world
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut gizmos: Gizmos,
) {
    // Camera

    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0., 0., 10.))
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(0., 0., 10.)),
        ..Default::default()
    });

    let [v0, v1, v2, v3, v4, v5] = get_hex_vertexes(1.75, PI / 6.);

    let material = materials.add(Color::rgb_u8(255, 0, 0).into());
    let mesh = meshes.add(poligon_mesh());

    let t0 = Transform::from_translation(Vec3::from_slice(&v0));
    let t1 = Transform::from_translation(Vec3::from_slice(&v1));
    let t2 = Transform::from_translation(Vec3::from_slice(&v2));
    let t3 = Transform::from_translation(Vec3::from_slice(&v3));
    let t4 = Transform::from_translation(Vec3::from_slice(&v4));
    let t5 = Transform::from_translation(Vec3::from_slice(&v5));

    println!("[{}, {}, {}]", v0[0], v0[1], v0[2]);
    println!("[{}, {}, {}]", v1[0], v1[1], v1[2]);
    println!("[{}, {}, {}]", v4[0], v4[1], v4[2]);

    commands.spawn(PbrBundle {
        material: material.clone(),
        mesh: mesh.clone(),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        transform: t0,
        material: material.clone(),
        mesh: mesh.clone(),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        transform: t1,
        material: material.clone(),
        mesh: mesh.clone(),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        transform: t2,
        material: material.clone(),
        mesh: mesh.clone(),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        transform: t3,
        material: material.clone(),
        mesh: mesh.clone(),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        transform: t4,
        material: material.clone(),
        mesh: mesh.clone(),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        transform: t5,
        material: material.clone(),
        mesh: mesh.clone(),
        ..Default::default()
    });

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
