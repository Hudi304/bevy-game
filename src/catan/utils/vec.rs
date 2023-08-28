use bevy::prelude::Vec3;

pub fn _vec_to_vec3([x, y, z]: [f32; 3]) -> Vec3 {
    return Vec3 { x, y, z };
}

pub fn vec3_to_vec(vec: &Vec3) -> [f32; 3] {
    return [vec[0], vec[1], vec[2]];
}

pub fn i32_tup_to_f32_tup((a, b, c): (i32, i32, i32)) -> (f32, f32, f32) {
    return (a as f32, b as f32, c as f32);
}
