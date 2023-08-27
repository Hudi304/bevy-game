use crate::catan::utils::vec::vec3_to_vec;

use super::polygon::get_polygon_vert_with_center;

pub struct Hex {
    center: [f32; 3],
}

pub fn get_hex_vertices_with_center(r: f32, offset: f32) -> Vec<[f32; 3]> {
    //
    let vertex_vector = get_polygon_vert_with_center(6, r, offset);

    let result = vertex_vector
        .iter()
        .map(|vec3| (vec3_to_vec(vec3)))
        .collect();

    return result;
}
