use std::f32::consts::PI;

use bevy::prelude::*;

use crate::catan::{cubic_coords::cube_coordinates::CubCoord, world::city::City};

use super::land_tile::{build_tile_mesh, LandTile};

// pub const TILE_RADIUS: f32 = 1.0;
// pub const NUMBER_OF_TILES: usize = 1 + 6;

// // Maybe a graph, bu it's not necessarily the best ides
// pub fn spawn_map_2(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     // let h = 3_f32.sqrt() / 2. * 1.01;
//     // let cub_coords_arr: Vec<CubCoord> = build_cub_coord_hex_gird(1);

//     let h = TILE_RADIUS * 3_f32.sqrt();

//     let center_cub_coord = CubCoord::from_tuple((0, 0, 0));
//     let cart_coord = center_cub_coord.to_cartesian_vec3(h);

//     let adjacent_cities_center = [
//         Some(City::build(center_cub_coord)),
//         Some(City::build(center_cub_coord)),
//         Some(City::build(center_cub_coord)),
//         Some(City::build(center_cub_coord)),
//         Some(City::build(center_cub_coord)),
//         Some(City::build(center_cub_coord)),
//     ];

//     let center_tile = LandTile {
//         cub_coord: center_cub_coord,
//         cart_coord: center_cub_coord.to_cartesian_vec3(h),
//         tile_type: super::tile_type::TileType::CLAY,
//         richness: 1,
//         // adjacent_cities: adjacent_cities_center.clone(),
//     };

//     commands.spawn((
//         PbrBundle {
//             mesh: meshes.add(build_tile_mesh(PI / 6.)),
//             material: materials.add(Color::PURPLE.into()),
//             transform: Transform::from_translation(cart_coord),
//             ..default()
//         },
//         center_tile, // LandTile
//     ));

//     let center_neighbors = center_cub_coord.get_neighbors();

//     for neighbor in center_neighbors {
//         let cart_coord = neighbor.to_cartesian_vec3(h);

//         commands.spawn((
//             PbrBundle {
//                 mesh: meshes.add(build_tile_mesh(PI / 6.)),
//                 material: materials.add(Color::GREEN.into()),
//                 transform: Transform::from_translation(cart_coord),
//                 ..default()
//             },
//             LandTile {
//                 cub_coord: neighbor,
//                 cart_coord: neighbor.to_cartesian_vec3(h),
//                 tile_type: super::tile_type::TileType::CLAY,
//                 richness: 1,
//                 // adjacent_cities: [None, None, None, None, None, None],
//             },
//         ));
//     }
// }
