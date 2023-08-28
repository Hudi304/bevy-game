use bevy::prelude::*;

use crate::catan::cubic_coords::cube_coordinates::CubCoord;

use super::{land_tile::TILE_RADIUS, tile_type::TileType};
#[derive(Component)]
pub struct WaterTile {
    pub cub_coord: CubCoord,
    pub cart_coord: Vec3,
    pub tile_type: TileType,
    pub richness: u8,
}

impl WaterTile {
    /// Builds a PrbBundle from a hex center and translates it.
    pub fn build(
        cub_coord: CubCoord,
        material: Handle<StandardMaterial>,
        mesh: Handle<Mesh>,
        tile_type: TileType,
        richness: u8,
    ) -> (PbrBundle, WaterTile) {
        // let h = 3_f32.sqrt() * 1.01;
        let h = TILE_RADIUS * 3_f32.sqrt();

        let cart_coord = cub_coord.to_cartesian_vec3(h);

        return (
            PbrBundle {
                mesh,
                material,
                transform: Transform::from_translation(cart_coord),
                ..default()
            },
            WaterTile {
                cub_coord,
                cart_coord,
                tile_type,
                richness,
            },
        );
    }
}
