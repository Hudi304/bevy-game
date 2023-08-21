use std::{f32::consts::PI, ops::Range};

use bevy::prelude::*;
use rand::random;

use crate::{
    common::cube_coordinates::CubCoord,
    hex::polygon::{build_polygon_mesh, get_polygon_vert_with_center},
};

use super::tile_type::TileType;

pub const TILE_RADIUS: f32 = 1.0;
pub const NUMBER_OF_TILES: usize = 1 + 6 + 12; // 19 default tiles

#[derive(Component)]
pub struct HexWorldTile {
    cub_coord: CubCoord,
    cart_coord: Vec3,
    tile_type: TileType,
    // vertices: [Vec3; 6],
    // adjacent_tiles: [Box<HexWorldTile>; 6],
    // edges / roads -> 6
    // towns -> 6
    richness: u8,
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

pub fn build_hex_mesh(offset_angle: f32) -> Mesh {
    // center + 6 vertices
    let hex_tile_vertex_vec = get_polygon_vert_with_center(6, TILE_RADIUS, offset_angle);
    // vertices + edges + normals + uvs
    let hex_tile_mesh: Mesh = build_polygon_mesh(&hex_tile_vertex_vec);

    return hex_tile_mesh;
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

    // println!("{:?} | {:?}", tile_type, result);
    return result;
}

// TODO refactor this into a nice method sometime
/// Returns an array with the specified size of random TileTypes.\
/// There are rules of how many types of tiles there can be in the array.\
/// The rules are hard coded.
fn build_random_tile_type_array(size: usize) -> Vec<TileType> {
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

fn build_random_tile_richness_array(size: usize) -> Vec<u8> {
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

pub fn spawn_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // let h = 3_f32.sqrt() / 2. * 1.01;
    let cub_coords_arr: Vec<CubCoord> = build_cub_coord_hex_gird(7);

    let mut i = 0;
    let tile_type_arr = build_random_tile_type_array(NUMBER_OF_TILES);
    let richness_arr = build_random_tile_richness_array(NUMBER_OF_TILES);

    for cub_coord in cub_coords_arr {
        // TODO refactor this so the HexWorldTile contains the material asset and the mesh asset
        if cub_coord.ring < 3 {
            let tile_type = tile_type_arr[i];
            let tile_richness = richness_arr[i];
            let material = materials.add(tile_type.into_color().into());
            let mesh: Handle<Mesh> = meshes.add(build_hex_mesh(PI / 6.));
            let ent = HexWorldTile::build(cub_coord, material, mesh, tile_type, tile_richness);
            commands.spawn(ent);
            i += 1;
        } else {
            let material = materials.add(Color::BLUE.into());
            let mesh: Handle<Mesh> = meshes.add(build_hex_mesh(PI / 6.));
            let ent = HexWorldTile::build(cub_coord, material, mesh, TileType::WATER, 0);
            commands.spawn(ent);
        }
    }
}

pub fn spawn_tile_richness_text(mut commands: Commands, tile_query: Query<&HexWorldTile>) {
    let tile_iter = tile_query.iter();

    for tile_ent in tile_iter {
        let center = tile_ent.cart_coord;

        
    }

    return ();
}
