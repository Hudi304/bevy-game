use std::{f32::consts::PI, ops::Range};

use bevy::prelude::*;
use bevy_mod_picking::prelude::{Listener, On, Over, Pointer};

use crate::catan::cubic_coords::cube_coordinates::CubCoord;

use super::{
    hex_tile::{build_tile_mesh, HexWorldTile, NUMBER_OF_TILES},
    randomize_richness::build_random_tile_richness_array,
    randomize_tiles::build_random_tile_type_array,
    tile_type::TileType,
};

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

            let mesh: Handle<Mesh> = meshes.add(build_tile_mesh(PI / 6.));

            let (prb_bundle, component) =
                HexWorldTile::build(cub_coord, material, mesh, tile_type, tile_richness);

            commands.spawn((
                prb_bundle,
                component,
                On::<Pointer<Over>>::run(|param: Listener<Pointer<Over>>| {
                    println!("{:?} ", param);
                }),
            ));

            i += 1;
        } else {
            let material = materials.add(Color::BLUE.into());

            let mesh: Handle<Mesh> = meshes.add(build_tile_mesh(PI / 6.));

            let ent = HexWorldTile::build(cub_coord, material, mesh, TileType::WATER, 0);

            commands.spawn(ent);
        }
    }
}

fn build_cub_coord_hex_gird(radius: i32) -> Vec<CubCoord> {
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
