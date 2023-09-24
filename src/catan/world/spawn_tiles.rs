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

const TILE_OFFSET_ANGLE_RAD: f32 = PI / 6.0; // 30deg

// I think you can also compute this with some kind of tree
// or trying not to overlap the city positions
// but I have no idea if that's worth it or faster

pub fn spawn_land_tiles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    // let h = 3_f32.sqrt() / 2. * 1.01;

    println!("spawn_land_tiles");

    let cub_coords_arr: Vec<CubicCoord> = build_cub_coord_hex_gird(7);
    let eps = 0.01;

    let mut i = 0;
    let tile_type_arr = build_random_tile_type_array(NUMBER_OF_TILES);
    let richness_arr = build_random_tile_richness_array(NUMBER_OF_TILES);

    let mut land_tile_arr = Vec::with_capacity(NUMBER_OF_TILES);

    for cub_coord in cub_coords_arr {
        // TODO refactor this so the HexWorldTile contains the material asset and the mesh asset
        if cub_coord.ring < 3 {
            // Land tile
            let tile_type = tile_type_arr[i];
            let tile_richness = richness_arr[i];

            let material = materials.add(tile_type.into_color().into());

            let mesh: Handle<Mesh> = meshes.add(build_tile_mesh(TILE_OFFSET_ANGLE_RAD));

            let (prb_bundle, land_tile) = LandTile::build(
                cub_coord,
                material,
                mesh,
                tile_type,
                tile_richness,
                TILE_OFFSET_ANGLE_RAD
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
            let mesh: Handle<Mesh> = meshes.add(build_tile_mesh(TILE_OFFSET_ANGLE_RAD));
            let ent = WaterTile::build(cub_coord, material, mesh, TileType::WATER, 0);
            commands.spawn(ent);
        }
    }

    // All the hex vertices
    let mut all_tile_vertices: Vec<Vec3> = land_tile_arr
        .iter()
        .flat_map(|tile| tile.vertices.iter().cloned())
        .collect();

    // sorted array of hex vertices, sorted by x,y
    all_tile_vertices.sort_by(|v1, v2| sort_positions(v1, v2, eps));
    // sorted array of hex vertices without duplicates within eps
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

    let mut unique_city_positions = Vec::new();

    let mut prev_city_pos = city_points_iter.next().unwrap();

    unique_city_positions.push(prev_city_pos.clone());

    for city_pos in city_points_iter {
        let ord = sort_positions(prev_city_pos, city_pos, eps);

        if ord != Ordering::Equal {
            unique_city_positions.push(city_pos.clone());
        }

        prev_city_pos = city_pos;
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
    use crate::{
        catan::{ world::land_tile::TILE_RADIUS, utils::vec },
        hex::polygon::get_hex_vertices,
    };

    use super::*;

    #[test]
    fn build_city_positions_1hex() {
        let eps = 0.01;
        let h = TILE_RADIUS * (3_f32).sqrt();

        let hex_cub_coord = build_cub_coord_hex_gird(0);
        assert_eq!(hex_cub_coord.len(), 1);

        let center_hex = hex_cub_coord.first().unwrap().to_owned();

        let center_cartesian_coord = center_hex.to_cartesian_vec3(h);
        let mut hex_vertices: Vec<Vec3> = get_hex_vertices(TILE_RADIUS, TILE_OFFSET_ANGLE_RAD)
            .iter()
            .map(|vert| vert.clone() + center_cartesian_coord)
            .collect();

        // sorted array of hex vertices, sorted by x,y
        hex_vertices.sort_by(|v1, v2| sort_positions(v1, v2, eps));

        // sorted array of hex vertices without duplicates within eps
        let unique_city_positions = filter_city_positions(&hex_vertices, eps);

        assert_eq!(unique_city_positions.len(), 6);
    }

    #[test]
    fn build_city_positions_7hex() {
        let eps = 0.01;
        let h = TILE_RADIUS * (3_f32).sqrt();

        let hex_cub_coord = build_cub_coord_hex_gird(1);

        assert_eq!(hex_cub_coord.len(), 7);

        let hex_vertex_cart = hex_cub_coord
            .iter()
            .map(|cub_coord| cub_coord.to_cartesian_vec3(h))
            .collect::<Vec<Vec3>>();

        let mut all_hex_vertices = vec![];

        for hex_vert in hex_vertex_cart {
            let mut hex_vertices: Vec<Vec3> = get_hex_vertices(TILE_RADIUS, TILE_OFFSET_ANGLE_RAD)
                .iter()
                .map(|vert| vert.clone() + hex_vert)
                .collect();

            all_hex_vertices.extend(hex_vertices);
        }

        // sorted array of hex vertices, sorted by x,y
        all_hex_vertices.sort_by(|v1, v2| sort_positions(v1, v2, eps));

        assert_eq!(all_hex_vertices.len(), 7 * 6);

        // sorted array of hex vertices without duplicates within eps
        let unique_city_positions = filter_city_positions(&all_hex_vertices, eps);

        // total number 6 (center) + 6 * 3  = 24
        // on the outer ring 2 vertices overlap the center
        // and one overlaps another outer hex
        assert_eq!(unique_city_positions.len(), 6 + 6 * 3);
    }

    #[test]
    fn build_city_positions_19hex() {
        let eps = 0.01;
        let h = TILE_RADIUS * (3_f32).sqrt();

        let hex_cub_coord = build_cub_coord_hex_gird(2);

        assert_eq!(hex_cub_coord.len(), 19);

        let hex_vertex_cart = hex_cub_coord
            .iter()
            .map(|cub_coord| cub_coord.to_cartesian_vec3(h))
            .collect::<Vec<Vec3>>();

        let mut all_hex_vertices = vec![];

        for hex_vert in hex_vertex_cart {
            let mut hex_vertices: Vec<Vec3> = get_hex_vertices(TILE_RADIUS, TILE_OFFSET_ANGLE_RAD)
                .iter()
                .map(|vert| vert.clone() + hex_vert)
                .collect();

            all_hex_vertices.extend(hex_vertices);
        }

        // sorted array of hex vertices, sorted by x,y
        all_hex_vertices.sort_by(|v1, v2| sort_positions(v1, v2, eps));

        assert_eq!(all_hex_vertices.len(), 19 * 6);

        // sorted array of hex vertices without duplicates within eps
        let unique_city_positions = filter_city_positions(&all_hex_vertices, eps);

        // on the first ring, all the hexes are special cases.
        // they are all on the vertex of the virtual (big) hex
        // on the second ring, there are 12 hexes
        // 6 of them are on the vertexes and 6 are in the middle of an edge
        // the vertex ones can contribute with 3 cities each, so the previous formula still stands
        // the hexes on the edges can only contribute with 2 new cities.

        // r(0) = 6
        // r(1) = r(0) + 6 * 3 = 24
        // r(2) = r(1) + 6 * 3 + (6*2 - 6) * 2
        // r(2) = 24  + 18 + 12 = 54
        // r(2) = r(1) + 6*(3 + 2(1)) = 24 + 6 * 5 = 54

        // maybe prove this with mathematical induction

        // r(x) = r(x-1) + 6*3 + 6*2*(r-1)
        // r(x) = r(x-1) + 6*(3 + 2*(r-1))

        assert_eq!(unique_city_positions.len(), 6 + 6 * 3 + 6 * 3 + 6 * 2);
    }
}
