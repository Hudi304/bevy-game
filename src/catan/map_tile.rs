use std::{f32::consts::PI, ops::Range};

use bevy::prelude::*;

use crate::{
    common::cube_coordinates::CubCoord,
    hex::polygon::{build_polygon_mesh, get_polygon_vert_with_center},
};

pub const TILE_RADIUS: f32 = 1.0;

pub enum TileType {
    WHEAT,
    STONE,
    SHEEP,
    CLAY,
    WOOD,
    DESERT,
}

#[derive(Component)]
pub struct HexWorldTile {
    cub_coord: CubCoord,
    cart_coord: Vec3,
    // vertices: [Vec3; 6],
    // adjacent_tiles: [Box<HexWorldTile>; 6],
    // edges / roads -> 6
    // towns -> 6
    // richness -> u8
}

impl HexWorldTile {
    /// Builds a PrbBundle from a hex center and translates it.
    pub fn build(
        cub_coord: CubCoord,
        material: Handle<StandardMaterial>,
        mesh: Handle<Mesh>,
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
            },
        );
    }

    pub fn build_hex_mesh(offset_angle: f32) -> Mesh {
        // center + 6 vertices
        let hex_tile_vertex_vec = get_polygon_vert_with_center(6, TILE_RADIUS, offset_angle);
        // vertices + edges + normals + uvs
        let hex_tile_mesh: Mesh = build_polygon_mesh(&hex_tile_vertex_vec);

        return hex_tile_mesh;
    }
}

pub fn build_cub_coord_hex_gird(radius: i32) -> Vec<CubCoord> {
    let mut hex_arr = vec![];
    let slice: Range<i32> = -radius..radius + 1;

    for q in slice.clone() {
        for r in slice.clone() {
            let s: i32 = 0 - q - r;

            if s.abs() > radius {
                continue;
            }

            hex_arr.push(CubCoord::from_tuple((q, r, s)));
        }
    }

    return hex_arr;
}

pub fn test_tile(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // let h = 3_f32.sqrt() / 2. * 1.01;
    let cub_coords_arr: Vec<CubCoord> = build_cub_coord_hex_gird(7);

    for cub_coord in cub_coords_arr {
        let mut material: Handle<StandardMaterial> = materials.add(Color::BLUE.into());

        if cub_coord.ring < 4 {
            material = materials.add(Color::GREEN.into());
        }

        let mesh: Handle<Mesh> = meshes.add(HexWorldTile::build_hex_mesh(PI / 6.));
        let ent = HexWorldTile::build(cub_coord, material, mesh);
        commands.spawn(ent);
    }
}
