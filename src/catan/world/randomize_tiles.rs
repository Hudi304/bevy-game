use rand::random;

use super::tile_type::TileType;

/// Returns an array with the specified size of random TileTypes.\
/// There are rules of how many types of tiles there can be in the array.\
/// The rules are hard coded.
pub fn build_random_tile_type_array(size: usize) -> Vec<TileType> {
    let mut result: Vec<TileType> = Vec::with_capacity(size);

    // Tile type, max number of tiles, current number of tiles
    let mut type_counter_arr: Vec<(TileType, i32, i32)> = vec![
        (TileType::CLAY, 3, 0),
        (TileType::DESERT, 1, 0),
        (TileType::SHEEP, 4, 0),
        (TileType::STONE, 3, 0),
        (TileType::WHEAT, 4, 0),
        (TileType::WOOD, 4, 0),
    ];

    for _ in 0..size {
        let rand_pos = random::<f32>() * type_counter_arr.len() as f32;
        let rand_pos = rand_pos as usize;
        // Increment actual
        type_counter_arr[rand_pos].2 += 1;

        let (rand_type, max, actual) = type_counter_arr[rand_pos];

        // if actual got to max, we generated as many tiles as we need
        // remove the type from the type array, so it can not be generated again
        if actual == max {
            let arr = filter_type_out_with_ints(&type_counter_arr, rand_type);
            type_counter_arr = arr;
        }

        result.push(rand_type.clone());
    }

    return result;
}

/// Takes in a type_counter_array (Vec<(TileType, i32, i32)>), and a TileType \
/// Returns a new vector that does not contain the provided type
fn filter_type_out_with_ints(
    type_counter_arr: &Vec<(TileType, i32, i32)>,
    tile_type: TileType,
) -> Vec<(TileType, i32, i32)> {
    // TODO empty guard
    let mut result = Vec::new();

    type_counter_arr.iter().for_each(|(el, max, actual)| {
        if *el != tile_type {
            result.push((el.clone(), *max, *actual))
        }
    });

    return result;
}

