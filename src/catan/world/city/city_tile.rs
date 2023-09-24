use std::ops::Range;

use bevy::prelude::{ shape::Circle, * };

use crate::catan::{ cubic_coords::cube_coordinates::CubicCoord, utils::vec::i32_tup_to_f32_tup };

#[derive(Component)]
pub struct CityTile {
    // adjacent resources
    // maybe update this when the city is placed
    // also maybe 3 possibilities of roads 
}

// TODO build a data structure with all the possible coordinates
// render all the possibilities with some debug function
// place a city
// compute the remaining valid positions
// can the positions list be a resource?

pub fn spawn_city_placer_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let cub_coords_arr: Vec<CubicCoord> = build_city_hex_grid(5);

    let color_arr = vec![
        Color::WHITE,
        Color::PINK,
        Color::ANTIQUE_WHITE,
        Color::YELLOW,
        Color::PURPLE,
        Color::WHITE
    ];

    let circ = Circle::new(0.1);

    for cub_coord in cub_coords_arr {
        let mut cart_coord = cub_coord.to_cartesian_vec3(1.0);

        cart_coord.z = 0.1;

        let cub_f32_tup = i32_tup_to_f32_tup((cub_coord.q, cub_coord.r, cub_coord.s));
        let city_cart = city_cart(cub_f32_tup, 1.0);

        commands.spawn((
            PbrBundle {
                mesh: meshes.add(circ.into()),
                material: materials.add(color_arr[cub_coord.ring as usize].into()),
                transform: Transform::from_translation(Vec3 {
                    x: city_cart.0,
                    y: city_cart.1,
                    z: 0.1,
                }),
                ..default()
            },
            CityTile {},
        ));
    }
}

fn city_x((q, r, _): (f32, f32, f32), a: f32) -> f32 {
    let sqrt_3 = (3_f32).sqrt();
    return ((a * sqrt_3) / 2.0) * (q + r);
}

fn city_y((q, r, _): (f32, f32, f32), a: f32) -> f32 {
    return ((r - q) / 2.0) * a;
}

fn city_cart((q, r, s): (f32, f32, f32), a: f32) -> (f32, f32) {
    let x = city_x((q, r, s), a);
    let y = city_y((q, r, s), a);

    return (x, y);
}

// TODO I hate this, find a better way of doing it
pub fn build_city_hex_grid(radius: i32) -> Vec<CubicCoord> {
    let mut hex_arr = vec![];
    let slice: Range<i32> = -radius..radius + 1;
    // let slice: Vec<i32> = slice.into_iter().map(|i| i * 2 + 1).collect();

    // find a better way to do this
    for q in slice.clone() {
        for r in slice.clone() {
            let s: i32 = 0 - q - r;

            let sum = q.abs() + r.abs() + s.abs();

            if s.abs() > radius {
                continue;
            }

            if q == 0 && r == 0 && s == 0 {
                continue;
            }

            if sum / 2 == 2 {
                if q == r || s == r || q == s {
                    continue;
                }
            }

            if sum / 2 == 3 {
                if q.abs() == r.abs() || s.abs() == r.abs() || q.abs() == s.abs() {
                    continue;
                }
            }

            if sum / 2 == 4 {
                let q = q.abs();
                let r = r.abs();
                let s = s.abs();

                if (q, r, s) == (4, 2, 2) || (q, r, s) == (2, 4, 2) || (q, r, s) == (2, 2, 4) {
                    continue;
                }
            }

            if sum / 2 == 5 {
                let q = q.abs();
                let r = r.abs();
                let s = s.abs();

                if
                    (q, r, s) != (5, 3, 2) &&
                    (q, r, s) != (5, 2, 3) &&
                    (q, r, s) != (2, 3, 5) &&
                    (q, r, s) != (2, 5, 3) &&
                    (q, r, s) != (3, 2, 5) &&
                    (q, r, s) != (3, 5, 2)
                {
                    continue;
                }
            }

            hex_arr.push(CubicCoord::from_tuple((q, r, s)));
        }
    }

    return hex_arr;
}

#[cfg(test)]
mod tests {
    use std::f32::EPSILON;

    use crate::catan::utils::vec::i32_tup_to_f32_tup;

    use super::*;

    #[test]
    fn test_6_points() {
        let r = 1f32;

        let a = ((0, 1, -1), ((r * (3_f32).sqrt()) / 2.0, r / 2.0));
        let ap = ((0, -1, 1), ((-r * (3_f32).sqrt()) / 2.0, -r / 2.0));

        let b = ((1, 0, -1), ((r * (3_f32).sqrt()) / 2.0, -r / 2.0));
        let bp = ((-1, 0, 1), ((-r * (3_f32).sqrt()) / 2.0, r / 2.0));

        let c = ((-1, 1, 0), (0f32, r));
        let cp = ((1, -1, 0), (0f32, -r));

        let tests = vec![a, ap, b, bp, c, cp];

        for (cubic, cart) in tests {
            let f32_tup = i32_tup_to_f32_tup(cubic);
            let (cart_x, cart_y) = city_cart(f32_tup, r);

            assert_eq!(cart_x - cart.0 < EPSILON, true);
            assert_eq!(cart_y - cart.1 < EPSILON, true);
        }
    }
}
