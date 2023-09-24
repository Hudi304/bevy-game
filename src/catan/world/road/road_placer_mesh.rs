use bevy::prelude::Vec3;

use crate::catan::world::{
    land_tile::LandTile,
    vec3_utils::{ remove_vec3_duplicates, compare_vec3, remove_edges_duplicates },
    spawn_tiles::TILE_EPSILON,
};

pub fn get_road_positions(land_tiles: &Vec<LandTile>) -> Vec<(Vec3, f32)> {
    // All the hex vertices
    let mut all_tile_edge_centers: Vec<(Vec3, f32)> = land_tiles
        .iter()
        .flat_map(|tile| tile.edges.iter().cloned())
        .collect();

    all_tile_edge_centers.sort_by(|(v1, _), (v2, _)| compare_vec3(v1, v2, TILE_EPSILON));

    let unique_road_positions = remove_edges_duplicates(&all_tile_edge_centers, TILE_EPSILON);
    unique_road_positions
}

// #[cfg(test)]
// mod spawn_roads {
//     use bevy::prelude::Vec3;

//     use crate::{
//         catan::world::{
//             land_tile::{ TILE_RADIUS, BETWEEN_TILE_DISTANCE },
//             vec3_utils::{ remove_vec3_duplicates, sort_positions },
//             spawn_tiles::{ build_cub_coord_hex_gird, TILE_OFFSET_ANGLE_RAD, TILE_EPSILON },
//         },
//         hex::polygon::get_hex_vertices,
//     };

//     #[test]
//     fn build_roads_positions_1hex() {}
// }
