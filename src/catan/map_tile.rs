use std::{f32::consts::PI, ops::Range};

use bevy::prelude::*;
use rand::random;

use crate::{
    common::cube_coordinates::CubCoord,
    hex::polygon::{build_polygon_mesh, get_polygon_vert_with_center},
};

pub const TILE_RADIUS: f32 = 1.0;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum TileType {
    /// 4
    WHEAT,
    /// 3
    STONE,
    /// 4
    SHEEP,
    /// 3
    CLAY,
    /// 4
    WOOD,
    /// 1
    DESERT,
    /// ANY
    WATER,
}

impl TileType {
    fn into_color(&self) -> Color {
        match self {
            TileType::WHEAT => Color::YELLOW,
            TileType::CLAY => Color::ORANGE,
            TileType::STONE => Color::DARK_GRAY,
            TileType::SHEEP => Color::LIME_GREEN,
            TileType::WOOD => Color::DARK_GREEN,
            TileType::DESERT => Color::YELLOW_GREEN,
            TileType::WATER => Color::BLUE,
        }
    }
}

#[derive(Component)]
pub struct HexWorldTile {
    cub_coord: CubCoord,
    cart_coord: Vec3,
    tile_type: TileType,
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
        tile_type: TileType,
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

fn filter_type_out_with_ints(
    type_vec: &Vec<(TileType, i32, i32)>,
    f: TileType,
) -> Vec<(TileType, i32, i32)> {
    let mut result = Vec::new();

    type_vec.iter().for_each(|(el, max, actual)| {
        if *el != f {
            result.push((el.clone(), *max, *actual))
        }
    });

    println!("{:?} | {:?}", f, result);
    return result;
}

// TODO refactor this into a nice method sometime
fn build_tile_type_arr() -> Vec<TileType> {
    let no_of_tiles = 1 + 6 + 12;
    let mut tile_type_arr = Vec::<TileType>::with_capacity(no_of_tiles);

    // Tile type, max number of tiles, current number of tiles
    let mut type_arr: Vec<(TileType, i32, i32)> = vec![
        (TileType::CLAY, 3, 0),
        (TileType::DESERT, 1, 0),
        (TileType::SHEEP, 4, 0),
        (TileType::STONE, 3, 0),
        (TileType::WHEAT, 4, 0),
        (TileType::WOOD, 4, 0),
    ];

    for _ in 0..no_of_tiles {
        let rand_pos = random::<f32>() * type_arr.len() as f32;
        let rand_pos = rand_pos as usize;
        // Increment actual
        type_arr[rand_pos].2 += 1;

        let (rand_type, max, actual) = type_arr[rand_pos];

        // if actual got to max, we generated as many tiles as we need
        // remove the type from the type array, so it can not be generated again
        if actual == max {
            let arr = filter_type_out_with_ints(&type_arr, rand_type);
            type_arr = arr;
        }

        tile_type_arr.push(rand_type.clone());
    }

    return tile_type_arr;
}

pub fn test_tile(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // let h = 3_f32.sqrt() / 2. * 1.01;
    let cub_coords_arr: Vec<CubCoord> = build_cub_coord_hex_gird(7);

    let mut i = 0;
    let tile_type_arr = build_tile_type_arr();

    for cub_coord in cub_coords_arr {
        if cub_coord.ring < 3 {
            let tile_type = tile_type_arr[i];

            let material = materials.add(tile_type.into_color().into());
            let mesh: Handle<Mesh> = meshes.add(HexWorldTile::build_hex_mesh(PI / 6.));
            let ent = HexWorldTile::build(cub_coord, material, mesh, tile_type);
            commands.spawn(ent);
            i += 1;
        } else {
            let material = materials.add(Color::BLUE.into());
            let mesh: Handle<Mesh> = meshes.add(HexWorldTile::build_hex_mesh(PI / 6.));
            let ent = HexWorldTile::build(cub_coord, material, mesh, TileType::WATER);
            commands.spawn(ent);
        }
    }
}
