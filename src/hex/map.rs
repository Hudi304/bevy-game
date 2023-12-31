use std::f32::consts::PI;

use bevy::prelude::*;

use super::{polygon::{build_polygon_mesh, get_polygon_vert_with_center}, hex::_get_hex_vertices_with_center};

pub fn _render_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // color
    let red = Color::rgb_u8(255, 0, 0);
    let material = materials.add(red.into());

    // tile mesh
    let hex_tile_vtx_pos = get_polygon_vert_with_center(6, 1.0, 0.);
    let hex_tile_mesh = build_polygon_mesh(&hex_tile_vtx_pos);
    let mesh = meshes.add(hex_tile_mesh);

    // tile positions

    // center and first circle
    let mut tile_pos_vec1 = _get_hex_vertices_with_center(1.8, PI / 6.);

    // outer circle
    let outer_circle = _get_hex_vertices_with_center(3.1, 0.0);
    let outer_circle: Vec<[f32; 3]> = outer_circle.iter().skip(1).cloned().collect();

    tile_pos_vec1.extend(outer_circle);

    for pos in tile_pos_vec1.iter() {
        let transform = Transform::from_translation(Vec3::from_slice(pos));

        commands.spawn(PbrBundle {
            material: material.clone(),
            mesh: mesh.clone(),
            transform,
            ..Default::default()
        });
    }
}
