use bevy::prelude::Vec3;

use crate::catan::world::{
    land_tile::LandTile,
    vec3_utils::{ compare_vec3, remove_edges_duplicates },
    spawn_tiles::TILE_EPSILON,
};

// TODO find a way to test this
// maybe split the spawning part from the building part of the tile
pub fn get_road_positions(land_tiles: &Vec<LandTile>) -> Vec<(Vec3, f32)> {
    // All the hex vertices
    let mut all_tile_edge_centers: Vec<(Vec3, f32)> = land_tiles
        .iter()
        .flat_map(|tile| tile.edges.iter().cloned())
        .collect();

    all_tile_edge_centers.sort_by(|(v1, _), (v2, _)| compare_vec3(v1, v2, TILE_EPSILON));

    let unique_road_positions = remove_edges_duplicates(&all_tile_edge_centers, TILE_EPSILON);

    assert_eq!(unique_road_positions.len(), 72);

    //  let r(x) be a function that returns the number of possible roads
    // for a given map radius x

    // r(0) = 6
    // r(1) = r(0) + 6 * 4 = 30
    // r(2) = r(1) + 6 * 4 + 6 * 3  = 30 + 24 + 18 = 72

    // r(x) = r(x-1) + 6*4 +  6 * (r-1) * 3
    // r(x) = r(x-1) + 6(4 + 3*r - 3)
    // r(x) = r(x-1) + 6(3*r + 1)

    unique_road_positions
}

// TODO write tests for this
