use bevy::prelude::Vec3;

pub fn _vec_to_vec3([x, y, z]: [f32; 3]) -> Vec3 {
    return Vec3 { x, y, z };
}

pub fn vec3_to_vec(vec: &Vec3) -> [f32; 3] {
    return [vec[0], vec[1], vec[2]];
}
