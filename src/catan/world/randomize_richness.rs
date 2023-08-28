use bevy::prelude::*;
use rand::random;

use super::land_tile::LandTile;

pub fn _spawn_tile_richness_text(mut _commands: Commands, tile_query: Query<&LandTile>) {
    let tile_iter = tile_query.iter();

    for tile_ent in tile_iter {
        let _center = tile_ent.cart_coord;
    }

    return ();
}

pub fn build_random_tile_richness_array(size: usize) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::with_capacity(size);
    // Tile type, max number of tiles, current number of tiles
    let mut type_counter_arr: Vec<(u8, u8, u8)> = vec![
        (2, 1, 0),  // *
        (12, 1, 0), // *
        (3, 2, 0),  // **
        (11, 2, 0), // **
        (4, 2, 0),  // ***
        (10, 2, 0), // ***
        (5, 2, 0),  // ****
        (9, 2, 0),  // ****
        (6, 2, 0),  // *****
        (8, 2, 0),  // *****
        (7, 1, 0),  // **********
    ];

    for _ in 0..size {
        let rand_pos = random::<f32>() * type_counter_arr.len() as f32;
        let rand_pos = rand_pos as usize;
        // Increment actual
        type_counter_arr[rand_pos].2 += 1;

        let (rand_richness, max, actual) = type_counter_arr[rand_pos];

        if actual == max {
            let mut filtered_ar = Vec::new();

            type_counter_arr.iter().for_each(|(richness, max, actual)| {
                if *richness != rand_richness {
                    filtered_ar.push((*richness, *max, *actual))
                }
            });

            type_counter_arr = filtered_ar;
        }

        result.push(rand_richness);
    }

    println!("{:?}", result);

    return result;
}
