use bevy::prelude::*;

use crate::{catan::cubic_coords::cube_coordinates::CubCoord, hex::polygon::{get_polygon_vert_with_center, build_polygon_mesh}};

use super::tile_type::TileType;

pub const TILE_RADIUS: f32 = 1.0;
pub const NUMBER_OF_TILES: usize = 1 + 6 + 12; // 19 default tiles

#[derive(Component)]
pub struct HexWorldTile {
    pub cub_coord: CubCoord,
    pub cart_coord: Vec3,
    pub tile_type: TileType,
    pub richness: u8,
    // vertices: [Vec3; 6],
    // adjacent_tiles: [Box<HexWorldTile>; 6],
    // edges / roads -> 6
    // towns -> 6
}

impl HexWorldTile {
    /// Builds a PrbBundle from a hex center and translates it.
    pub fn build(
        cub_coord: CubCoord,
        material: Handle<StandardMaterial>,
        mesh: Handle<Mesh>,
        tile_type: TileType,
        richness: u8,
    ) -> (PbrBundle, HexWorldTile) {
        let h = 3_f32.sqrt() / 2. * 1.01;
        let cart_coord = cub_coord.to_cartesian_vec3(h);

        return (
            PbrBundle {
                mesh,
                material,
                transform: Transform::from_translation(cart_coord),
                ..default()
            },
            HexWorldTile {
                cub_coord,
                cart_coord,
                tile_type,
                richness,
            },
        );
    }
}

// TODO refactor this to bevy hex mesh builder
pub fn build_tile_mesh(offset_angle: f32) -> Mesh {
    // center + 6 vertices
    let hex_tile_vertex_vec = get_polygon_vert_with_center(6, TILE_RADIUS, offset_angle);
    // vertices + edges + normals + uvs
    let hex_tile_mesh: Mesh = build_polygon_mesh(&hex_tile_vertex_vec);

    return hex_tile_mesh;
}