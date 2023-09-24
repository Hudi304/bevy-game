use std::{ cmp::Ordering, f32::{ consts::PI, EPSILON }, ops::Range };

use bevy::prelude::{ shape::{ Circle, Quad }, * };

use bevy_mod_picking::{ prelude::{ On, Out, Over, Pointer, RaycastPickTarget }, PickableBundle };

use crate::catan::cubic_coords::cube_coordinates::CubicCoord;

use super::{
    land_tile::{ build_tile_mesh, LandTile, NUMBER_OF_TILES },
    randomize_richness::build_random_tile_richness_array,
    randomize_tiles::build_random_tile_type_array,
    tile_type::TileType,
    water_tile::WaterTile,
    vec3_cmp::sort_positions,
};

pub fn spawn_land_tiles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    // let h = 3_f32.sqrt() / 2. * 1.01;

    println!("spawn_land_tiles");

    let offset_angle = PI / 6.0; // 30deg
    let cub_coords_arr: Vec<CubicCoord> = build_cub_coord_hex_gird(7);
    let eps = 0.01;

    let mut i = 0;
    let tile_type_arr = build_random_tile_type_array(NUMBER_OF_TILES);
    let richness_arr = build_random_tile_richness_array(NUMBER_OF_TILES);

    let mut land_tile_arr = Vec::with_capacity(NUMBER_OF_TILES);

    for cub_coord in cub_coords_arr {
        // TODO refactor this so the HexWorldTile contains the material asset and the mesh asset
        if cub_coord.ring < 3 {
            let tile_type = tile_type_arr[i];
            let tile_richness = richness_arr[i];

            let material = materials.add(tile_type.into_color().into());

            let mesh: Handle<Mesh> = meshes.add(build_tile_mesh(offset_angle));

            let (prb_bundle, land_tile) = LandTile::build(
                cub_coord,
                material,
                mesh,
                tile_type,
                tile_richness,
                offset_angle
            );

            land_tile_arr.push(land_tile.clone());

            // let circ = Circle::new(0.2);
            // for vertex in component.vertices.iter() {
            //     commands.spawn(PbrBundle {
            //         mesh: meshes.add(circ.into()),
            //         material: materials.add(Color::WHITE.into()),
            //         transform: Transform::from_translation(Vec3 {
            //             x: vertex.x,
            //             y: vertex.y,
            //             z: 0.15,
            //         }),
            //         ..default()
            //     });
            // }

            // let quad = Quad::new(Vec2 { x: 0.15, y: 0.6 });
            // let mut angle = PI / 3.;
            // for vertex in component.edges.iter() {
            //     commands.spawn(PbrBundle {
            //         mesh: meshes.add(quad.into()),
            //         material: materials.add(Color::WHITE.into()),
            //         transform: Transform::from_translation(Vec3 {
            //             x: vertex.x,
            //             y: vertex.y,
            //             z: 0.15,
            //         })
            //         .with_rotation(Quat::from_euler(
            //             EulerRot::XYZ,
            //             0.0,
            //             0.0,
            //             angle,
            //         )),
            //         ..default()
            //     });

            //     angle += PI / 3.;
            // }

            commands.spawn((
                prb_bundle,
                land_tile.clone(),
                PickableBundle::default(),
                On::<Pointer<Over>>::target_component_mut::<Transform>(|_, transform| {
                    let mut old_translation = transform.translation;
                    old_translation.z = 0.2;
                    transform.translation = old_translation;
                }),
                On::<Pointer<Out>>::target_component_mut::<Transform>(|_, transform| {
                    let mut old_translation = transform.translation;
                    old_translation.z = 0.0;
                    transform.translation = old_translation;
                }),
                RaycastPickTarget::default(),
            ));

            i += 1;
        } else {
            // Water tile
            let material = materials.add(Color::BLUE.into());
            let mesh: Handle<Mesh> = meshes.add(build_tile_mesh(offset_angle));
            let ent = WaterTile::build(cub_coord, material, mesh, TileType::WATER, 0);
            commands.spawn(ent);
        }
    }

    let mut all_tile_vertices: Vec<Vec3> = land_tile_arr
        .iter()
        .flat_map(|tile| tile.vertices.iter().cloned())
        .collect();

    // this sort by does not work as intended
    all_tile_vertices.sort_by(|v1, v2| sort_positions(v1, v2, eps));
    let unique_city_positions = filter_city_positions(&all_tile_vertices, eps);

    print_pos_vec(&unique_city_positions);

    let circle = Circle::new(0.2);

    for vertex in unique_city_positions.iter() {
        commands.spawn(PbrBundle {
            mesh: meshes.add(circle.into()),
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_translation(Vec3 {
                x: vertex.x,
                y: vertex.y,
                z: 0.15,
            }),
            ..default()
        });
    }
}

pub fn build_cub_coord_hex_gird(radius: i32) -> Vec<CubicCoord> {
    let mut hex_arr = vec![];
    let slice: Range<i32> = -radius..radius + 1;

    for q in slice.clone() {
        for r in slice.clone() {
            let s: i32 = 0 - q - r;

            if s.abs() > radius {
                continue;
            }

            hex_arr.push(CubicCoord::from_tuple((q, r, s)));
        }
    }

    return hex_arr;
}

// TODO test this with 1 hex, 7 hex and 19 hex
fn filter_city_positions(all_filtered_vertices: &Vec<Vec3>, eps: f32) -> Vec<Vec3> {
    let mut city_points_iter = all_filtered_vertices.iter();
    let mut prev_el = city_points_iter.next().unwrap();

    let mut unique_city_positions = Vec::new();

    for pt in city_points_iter {
        let ord = sort_positions(prev_el, pt, eps);

        if ord != Ordering::Equal {
            unique_city_positions.push(pt.clone());
        }

        prev_el = pt;
    }
    unique_city_positions
}

fn print_pos_vec(arr: &Vec<Vec3>) {
    for pt in arr.iter() {
        println!("{:.2},{:.2}", pt.x, pt.y);
    }
}

#[cfg(test)]
mod spawn_tiles_tests {
    use super::*;

    #[test]
    fn sort_Vec3() {
        let v = vec![Vec3::new(1.01, 1.01, 0.0), Vec3::new(1.01, -1.01, 0.0)];
    }
}
