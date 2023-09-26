use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{
    catan::cubic_coords::cube_coordinates::CubicCoord,
    hex::polygon::{ build_polygon_mesh, get_hex_vertices, get_polygon_vert_with_center },
};

use super::tile_type::TileType;

pub const SQRT_3: f32 = 1.7320508075688772f32; // This is an approximation of sqrt(3) as f32
pub const TILE_RADIUS: f32 = 1.0;
// pub const TILE_HEIGHT: f32 = (TILE_RADIUS * SQRT_3) / 2.0;
pub const BETWEEN_TILE_DISTANCE: f32 = TILE_RADIUS * SQRT_3;

pub const NUMBER_OF_TILES: usize = 1 + 6 + 12; // 19 default tiles

#[derive(Component, Debug, Clone)]
pub struct LandTile {
    pub cub_coord: CubicCoord,
    pub cart_coord: Vec3,
    pub tile_type: TileType,
    pub richness: u8,
    pub vertices: Vec<Vec3>,
    // position and slope in radians
    pub edges: Vec<(Vec3, f32)>,
    // adjacent_tiles: [Box<HexWorldTile>; 6],
    // edges / roads -> 6
    // towns -> 6
    // pub adjacent_cities: [Option<Arc<City>>; 6],
}

impl LandTile {
    /// Builds a PrbBundle from a hex center and translates it.
    pub fn build(
        cub_coord: CubicCoord,
        tile_type: TileType,
        richness: u8,
        offset_angle: f32
    ) -> LandTile {
        let center_cartesian_coord = cub_coord.to_cartesian_vec3(BETWEEN_TILE_DISTANCE);

        let vertices: Vec<Vec3> = get_hex_vertices(TILE_RADIUS, offset_angle)
            .iter()
            .map(|vert| vert.clone() + center_cartesian_coord)
            .collect();

        let mut edges = Vec::with_capacity(6);

        for i in 0..6 {
            if i == 5 {
                let edge_center = (vertices[0] + vertices[5]) / 2.0;
                let angle = calculate_slope(vertices[0], vertices[5]);
                edges.push((edge_center, angle));
                continue;
            }

            let edge_center = (vertices[i] + vertices[i + 1]) / 2.0;
            let angle = calculate_slope(vertices[i], vertices[i + 1]);

            edges.push((edge_center, angle));
        }

        return LandTile {
            cub_coord,
            cart_coord: center_cartesian_coord,
            tile_type,
            richness,
            vertices,
            edges,
        };
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

/// Calculates the slope (angle in radians) between two 3D vectors `v1` and `v2`.
///
/// # Arguments
///
/// * `v1` - The starting point vector.
/// * `v2` - The ending point vector.
///
/// # Returns
///
/// * The slope (angle in radians) between the two vectors. If the line is vertical
///   (x2 - x1 is zero), the slope and angle are undefined, and `PI / 2.0` is returned.
///
/// # Examples
///
/// ```
/// let v1 = Vec3 { x: 1.0, y: 2.0, z: 0.0 };
/// let v2 = Vec3 { x: 3.0, y: 5.0, z: 0.0 };
/// let slope = calculate_slope(v1, v2);
/// println!("Slope between v1 and v2: {}", slope);
/// ```
fn calculate_slope(v1: Vec3, v2: Vec3) -> f32 {
    // Check if the line is vertical (x2 - x1 is zero)
    if (v2.x - v1.x).abs() < 1e-6 {
        PI / 2.0 // The line is vertical, and the slope and angle are undefined
    } else {
        let slope = (v2.y - v1.y) / (v2.x - v1.x);
        let angle_radians = slope.atan2(1.0);
        angle_radians
    }
}
