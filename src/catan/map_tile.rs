use std::f32::consts::PI;

use bevy::prelude::*;

use crate::hex::{
    hex::get_hex_vertices_with_center,
    polygon::{build_polygon_mesh, get_hex_vertices, get_polygon_vert_with_center},
};

pub const TILE_RADIUS: f32 = 1.0;

#[derive(Component)]
pub struct HexWorldTile {
    center: Vec3,
    // vertices: [Vec3; 6],
    // adjacent_tiles: [Box<HexWorldTile>; 6],
    // edges / roads -> 6
    // towns -> 6
    pub r: f32,
}

// pub fn spawn_first_hex_ring() {}

impl HexWorldTile {
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
        // let position_offset = TILE_RADIUS / 10.0;
        // a little space between the hexes
        // let adjacent_hex_circle_radius = TILE_RADIUS * 2.0; //+ position_offset;

        let tile_dist = 3_f32.sqrt() * TILE_RADIUS;

        let hex_tile_vertex_vec = get_hex_vertices(tile_dist, offset_angle_radians);
        return hex_tile_vertex_vec;
    }
}

pub fn spawn_center_tile(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let center = Vec3::ZERO;
    let material: Handle<StandardMaterial> = materials.add(Color::GREEN.into());

    let center_tile_mesh = HexWorldTile::build_hex_mesh(0.0);
    let mesh = meshes.add(center_tile_mesh);
    let center_tile = HexWorldTile::build(center, material, mesh);
    commands.spawn(center_tile);
}

pub fn spawn_fist_tile_row(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let tile_dist = 3_f32.sqrt() * TILE_RADIUS;
    let hex_tile_vertex_vec = get_hex_vertices(tile_dist, PI / 6.);

    let mut red: u8 = 0;

    let adjacent_hexes: Vec<(PbrBundle, HexWorldTile)> = hex_tile_vertex_vec
        .iter()
        .map(|center_position| {
            let color = Color::rgb_u8(red, 10, 10);
            let material_handle = materials.add(color.into());
            red += 40;
            let mesh_handle = meshes.add(HexWorldTile::build_hex_mesh(0.0));
            return HexWorldTile::build(center_position.clone(), material_handle, mesh_handle);
        })
        .collect();

    for tile in adjacent_hexes {
        commands.spawn(tile);
    }
}

pub fn spawn_second_tile_row(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let hex_tile_vertex_vec = get_hex_vertices(TILE_RADIUS * 3., 0.0);
    let mut green: u8 = 255;
    let adjacent_hexes: Vec<(PbrBundle, HexWorldTile)> = hex_tile_vertex_vec
        .iter()
        .map(|center_position| {
            let color = Color::rgb_u8(10, green, 10);
            let material_handle = materials.add(color.into());
            green -= 30;
            let mesh_handle = meshes.add(HexWorldTile::build_hex_mesh(0.0));
            return HexWorldTile::build(center_position.clone(), material_handle, mesh_handle);
        })
        .collect();

    for tile in adjacent_hexes {
        commands.spawn(tile);
    }

    let hex_h = 3_f32.sqrt() * TILE_RADIUS / 2.;

    let radius = 4. * hex_h ;
    let hex_tile_vertex_vec = get_hex_vertices(radius, PI / 6.);
    let mut green: u8 = 255;
    let adjacent_hexes: Vec<(PbrBundle, HexWorldTile)> = hex_tile_vertex_vec
        .iter()
        .map(|center_position| {
            let color = Color::rgb_u8(10, green, 10);
            let material_handle = materials.add(color.into());
            green -= 30;
            let mesh_handle = meshes.add(HexWorldTile::build_hex_mesh(0.0));
            return HexWorldTile::build(center_position.clone(), material_handle, mesh_handle);
        })
        .collect();

    for tile in adjacent_hexes {
        commands.spawn(tile);
    }
}
