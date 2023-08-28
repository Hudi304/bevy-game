use bevy::prelude::*;

use super::land_tile::LandTile;

pub fn spawn_cities(tiles: Vec<LandTile>) {
    println!("spawn_cities");

    let size = tiles.iter().len();
    println!("spawn_cities {}", size);

    for tile in tiles.iter() {
        println!("{:?}", tile);
    }
}

pub fn spawn_roads(tiles: Vec<LandTile>) {



    
}
