use std::{f32::consts::PI, ops::Range};

use bevy::prelude::*;

use crate::{
    common::cube_coordinates::CubCoord,
    hex::polygon::{build_polygon_mesh, get_hex_vertices, get_polygon_vert_with_center},
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
    center: Vec3,
    // vertices: [Vec3; 6],
    // adjacent_tiles: [Box<HexWorldTile>; 6],
    // edges / roads -> 6
    // towns -> 6
    // richness -> u8
    pub r: f32,
}

impl HexWorldTile {
    /// Builds a PrbBundle from a hex center and translates it.
    pub fn build(
        center: Vec3,
        material: Handle<StandardMaterial>,
        mesh: Handle<Mesh>,
    ) -> (PbrBundle, HexWorldTile) {
        return (
            PbrBundle {
                mesh,
                material,
                transform: Transform::from_translation(center),
                ..default()
            },
            HexWorldTile {
                center: center,
                r: TILE_RADIUS,
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
    /// returns 6 elements
    pub fn get_adjacent_pos(&self, offset_angle_radians: f32) -> Vec<Vec3> {
        let tile_dist = 3_f32.sqrt() * TILE_RADIUS;
        let hex_tile_vertex_vec = get_hex_vertices(tile_dist, offset_angle_radians);
        return hex_tile_vertex_vec;
    }
}

pub fn hex(
    center: Vec3,
    material: &Handle<StandardMaterial>,
    mesh: &Handle<Mesh>,
) -> (PbrBundle, HexWorldTile) {
    return HexWorldTile::build(center, material.clone(), mesh.clone());
}

pub fn build_map_gird(coords: Vec<CubCoord>, h: f32) -> Vec<(Vec3, Color)> {
    coords
        .iter()
        .map(|coord| {
            if coord.ring > 3 {
                return (coord.to_cartesian_vec3(h), Color::BLUE);
            } else {
                return (coord.to_cartesian_vec3(h), Color::GREEN);
            }
        })
        .collect()
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
    let h = 3_f32.sqrt() / 2. * 1.01;

    // let x = |v: Vec3| (v.x + v.y / 2.) * 2. * h;
    // let y = |v: Vec3| (v.y * 3_f32.sqrt() / 2.) * 2. * h;
    // let xy = |v: Vec3| Vec3::new(1.01 * x(v), 1.01 * y(v), 0.0);
    // let mut hex_arr = vec![];

    let cub_coords_arr: Vec<CubCoord> = build_cub_coord_hex_gird(7);

    // hex_arr.iter().for_each(|el| println!("{:?}", el));

    println!();
    // for q in -3..4 {
    //     for r in -3..4 {
    //         let s: i32 = q + r;

    //         if s.abs() > 3 {
    //             continue;
    //         }

    //         let cub_coord = CubCoord::from_tuple((q, r, s));

    //         if cub_coord.ring == 3 {
    //             hex_arr.push((cub_coord.to_cartesian_vec3(h), Color::BLUE));
    //         } else {
    //             hex_arr.push((cub_coord.to_cartesian_vec3(h), Color::GREEN));
    //         }
    //     }
    // }

    // for q in -2..3 {
    //     for r in -2..3 {
    //         let s: i32 = q + r;
    //         if s.abs() > 2 {
    //             continue;
    //         }

    //         let cub_pos = Vec3::new(q as f32, r as f32, s as f32);
    //         hex_arr.push((xy(cub_pos), Color::GREEN));
    //     }
    // }

    // for q in -3..4 {
    //     for r in -3..4 {
    //         let s: i32 = q + r;
    //         let sum = q.abs() + r.abs() + s.abs();
    //         if s.abs() > 3 || sum < 6 {
    //             continue;
    //         }

    //         let cub_pos = Vec3::new(q as f32, r as f32, s as f32);
    //         hex_arr.push((xy(cub_pos), Color::BLUE));
    //     }
    // }

    let hex_arr = build_map_gird(cub_coords_arr, h);

    for (pos, color) in hex_arr {
        let material: Handle<StandardMaterial> = materials.add(color.into());
        let center_tile_mesh = HexWorldTile::build_hex_mesh(PI / 6.);
        let mesh = meshes.add(center_tile_mesh);
        let ent = hex(pos, &material, &mesh);

        commands.spawn(ent);
    }
}
