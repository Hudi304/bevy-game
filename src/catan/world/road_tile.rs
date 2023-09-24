use std::ops::Range;

use bevy::prelude::{
    shape::{Circle, Quad},
    *,
};

use crate::catan::{cubic_coords::cube_coordinates::CubicCoord, utils::vec::i32_tup_to_f32_tup};

#[derive(Component)]
pub struct RoadTile {}

pub fn spawn_road_placer_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cub_coords_arr: Vec<CubicCoord> = build_road_hex_grid(10);

    let color_arr = vec![
        Color::WHITE, // not visible
        Color::WHITE, // not visible
        Color::ANTIQUE_WHITE,
        Color::PURPLE,
        Color::WHITE, // not visible
        Color::ANTIQUE_WHITE,
        Color::PURPLE,
        Color::ANTIQUE_WHITE,
        Color::PURPLE,
        Color::ANTIQUE_WHITE,
        Color::PURPLE,
    ];

    let circ = Quad::new(Vec2 { x: 0.1, y: 0.2 });

    for cub_coord in cub_coords_arr {
        let mut cart_coord = cub_coord.to_cartesian_vec3(1.);

        cart_coord.z = 0.1;

        let cub_f32_tup = i32_tup_to_f32_tup((cub_coord.q, cub_coord.r, cub_coord.s));
        let city_cart = road_cart(cub_f32_tup, 1f32.sqrt() / 2.);

        commands.spawn((
            PbrBundle {
                mesh: meshes.add(circ.into()),
                material: materials.add(color_arr[(cub_coord.ring) as usize].into()),
                transform: Transform::from_translation(Vec3 {
                    x: city_cart.0,
                    y: city_cart.1,
                    z: 0.1,
                }),
                ..default()
            },
            RoadTile {},
        ));
    }
}

fn road_x((q, r, _): (f32, f32, f32), a: f32) -> f32 {
    let sqrt_3 = 3_f32.sqrt();
    return a * sqrt_3 / 2. * (q + r);
}

fn road_y((q, r, _): (f32, f32, f32), a: f32) -> f32 {
    return (r - q) / 2. * a;
}

fn road_cart((q, r, s): (f32, f32, f32), a: f32) -> (f32, f32) {
    let x = road_x((q, r, s), a);
    let y = road_y((q, r, s), a);

    return (x, y);
}

pub fn build_road_hex_grid(radius: i32) -> Vec<CubicCoord> {
    let mut hex_arr = vec![];
    let slice: Range<i32> = -radius..radius + 1;
    // let slice: Vec<i32> = slice.into_iter().map(|i| i * 2 + 1).collect();

    // find a better way to do this
    for q in slice.clone() {
        for r in slice.clone() {
            let s: i32 = 0 - q - r;

            let sum = q.abs() + r.abs() + s.abs();

            let ring = sum / 2;

            if ring == 0 || ring == 1 || ring == 4 {
                continue;
            }

            if ring == 2 {
                let q = q.abs();
                let r = r.abs();
                let s = s.abs();
                if q != 1 && r != 1 && s != 1 {
                    continue;
                }
            }

            if ring == 3 {
                let q = q.abs();
                let r = r.abs();
                let s = s.abs();
                if q != 0 && r != 0 && s != 0 {
                    continue;
                }
            }

            if ring == 5 {
                let q = q.abs();
                let r = r.abs();
                let s = s.abs();
                if q != 1 && r != 1 && s != 1 {
                    continue;
                }
            }

            if ring == 6 {
                let q = q.abs();
                let r = r.abs();
                let s = s.abs();
                if q != 3 && r != 3 && s != 3 {
                    continue;
                }
            }

            if ring == 7 {
                let q = q.abs();
                let r = r.abs();
                let s = s.abs();
                if q != 2 && r != 2 && s != 2 {
                    continue;
                }
            }

            if ring == 8 {
                let q = q.abs();
                let r = r.abs();
                let s = s.abs();
                if q != 1 && r != 1 && s != 1 {
                    continue;
                }
            }

            if ring == 9 {
                let q = q.abs();
                let r = r.abs();
                let s = s.abs();
                if q != 3 && r != 3 && s != 3 {
                    continue;
                }
            }

            if ring == 10 {
                let q = q.abs();
                let r = r.abs();
                let s = s.abs();
                if q != 5 && r != 5 && s != 5 {
                    continue;
                }
            }

            if s.abs() > radius {
                continue;
            }

            hex_arr.push(CubicCoord::from_tuple((q, r, s)));
        }
    }

    return hex_arr;
}
