use bevy::prelude::{shape::Circle, *};

use crate::catan::cubic_coords::cube_coordinates::CubCoord;

use super::spawn_map::build_cub_coord_hex_gird;

#[derive(Component)]
pub struct CityTile {}

// TODO build a data structure with all the possible coordinates
// render all the possibilities with some debug function
// place a city
// compute the remaining valid positions
// can the positions list be a resource?

pub fn spawn_city_placer_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cub_coords_arr: Vec<CubCoord> = build_cub_coord_hex_gird(2);

    let circ = Circle::new(0.1);

    for cub_coord in cub_coords_arr {
        let mut cart_coord = cub_coord.to_cartesian_vec3(1.);

        cart_coord.z = 0.1;

        commands.spawn((
            PbrBundle {
                mesh: meshes.add(circ.into()),
                material: materials.add(Color::PURPLE.into()),
                transform: Transform::from_translation(cart_coord),
                ..default()
            },
            CityTile {},
        ));
    }
}

fn city_x((q, r, s): (f32, f32, f32), a: f32) -> f32 {
    let sqrt_3 = 3_f32.sqrt();
    return a * sqrt_3 / 2. * (q + r);
}

fn city_y((q, r, s): (f32, f32, f32), a: f32) -> f32 {
    let sqrt_3 = 3_f32.sqrt();
    return (r - q) / 2.;
}

fn city_cart((q, r, s): (f32, f32, f32), a: f32) -> (f32, f32) {
    let x = city_x((q, r, s), a);
    let y = city_y((q, r, s), a);

    return (x, y);
}

#[cfg(test)]
mod tests {
    use std::f32::EPSILON;

    use crate::catan::utils::vec::i32_tup_to_f32_tup;

    use super::*;

    #[test]
    fn test_6_points() {
        let r = 1f32;

        let a = ((0, 1, -1), (r * 3_f32.sqrt() / 2., r / 2.));
        let ap = ((0, -1, 1), (-r * 3_f32.sqrt() / 2., -r / 2.));

        let b = ((1, 0, -1), (r * 3_f32.sqrt() / 2., -r / 2.));
        let bp = ((-1, 0, 1), (-r * 3_f32.sqrt() / 2., r / 2.));

        let c = ((-1, 1, 0), (0f32, r));
        let cp = ((1, -1, 0), (0f32, -r));

        let tests = vec![a, ap, b, bp, c, cp];

        for (cubic, cart) in tests {
            let f32_tup = i32_tup_to_f32_tup(cubic);
            let (cart_x, cart_y) = city_cart(f32_tup, r);

            assert_eq!((cart_x - cart.0) < EPSILON, true);
            assert_eq!((cart_y - cart.1) < EPSILON, true);
        }
    }
}
