use super::poligon::{get_polygon_vert, vec3_to_vec};

pub struct Hex {
    center: [f32; 3],
}

pub fn get_hex_vertices(r: f32, offset: f32) -> Vec<[f32; 3]> {
    let vertex_vector = get_polygon_vert(6, r, offset);

    let result = vertex_vector
        .iter()
        .map(|vec3| (vec3_to_vec(vec3)))
        .collect();

    return result;
}
