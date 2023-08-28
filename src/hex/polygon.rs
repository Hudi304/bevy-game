use std::f32::consts::PI;

use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

use crate::catan::utils::vec::vec3_to_vec;

/// Returns the vertices of a polygon as a Vec<Vec3>. \
/// The first element is the center.
pub fn get_polygon_vert_with_center(vert_no: usize, radius: f32, offset_angle: f32) -> Vec<Vec3> {
    let center: Vec3 = Vec3::ZERO;

    let mut vertex_vector = Vec::with_capacity(vert_no);

    vertex_vector.push(center);

    let vert_no_f32 = vert_no as f32;

    // the whole circle 2*PI divided by the number of vertices
    let vertex_to_ox_angle = 2. * PI / vert_no_f32;

    // for the unit circle it's enough but for other circles it's multiplied by the radius
    // the x component of that vertex is cos(r * angle)
    let x = |root: f32| radius * ((root * vertex_to_ox_angle + offset_angle).cos());

    // the y component of that vertex is sin(r * angle)
    let y = |root: f32| radius * ((root * vertex_to_ox_angle + offset_angle).sin());

    for i in 0..vert_no {
        // conversion
        let vert_x = x(i as f32);
        let vert_y = y(i as f32);

        let vert = Vec3::new(vert_x, vert_y, 0.);

        vertex_vector.push(vert);
    }

    return vertex_vector;
}

/// Returns the vertices of a polygon as a Vec<Vec3>. \
/// Without the center
pub fn get_hex_vertices(radius: f32, offset_angle: f32) -> Vec<Vec3> {
    let mut vertex_vector = Vec::with_capacity(6);
    let vert_no_f32 = 6.0;

    let vertex_to_ox_angle = 2. * PI / vert_no_f32;

    let x = |root: f32| radius * ((root * vertex_to_ox_angle + offset_angle).cos());
    let y = |root: f32| radius * ((root * vertex_to_ox_angle + offset_angle).sin());

    for i in 0..6 {
        // conversion
        let vert_x = x(i as f32);
        let vert_y = y(i as f32);

        let vert = Vec3::new(vert_x, vert_y, 0.);

        vertex_vector.push(vert);
    }

    return vertex_vector;
}

pub fn build_polygon_mesh(vertex_vector: &Vec<Vec3>) -> Mesh {
    let mut positions = Vec::with_capacity(6);
    let mut normals = Vec::with_capacity(6);
    let mut uvs = Vec::with_capacity(6);

    for vertex_position in vertex_vector.iter() {
        positions.push(vec3_to_vec(vertex_position));
        normals.push(vec3_to_vec(&Vec3::Z));
        uvs.push([0., 0.]);
    }

    // TODO this only works for hex make it generic
    let indices = Indices::U32(vec![0, 1, 2, 0, 2, 3, 0, 3, 4, 0, 4, 5, 0, 5, 6, 0, 6, 1]);

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(indices));

    return mesh;
}
