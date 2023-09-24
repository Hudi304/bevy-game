use std::sync::Arc;

use bevy::prelude::*;

use crate::catan::cubic_coords::cube_coordinates::CubicCoord;

#[derive(Component, Default)]
pub struct City {
    pub cart_coord: Vec3,
    pub is_placed: bool,
    pub parent_cub_coords: CubicCoord, // pub adjacent_tiles: [Option<Arc<LandTile>>; 3],
}

impl City {
    pub fn build(parent_cub_coords: CubicCoord) -> Arc<City> {
        return Arc::new(City {
            parent_cub_coords,
            ..default()
        });
    }
}
